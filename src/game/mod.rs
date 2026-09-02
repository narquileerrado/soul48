//! El estado del juego y la orquestación del turno.
//!
//! Antes todo esto era un solo `app.rs` de casi 2.000 líneas donde convivían
//! los tipos, el combate, la IA, el inventario, la visión y el guardado. Cada
//! submódulo agrega sus métodos al mismo `App` con su propio bloque `impl`.

pub mod ai;
pub mod combat;
pub mod fov;
pub mod interaction;
pub mod inventory;
pub mod map;
pub mod pathing;
pub mod save;

pub use save::SaveData;

use crate::balance;
use crate::player::Player;
use crate::settings::Settings;
use crate::world::map_builder;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Representa el estado actual de la aplicación/juego.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum GameState {
    TitleScreen,
    Playing,
    GameOver,
    /// El Archidemonio cayó: la corrida terminó y terminó bien.
    Victory,
    Bestiary,
    Options,
}

/// Define el comportamiento actual de una entidad enemiga.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum EnemyState {
    Asleep,
    Wandering,
    Aggressive,
}

/// Especifica el tipo de inteligencia artificial que rige a un enemigo.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum EnemyAI {
    Melee,
    Wandering,
    Coward,
    Stationary,
}

/// Clasificación de las entidades presentes en el mundo del juego.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum EntityType {
    Mob {
        hp: i32,
        max_hp: i32,
        state: EnemyState,
        ai: EnemyAI,
        min_dmg: i32,
        max_dmg: i32,
        defense: i32,
        pacified: bool,
    },
    Item,
    Weapon {
        min_dmg: i32,
        max_dmg: i32,
    },
    Armor {
        defense: i32,
    },
    Helmet {
        defense: i32,
    },
    Ring {
        stat_bonus: i32,
    },
    Amulet {
        sanity_bonus: i32,
    },
    Chest {
        locked: bool,
    },
    Key,
    TalkingWall {
        message: String,
        whispered: bool,
    },
    EchoAltar {
        used: bool,
    },
    Scroll {
        scroll_type: ScrollType,
    },
    Door {
        locked: bool,
        secret: bool,
        open: bool,
    },
    Hazard {
        hazard_type: HazardType,
    },
    SpecialRoomMarker {
        room_type: SpecialRoomType,
    },
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum HazardType {
    Spikes,
    Acid,
    Oil,
    Fire,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SpecialRoomType {
    Armory,
    Library,
    RitualCircle,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ScrollType {
    Lightning,
    Fireball,
    Teleport,
    Invisibility,
}

/// Categorías para los mensajes de registro (log).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum LogType {
    Info,
    Combat,
    Item,
    Warning,
    /// Lo que dicen las paredes y los ecos: tiene color propio.
    Whisper,
}

/// Estructura de un mensaje para el historial de eventos.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMessage {
    pub text: String,
    pub l_type: LogType,
}

/// Representación de una coordenada bidimensional en el mapa.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Estructura base para cualquier objeto o criatura interactuable.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StatusEffectType {
    Poison,
    Bleed,
    Freeze,
    Burn,
    Confusion,
    Blindness,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: usize,
    pub damage_per_turn: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub pos: Point,
    pub glyph: char,
    pub color: Color,
    pub name: String,
    pub e_type: EntityType,
    #[serde(default)]
    pub status_effects: Vec<StatusEffect>,
}

/// Algo a la vista del héroe, tal como lo lista el panel lateral.
pub struct Avistamiento<'a> {
    pub glifo: char,
    pub color: Color,
    pub nombre: &'a str,
    /// Distancia en pasos, medida en diagonal.
    pub distancia: usize,
    /// Proporción de vida, sólo para criaturas.
    pub vida: Option<f64>,
}

/// Estructura principal que mantiene el estado global de la simulación.
pub struct App {
    /// El héroe entero: números, equipo y efectos.
    pub player: Player,
    pub logs: VecDeque<LogMessage>,
    pub map: Vec<Vec<char>>,
    pub visible: Vec<Vec<bool>>,
    pub explored: Vec<Vec<bool>>,
    pub fov_radius: isize,
    pub entities: Vec<Entity>,
    pub inventory: Vec<(Entity, usize)>,
    pub seed: u64,
    pub depth: u32,

    // Flags de mecánicas de flujo
    /// El aceite del último paso te dejó resbalando.
    pub resbalon_pendiente: bool,
    pub drop_mode: bool,
    pub show_descend_prompt: bool,

    // Estado de navegación y UI
    pub state: GameState,
    pub settings: Settings,
    rng: ChaCha8Rng,
}

impl App {
    /// Inicializa una nueva instancia de la aplicación, generando el nivel y las entidades.
    pub fn new(
        custom_seed: Option<u64>,
        hp: Option<i32>,
        inventory: Option<Vec<(Entity, usize)>>,
        depth: u32,
        weapon: Option<(String, i32, i32)>,
    ) -> App {
        let seed = custom_seed.unwrap_or_else(|| rand::thread_rng().gen());
        let rng = ChaCha8Rng::seed_from_u64(seed);

        let map_builder = map_builder::MapBuilder::new(seed, depth);
        let map = map_builder.map;
        let hero_pos = map_builder.hero_start;
        let entities = map_builder.entities;
        let map_width = map[0].len();
        let map_height = map.len();

        let initial_state = if depth == 1 && hp.is_none() {
            GameState::TitleScreen
        } else {
            GameState::Playing
        };

        let mut player = Player::nuevo(hero_pos);
        if let Some(vida) = hp {
            player.hp = vida;
        }
        player.equipment.weapon = weapon;

        let mut app = App {
            player,
            logs: VecDeque::from([LogMessage {
                text: format!("> NIVEL {} - SEED: {}", depth, seed),
                l_type: LogType::Info,
            }]),
            map,
            visible: vec![vec![false; map_width]; map_height],
            explored: vec![vec![false; map_width]; map_height],
            fov_radius: balance::percepcion::RADIO_FOV,
            entities,
            inventory: inventory.unwrap_or_default(),
            seed,
            depth,

            resbalon_pendiente: false,
            drop_mode: false,
            show_descend_prompt: false,

            state: initial_state,
            settings: Settings::load(crate::settings::RUTA_AJUSTES),
            rng,
        };

        if app.state == GameState::Playing {
            app.smooth_walls();
            app.calculate_fov();
        }

        app
    }

    /// Baja un piso: mundo nuevo, héroe intacto.
    ///
    /// Antes esto era una llamada a `App::new` desde `main.rs`, que reconstruía
    /// al héroe entero: nivel, experiencia, atributos y las cuatro ranuras de
    /// equipo volvían a cero en cada escalera. Lo único que cambia al bajar es
    /// el mundo.
    pub fn descend(&mut self) {
        let profundidad = self.depth + 1;
        let mut abajo = App::new(None, None, None, profundidad, None);

        // lo único que se renueva es el mundo: el héroe cruza entero
        std::mem::swap(&mut abajo.settings, &mut self.settings);
        std::mem::swap(&mut abajo.inventory, &mut self.inventory);
        let entrada = abajo.player.pos;
        std::mem::swap(&mut abajo.player, &mut self.player);

        // la escalera te deja donde el piso nuevo te ponga, no donde estabas
        abajo.player.pos = entrada;
        abajo.player.hp = abajo.player.hp.min(abajo.player.max_hp);
        abajo.player.parry_active = false;
        abajo.player.damage_flash_turns = 0;
        abajo.player.ajustar_cordura();

        abajo.state = GameState::Playing;
        // `App::new` ya alisó los muros; sólo falta mirar desde la entrada
        abajo.calculate_fov();
        abajo.add_log(
            format!("> HAS DESCENDIDO AL NIVEL {}", profundidad),
            LogType::Info,
        );

        // al cruzar a un tramo nuevo, el descenso se presenta
        let antes = crate::world::tramo::de_piso(profundidad - 1);
        let ahora = crate::world::tramo::de_piso(profundidad);
        if !std::ptr::eq(antes, ahora) {
            abajo.add_log(format!("> {}", ahora.nombre), LogType::Whisper);
            abajo.add_log(ahora.entrada.to_string(), LogType::Whisper);
        }
        *self = abajo;
    }

    /// Sala vacía de 21x21 con muros en el borde y el héroe en el centro.
    ///
    /// Los tests de mecánicas no deberían depender de lo que haya generado
    /// `MapBuilder` para una semilla: acá el escenario es explícito y cada
    /// test coloca sólo las entidades que le interesan.
    pub fn arena(seed: u64) -> App {
        const LADO: usize = 21;
        let mut app = App::new(Some(seed), None, None, 1, None);

        let mut map = vec![vec!['.'; LADO]; LADO];
        for i in 0..LADO {
            map[0][i] = '#';
            map[LADO - 1][i] = '#';
            map[i][0] = '#';
            map[i][LADO - 1] = '#';
        }

        app.map = map;
        app.visible = vec![vec![false; LADO]; LADO];
        app.explored = vec![vec![false; LADO]; LADO];
        app.player.pos = Point::new(LADO / 2, LADO / 2);
        app.entities.clear();
        app.logs.clear();
        app.state = GameState::Playing;
        app.calculate_fov();
        app
    }

    /// Transiciona el juego al estado activo e inicializa la visión.
    pub fn start_new_game(&mut self) {
        self.smooth_walls();
        self.calculate_fov();
        self.state = GameState::Playing;
        self.add_log(
            "> Un viaje traicionero comienza... PISO 1".into(),
            LogType::Info,
        );
    }

    /// Suma un delta a una coordenada devolviendo None si el resultado sería negativo.
    fn offset_point(pos: Point, dx: isize, dy: isize) -> Option<Point> {
        let nx = pos.x as isize + dx;
        let ny = pos.y as isize + dy;
        if nx < 0 || ny < 0 {
            return None;
        }
        Some(Point::new(nx as usize, ny as usize))
    }

    /// Intenta desplazar al héroe a una nueva posición, gestionando colisiones e interacciones.
    pub fn try_move(&mut self, dx: isize, dy: isize) -> bool {
        // Con la cordura baja la penumbra empieza a torcerte los pasos. Antes
        // el medidor bajaba durante toda la partida sin consecuencia alguna.
        let (dx, dy) = self.desviar_por_cordura(dx, dy);
        let new_pos = match Self::offset_point(self.player.pos, dx, dy) {
            Some(p) => p,
            None => return false,
        };

        let tile = match self.tile(new_pos) {
            Some(t) if self.es_transitable(new_pos) => t,
            _ => return false,
        };

        if tile == '>' {
            self.show_descend_prompt = true;
            return true;
        }

        let mut action_taken = false;
        let mut move_allowed = true;

        if let Some(index) = self.entities.iter().position(|e| e.pos == new_pos) {
            let (allowed, action) = self.interact_with_entity(index);
            move_allowed = allowed;
            action_taken = action;
        }

        if move_allowed {
            self.player.pos = new_pos;
            action_taken = true;

            // el aceite te lleva un paso más en la misma dirección, si hay
            // dónde: es lo que el historial venía prometiendo sin cumplir
            if std::mem::take(&mut self.resbalon_pendiente) {
                if let Some(mas_alla) = Self::offset_point(new_pos, dx, dy) {
                    let libre = self.es_transitable(mas_alla)
                        && !self.entities.iter().any(|e| e.pos == mas_alla);
                    if libre {
                        self.player.pos = mas_alla;
                    }
                }
            }
        }
        self.resbalon_pendiente = false;

        self.reap_dead();
        action_taken
    }

    /// Añade experiencia al personaje y gestiona subidas de nivel.
    pub fn add_xp(&mut self, amount: u32) {
        self.add_log(format!("> Ganas {} de experiencia.", amount), LogType::Info);
        for aviso in self.player.ganar_xp(amount) {
            self.add_log(aviso, LogType::Info);
        }
    }

    /// Cierra el prompt de descenso.
    pub fn confirm_descent(&mut self) {
        self.show_descend_prompt = false;
    }

    /// Tuerce la dirección de un paso cuando la cordura está por el piso.
    ///
    /// La probabilidad crece a medida que la cordura cae por debajo del
    /// umbral; con la cordura entera, devuelve el paso tal cual.
    fn desviar_por_cordura(&mut self, dx: isize, dy: isize) -> (isize, isize) {
        let umbral = balance::cordura::UMBRAL_ALUCINACION;
        // la confusión hace lo mismo que la cordura baja, y se suma a ella
        let confundido = self.player.tiene(&StatusEffectType::Confusion);
        if self.player.sanity >= umbral && !confundido {
            return (dx, dy);
        }
        let por_cordura = (umbral - self.player.sanity.max(0)).max(0) as f64 / umbral as f64;
        let caida = if confundido {
            por_cordura.max(0.6)
        } else {
            por_cordura
        };
        if !self.rng.gen_bool((caida * 0.5).clamp(0.0, 1.0)) {
            return (dx, dy);
        }

        let desvios = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let elegido = desvios[self.rng.gen_range(0..desvios.len())];
        if elegido == (dx, dy) {
            return (dx, dy);
        }
        let motivo = if confundido {
            "> La confusión te tuerce el paso."
        } else {
            "> La penumbra te tuerce el paso."
        };
        self.add_log(motivo.into(), LogType::Whisper);
        elegido
    }

    /// Añade un mensaje al historial.
    ///
    /// El tope es de datos y fijo: cuántas líneas se ven es una decisión de la
    /// interfaz (`settings.lineas_susurro`), y antes ese ajuste de pantalla
    /// borraba el historial de verdad y se llevaba puesto lo que iba al save.
    pub fn add_log(&mut self, text: String, l_type: LogType) {
        self.logs.push_back(LogMessage { text, l_type });
        // `Vec::remove(0)` corría los 200 mensajes de lugar en cada línea
        while self.logs.len() > balance::percepcion::TOPE_HISTORIAL {
            self.logs.pop_front();
        }
    }

    /// Muestra información detallada sobre un tile específico inspeccionado por el usuario.
    pub fn inspect_tile(&mut self, tx: u16, ty: u16) {
        // el mapa se dibuja bajo la cinta superior: una columna y dos filas de desfase
        if tx == 0 || ty < 2 {
            return;
        }
        let mouse_pos = Point::new(tx as usize - 1, ty as usize - 2);
        if self.tile(mouse_pos).is_none() {
            return;
        }

        if self.visible[mouse_pos.y][mouse_pos.x] {
            if let Some(e) = self.entities.iter().find(|e| e.pos == mouse_pos) {
                self.add_log(format!("> INFO: {}", e.name), LogType::Info);
            } else if self.tile(mouse_pos) == Some('>') {
                self.add_log("> INFO: Escaleras hacia abajo.".into(), LogType::Info);
            } else {
                self.add_log("> INFO: Terreno despejado.".into(), LogType::Info);
            }
        }
    }

    /// Índice de entidades por casilla.
    ///
    /// Dibujar el mapa consultaba `entities` entera por *cada* celda: con
    /// 60x25 casillas eran miles de comparaciones por cuadro. Con el índice es
    /// una tabla que se arma una vez y se consulta en tiempo constante.
    ///
    /// Cuando dos entidades comparten casilla gana la primera, igual que en
    /// `try_move`; `MapBuilder` ya se encarga de que eso no pase al generar.
    pub fn indice_entidades(&self) -> HashMap<Point, usize> {
        let mut indice = HashMap::with_capacity(self.entities.len());
        for (i, e) in self.entities.iter().enumerate() {
            indice.entry(e.pos).or_insert(i);
        }
        indice
    }

    /// Lo que el héroe tiene a la vista, de lo más cerca a lo más lejos.
    pub fn entidades_cercanas(&self, tope: usize) -> Vec<Avistamiento<'_>> {
        let mut cerca: Vec<Avistamiento> = self
            .entities
            .iter()
            .filter(|e| self.visible[e.pos.y][e.pos.x])
            .map(|e| {
                let dx = (e.pos.x as isize - self.player.pos.x as isize).abs();
                let dy = (e.pos.y as isize - self.player.pos.y as isize).abs();
                Avistamiento {
                    glifo: e.glyph,
                    color: e.color,
                    nombre: e.name.as_str(),
                    distancia: dx.max(dy) as usize,
                    // `max_hp` estaba declarado y guardado, y no lo miraba nadie
                    vida: match e.e_type {
                        EntityType::Mob { hp, max_hp, .. } if max_hp > 0 => {
                            Some(hp.max(0) as f64 / max_hp as f64)
                        }
                        _ => None,
                    },
                }
            })
            .collect();
        cerca.sort_by(|a, b| a.distancia.cmp(&b.distancia).then(a.nombre.cmp(b.nombre)));
        cerca.truncate(tope);
        cerca
    }

    /// Lo que se puede levantar del suelo y guardar en el inventario.
    pub fn es_recogible(e_type: &EntityType) -> bool {
        matches!(
            e_type,
            EntityType::Item
                | EntityType::Key
                | EntityType::Scroll { .. }
                | EntityType::Weapon { .. }
                | EntityType::Armor { .. }
                | EntityType::Helmet { .. }
                | EntityType::Ring { .. }
                | EntityType::Amulet { .. }
        )
    }

    /// Lo que no se mueve, y por lo tanto se puede recordar en el mapa.
    pub fn es_estatico(e_type: &EntityType) -> bool {
        matches!(
            e_type,
            EntityType::TalkingWall { .. }
                | EntityType::EchoAltar { .. }
                | EntityType::Chest { .. }
                | EntityType::Door { .. }
                | EntityType::Hazard { .. }
                | EntityType::SpecialRoomMarker { .. }
        )
    }
}
