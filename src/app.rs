use crate::map_builder;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ratatui::{style::Color, widgets::ListState};
use serde::{Deserialize, Serialize};

/// Representa el estado actual de la aplicación/juego.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum GameState {
    TitleScreen,
    Playing,
    GameOver,
    Bestiary,
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
}

/// Estructura de un mensaje para el historial de eventos.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMessage {
    pub text: String,
    pub l_type: LogType,
}

/// Representación de una coordenada bidimensional en el mapa.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
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

/// Estructura serializable para la persistencia del juego en disco.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub hero_pos: Point,
    pub hero_hp: i32,
    pub hero_max_hp: i32,
    pub sanity: i32,
    pub max_sanity: i32,
    pub level: u32,
    pub xp: u32,
    pub next_level_xp: u32,
    pub strength: i32,
    pub agility: i32,
    pub willpower: i32,
    pub hero_status_effects: Vec<StatusEffect>,
    pub parry_active: bool,
    pub invisible_turns: usize,
    pub equipped_weapon: Option<(String, i32, i32)>,
    pub equipped_armor: Option<(String, i32)>,
    pub equipped_helmet: Option<(String, i32)>,
    pub equipped_ring: Option<(String, i32)>,
    pub equipped_amulet: Option<(String, i32)>,
    pub logs: Vec<LogMessage>,
    pub map: Vec<Vec<char>>,
    pub visible: Vec<Vec<bool>>,
    pub explored: Vec<Vec<bool>>,
    pub entities: Vec<Entity>,
    pub inventory: Vec<(Entity, usize)>,
    pub seed: u64,
    pub depth: u32,
}

/// Estructura principal que mantiene el estado global de la simulación.
pub struct App {
    pub hero_pos: Point,
    pub hero_hp: i32,
    pub hero_max_hp: i32,
    pub sanity: i32,
    pub max_sanity: i32,
    pub level: u32,
    pub xp: u32,
    pub next_level_xp: u32,
    pub strength: i32,
    pub agility: i32,
    pub willpower: i32,
    pub hero_status_effects: Vec<StatusEffect>,
    pub parry_active: bool,
    pub invisible_turns: usize,
    pub equipped_weapon: Option<(String, i32, i32)>,
    pub equipped_armor: Option<(String, i32)>,
    pub equipped_helmet: Option<(String, i32)>,
    pub equipped_ring: Option<(String, i32)>,
    pub equipped_amulet: Option<(String, i32)>,
    pub logs: Vec<LogMessage>,
    pub map: Vec<Vec<char>>,
    pub visible: Vec<Vec<bool>>,
    pub explored: Vec<Vec<bool>>,
    pub fov_radius: isize,
    pub entities: Vec<Entity>,
    pub inventory: Vec<(Entity, usize)>,
    pub seed: u64,
    pub depth: u32,

    // Flags de mecánicas de flujo
    pub drop_mode: bool,
    pub show_descend_prompt: bool,

    // Estado de navegación y UI
    pub state: GameState,
    pub title_menu_state: ListState,
    pub bestiary_state: ListState,
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

        let mut title_menu_state = ListState::default();
        title_menu_state.select(Some(0));

        let mut bestiary_state = ListState::default();
        bestiary_state.select(Some(0));

        let initial_state = if depth == 1 && hp.is_none() {
            GameState::TitleScreen
        } else {
            GameState::Playing
        };

        let mut app = App {
            hero_pos,
            hero_hp: hp.unwrap_or(20),
            hero_max_hp: 20,
            sanity: 100,
            max_sanity: 100,
            level: 1,
            xp: 0,
            next_level_xp: 50,
            strength: 5,
            agility: 5,
            willpower: 5,
            hero_status_effects: Vec::new(),
            parry_active: false,
            invisible_turns: 0,
            equipped_weapon: weapon,
            equipped_armor: None,
            equipped_helmet: None,
            equipped_ring: None,
            equipped_amulet: None,
            logs: vec![LogMessage {
                text: format!("> NIVEL {} - SEED: {}", depth, seed),
                l_type: LogType::Info,
            }],
            map,
            visible: vec![vec![false; map_width]; map_height],
            explored: vec![vec![false; map_width]; map_height],
            fov_radius: 6,
            entities,
            inventory: inventory.unwrap_or_default(),
            seed,
            depth,

            drop_mode: false,
            show_descend_prompt: false,

            state: initial_state,
            title_menu_state,
            bestiary_state,
            rng,
        };

        if app.state == GameState::Playing {
            app.smooth_walls();
            app.calculate_fov();
        }

        app
    }

    /// Gestiona la interacción física o lógica con una entidad en el mapa.
    fn interact_with_entity(&mut self, index: usize) -> (bool, bool) {
        let mut entity_clone = self.entities[index].clone();
        let mut move_allowed = true;
        let mut entity_index_to_remove = None;

        match &mut entity_clone.e_type {
            EntityType::Mob {
                hp, state, defense, pacified, ..
            } => {
                if *pacified {
                    self.add_log(format!("> {} ignora tu presencia en paz.", entity_clone.name), LogType::Info);
                    move_allowed = false;
                } else if entity_clone.name.contains("Ladrón") && *state != EnemyState::Aggressive && self.sanity >= 20 {
                    // Oportunidad de negociación pacífica con el espíritu / espíritu errante
                    *pacified = true;
                    self.sanity -= 10;
                    self.add_log(
                        format!("> NEGOCIACIÓN: Calmas al {} entregando un fragmento de tu voz (-10 Cordura).", entity_clone.name),
                        LogType::Info,
                    );
                    self.entities[index] = entity_clone;
                    move_allowed = false;
                } else {
                    let (mut min_d, mut max_d) = self
                        .equipped_weapon
                        .as_ref()
                        .map(|w| (w.1, w.2))
                        .unwrap_or((1, 3));
                    if min_d > max_d {
                        std::mem::swap(&mut min_d, &mut max_d);
                    }

                    let mut damage = self.rng.gen_range(min_d..=max_d);
                    damage = (damage - *defense).max(1);

                    if self.rng.gen_bool(0.2) {
                        damage *= 2;
                        self.add_log(format!("> CRÍTICO: ¡{} daño!", damage), LogType::Combat);
                    } else {
                        self.add_log(
                            format!("> {} daño a {}.", damage, entity_clone.name),
                            LogType::Combat,
                        );
                    }
                    *hp -= damage;
                    *state = EnemyState::Aggressive;

                    if *hp <= 0 {
                        self.add_log(
                            format!("> {} eliminada.", entity_clone.name),
                            LogType::Combat,
                        );
                    let gained_xp = match entity_clone.name.as_str() {
                        "Murciélago" => 10,
                        "Serpiente" => 15,
                        "Ladrón" => 20,
                        "Gnoll" => 30,
                        "Cofre Sospechoso" => 45,
                        _ => 15,
                    };
                    self.add_xp(gained_xp);
                        entity_index_to_remove = Some(index);
                    }
                    self.entities[index] = entity_clone;
                    move_allowed = false;
                }
            }
            EntityType::Chest { locked } => {
                move_allowed = false;
                if *locked {
                    if let Some(k_idx) = self
                        .inventory
                        .iter()
                        .position(|(i, _)| i.name == "Llave de Hierro")
                    {
                        if self.inventory[k_idx].1 > 1 {
                            self.inventory[k_idx].1 -= 1;
                        } else {
                            self.inventory.remove(k_idx);
                        }

                        self.add_log("> Abres el cofre con la llave.".into(), LogType::Info);
                        let dmg_bonus = self.depth as i32;

                        self.entities[index] = Entity {
                            pos: entity_clone.pos,
                            glyph: '/',
                            color: Color::Cyan,
                            name: format!("Espada +{}", dmg_bonus),
                            e_type: EntityType::Weapon {
                                min_dmg: 3 + dmg_bonus,
                                max_dmg: 8 + dmg_bonus,
                            },
                            status_effects: Vec::new(),
                        };
                    } else {
                        self.add_log(
                            "> El cofre está cerrado (Necesitas Llave).".into(),
                            LogType::Warning,
                        );
                    }
                }
            }
            EntityType::TalkingWall { message, whispered } => {
                move_allowed = false;
                if !*whispered {
                    self.add_log(format!("> SUSURRO DE LA PARED: \"{}\"", message), LogType::Info);
                    *whispered = true;
                    self.entities[index] = entity_clone;
                } else {
                    self.add_log("> La pared guarda silencio ahora.".into(), LogType::Info);
                }
            }
            EntityType::EchoAltar { used } => {
                move_allowed = false;
                if !*used {
                    if self.hero_hp > 5 {
                        self.hero_hp -= 5;
                        *used = true;
                        self.entities[index] = entity_clone;
                        self.add_log("> PACTO DE SANGRE: Ofreces 5 HP al Altar de Ecos.".into(), LogType::Warning);
                        self.add_log("> EL PISO SE REVELA ANTE TI.".into(), LogType::Info);

                        // Revela todo el mapa explorado
                        let map_height = self.map.len();
                        let map_width = self.map[0].len();
                        for y in 0..map_height {
                            for x in 0..map_width {
                                self.explored[y][x] = true;
                            }
                        }
                    } else {
                        self.add_log("> Tu alma está demasiado débil para ofrecer sangre.".into(), LogType::Warning);
                    }
                } else {
                    self.add_log("> El Altar de Ecos ha consumido su tributo.".into(), LogType::Info);
                }
            }
            EntityType::Door { locked, secret, open } => {
                if *open {
                    move_allowed = true;
                } else if *locked {
                    move_allowed = false;
                    if let Some(k_idx) = self.inventory.iter().position(|(i, _)| i.name == "Llave de Hierro") {
                        if self.inventory[k_idx].1 > 1 {
                            self.inventory[k_idx].1 -= 1;
                        } else {
                            self.inventory.remove(k_idx);
                        }
                        *locked = false;
                        *open = true;
                        entity_clone.glyph = '\'';
                        entity_clone.name = "Puerta Abierta".into();
                        self.add_log("> Desbloqueas y abres la puerta con la llave.".into(), LogType::Info);
                        self.entities[index] = entity_clone;
                    } else {
                        self.add_log("> La puerta está cerrada con llave.".into(), LogType::Warning);
                    }
                } else {
                    move_allowed = false;
                    let is_sec = *secret;
                    *open = true;
                    entity_clone.glyph = '\'';
                    entity_clone.name = if is_sec { "Pasaje Secreto Revelado".into() } else { "Puerta Abierta".into() };
                    self.add_log(if is_sec { "> ¡Descubres un pasaje secreto!".into() } else { "> Abres la puerta.".into() }, LogType::Info);
                    self.entities[index] = entity_clone;
                }
            }
            EntityType::Hazard { hazard_type } => {
                move_allowed = true;
                match hazard_type {
                    HazardType::Spikes => {
                        self.hero_hp = (self.hero_hp - 4).max(0);
                        self.add_log("> TRAMPA DE PINCHOS: ¡Sufres 4 de daño!".into(), LogType::Warning);
                    }
                    HazardType::Acid => {
                        self.hero_hp = (self.hero_hp - 6).max(0);
                        self.hero_status_effects.push(StatusEffect {
                            effect_type: StatusEffectType::Poison,
                            duration: 3,
                            damage_per_turn: 2,
                        });
                        self.add_log("> POZO DE ÁCIDO: ¡Sufres 6 daño y te envenenas!".into(), LogType::Warning);
                    }
                    HazardType::Oil => {
                        self.add_log("> CHARCO DE ACEITE: El suelo resbaladizo dificulta tus pasos.".into(), LogType::Info);
                    }
                    HazardType::Fire => {
                        self.hero_hp = (self.hero_hp - 8).max(0);
                        self.hero_status_effects.push(StatusEffect {
                            effect_type: StatusEffectType::Burn,
                            duration: 2,
                            damage_per_turn: 3,
                        });
                        self.add_log("> FUEGO: ¡Sufres 8 daño y te quemas!".into(), LogType::Warning);
                    }
                }
            }
            EntityType::SpecialRoomMarker { room_type } => {
                move_allowed = true;
                match room_type {
                    SpecialRoomType::Armory => {
                        self.add_log("> ENTRANDO A LA ARMERÍA: El olor a metal templado llena el aire.".into(), LogType::Info);
                    }
                    SpecialRoomType::Library => {
                        self.add_log("> ENTRANDO A LA BIBLIOTECA: Pergaminos arcanos descansan en los estantes.".into(), LogType::Info);
                    }
                    SpecialRoomType::RitualCircle => {
                        self.add_log("> CIRCULO RITUAL: Sientes un escalofrío de energía oscura.".into(), LogType::Warning);
                    }
                }
            }
            EntityType::Item | EntityType::Key | EntityType::Weapon { .. } | EntityType::Scroll { .. } | EntityType::Armor { .. } | EntityType::Helmet { .. } | EntityType::Ring { .. } | EntityType::Amulet { .. } => {
                let is_stackable =
                    matches!(entity_clone.e_type, EntityType::Item | EntityType::Key | EntityType::Scroll { .. });

                let stack_index = if is_stackable {
                    self.inventory
                        .iter()
                        .position(|(i, _)| i.name == entity_clone.name)
                } else {
                    None
                };

                if let Some(idx) = stack_index {
                    self.inventory[idx].1 += 1;
                    let new_count = self.inventory[idx].1;
                    self.add_log(
                        format!("> Recoges {} (x{}).", entity_clone.name, new_count),
                        LogType::Item,
                    );
                    entity_index_to_remove = Some(index);
                } else if self.inventory.len() < 9 {
                    self.add_log(format!("> Recoges {}.", entity_clone.name), LogType::Item);
                    self.inventory.push((entity_clone, 1));
                    entity_index_to_remove = Some(index);
                } else {
                    self.add_log("> Inventario lleno.".into(), LogType::Warning);
                    move_allowed = false;
                }
            }
        }

        if let Some(i) = entity_index_to_remove {
            self.entities.remove(i);
        }

        (move_allowed, true)
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
        let new_pos = match Self::offset_point(self.hero_pos, dx, dy) {
            Some(p) => p,
            None => return false,
        };

        if new_pos.y >= self.map.len() || new_pos.x >= self.map[0].len() {
            return false;
        }

        let tile = self.map[new_pos.y][new_pos.x];
        if tile != '.' && tile != '>' {
            return false;
        }

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
            self.hero_pos = new_pos;
            action_taken = true;
        }

        action_taken
    }

    /// Ejecuta la habilidad activa de Embestida (Pushback) sobre un enemigo adyacente.
    pub fn use_pushback(&mut self) -> bool {
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in directions {
            if let Some(target_pos) = Self::offset_point(self.hero_pos, dx, dy) {
                if let Some(idx) = self.entities.iter().position(|e| e.pos == target_pos && matches!(e.e_type, EntityType::Mob { .. })) {
                    let push_pos = Self::offset_point(target_pos, dx, dy);
                    if let Some(p_pos) = push_pos {
                        if p_pos.y < self.map.len() && p_pos.x < self.map[0].len() && self.map[p_pos.y][p_pos.x] == '.' {
                            self.entities[idx].pos = p_pos;
                            self.add_log(format!("> EMBESTIDA: Empujas a {} hacia atrás.", self.entities[idx].name), LogType::Combat);
                            return true;
                        }
                    }
                    self.add_log(format!("> EMBESTIDA: Impactas a {} contra la pared.", self.entities[idx].name), LogType::Combat);
                    if let EntityType::Mob { ref mut hp, .. } = self.entities[idx].e_type {
                        *hp -= 5;
                    }
                    return true;
                }
            }
        }
        self.add_log("> No hay enemigos adyacentes para embestir.".into(), LogType::Warning);
        false
    }

    /// Activa la postura de Bloqueo / Parry para reducir el daño del próximo turno.
    pub fn use_parry(&mut self) -> bool {
        self.parry_active = true;
        self.add_log("> BLOQUEO ACTIVO: Te preparas para desviar el próximo impacto.".into(), LogType::Info);
        true
    }

    /// Añade experiencia al personaje y gestiona subidas de nivel.
    pub fn add_xp(&mut self, amount: u32) {
        self.xp += amount;
        self.add_log(format!("> Ganas {} de experiencia.", amount), LogType::Info);

        while self.xp >= self.next_level_xp {
            self.xp -= self.next_level_xp;
            self.level += 1;
            self.next_level_xp = (self.next_level_xp as f32 * 1.5) as u32;

            self.hero_max_hp += 5;
            self.hero_hp = self.hero_max_hp;
            self.max_sanity += 10;
            self.sanity = self.max_sanity;
            self.strength += 1;
            self.agility += 1;
            self.willpower += 1;

            self.add_log(
                format!("> ¡SUBIDA DE NIVEL! Alcanzas el Nivel {}. Atributos incrementados.", self.level),
                LogType::Info,
            );
        }
    }

    /// Cierra el prompt de descenso.
    pub fn confirm_descent(&mut self) {
        self.show_descend_prompt = false;
    }

    /// Aplica el efecto de un objeto del inventario o lo equipa.
    pub fn use_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        let item = self.inventory[index].0.clone();
        let mut item_used = false;

        match item.e_type {
            EntityType::Item => {
                if item.name == "Poción de Curación" {
                    self.hero_hp = (self.hero_hp + 15).min(self.hero_max_hp);
                    self.add_log("> Te sientes recuperado.".into(), LogType::Item);
                    item_used = true;
                }
            }
            EntityType::Scroll { ref scroll_type } => {
                match scroll_type {
                    ScrollType::Lightning => {
                        let mut hit_msgs = Vec::new();
                        for entity in &mut self.entities {
                            if matches!(entity.e_type, EntityType::Mob { .. }) {
                                let dist = (self.hero_pos.x as isize - entity.pos.x as isize).abs()
                                    + (self.hero_pos.y as isize - entity.pos.y as isize).abs();
                                if dist <= 5 {
                                    if let EntityType::Mob { ref mut hp, .. } = entity.e_type {
                                        *hp -= 12;
                                        hit_msgs.push(format!("> RAYO: ¡Impactas a {} con 12 daño de rayo!", entity.name));
                                    }
                                }
                            }
                        }
                        if hit_msgs.is_empty() {
                            self.add_log("> El pergamino de rayo chisporrotea sin blanco cercano.".into(), LogType::Warning);
                        } else {
                            for msg in hit_msgs {
                                self.add_log(msg, LogType::Combat);
                            }
                        }
                        item_used = true;
                    }
                    ScrollType::Fireball => {
                        let mut hit_msgs = Vec::new();
                        for entity in &mut self.entities {
                            if matches!(entity.e_type, EntityType::Mob { .. }) {
                                let dist = (self.hero_pos.x as isize - entity.pos.x as isize).abs()
                                    + (self.hero_pos.y as isize - entity.pos.y as isize).abs();
                                if dist <= 3 {
                                    if let EntityType::Mob { ref mut hp, .. } = entity.e_type {
                                        *hp -= 15;
                                        hit_msgs.push(format!("> ¡{} sufre 15 daño por fuego!", entity.name));
                                    }
                                }
                            }
                        }
                        self.add_log("> BOLA DE FUEGO: ¡Explosión de fuego cercana!".into(), LogType::Combat);
                        for msg in hit_msgs {
                            self.add_log(msg, LogType::Combat);
                        }
                        item_used = true;
                    }
                    ScrollType::Teleport => {
                        let mut rng = rand::thread_rng();
                        for _ in 0..100 {
                            let rx = rng.gen_range(1..self.map[0].len() - 1);
                            let ry = rng.gen_range(1..self.map.len() - 1);
                            if self.map[ry][rx] == '.' {
                                self.hero_pos = Point::new(rx, ry);
                                self.add_log("> TELETRANSPORTE: Te desvaneces y reapareces en otro lugar.".into(), LogType::Info);
                                self.calculate_fov();
                                break;
                            }
                        }
                        item_used = true;
                    }
                    ScrollType::Invisibility => {
                        self.invisible_turns = 8;
                        self.add_log("> INVISIBILIDAD: Tu cuerpo se vuelve transparente por 8 turnos.".into(), LogType::Info);
                        item_used = true;
                    }
                }
            }
            EntityType::Weapon { min_dmg, max_dmg } => {
                if let Some(old_w) = &self.equipped_weapon {
                    self.inventory.push((
                        Entity {
                            pos: Point::new(0, 0),
                            glyph: '/',
                            color: Color::Cyan,
                            name: old_w.0.clone(),
                            e_type: EntityType::Weapon {
                                min_dmg: old_w.1,
                                max_dmg: old_w.2,
                            },
                            status_effects: Vec::new(),
                        },
                        1,
                    ));
                }
                self.equipped_weapon = Some((item.name.clone(), min_dmg, max_dmg));
                self.add_log(format!("> Equipas {}.", item.name), LogType::Info);
                item_used = true;
            }
            EntityType::Armor { defense } => {
                if let Some(old_a) = &self.equipped_armor {
                    self.inventory.push((
                        Entity {
                            pos: Point::new(0, 0),
                            glyph: '[',
                            color: Color::Gray,
                            name: old_a.0.clone(),
                            e_type: EntityType::Armor { defense: old_a.1 },
                            status_effects: Vec::new(),
                        },
                        1,
                    ));
                }
                self.equipped_armor = Some((item.name.clone(), defense));
                self.add_log(format!("> Equipas armadura {}.", item.name), LogType::Info);
                item_used = true;
            }
            EntityType::Helmet { defense } => {
                if let Some(old_h) = &self.equipped_helmet {
                    self.inventory.push((
                        Entity {
                            pos: Point::new(0, 0),
                            glyph: '^',
                            color: Color::Yellow,
                            name: old_h.0.clone(),
                            e_type: EntityType::Helmet { defense: old_h.1 },
                            status_effects: Vec::new(),
                        },
                        1,
                    ));
                }
                self.equipped_helmet = Some((item.name.clone(), defense));
                self.add_log(format!("> Equipas casco {}.", item.name), LogType::Info);
                item_used = true;
            }
            EntityType::Ring { stat_bonus } => {
                if let Some(old_r) = &self.equipped_ring {
                    self.inventory.push((
                        Entity {
                            pos: Point::new(0, 0),
                            glyph: '=',
                            color: Color::Yellow,
                            name: old_r.0.clone(),
                            e_type: EntityType::Ring { stat_bonus: old_r.1 },
                            status_effects: Vec::new(),
                        },
                        1,
                    ));
                }
                self.equipped_ring = Some((item.name.clone(), stat_bonus));
                self.add_log(format!("> Equipas anillo {}.", item.name), LogType::Info);
                item_used = true;
            }
            EntityType::Amulet { sanity_bonus } => {
                if let Some(old_am) = &self.equipped_amulet {
                    self.inventory.push((
                        Entity {
                            pos: Point::new(0, 0),
                            glyph: '"',
                            color: Color::LightCyan,
                            name: old_am.0.clone(),
                            e_type: EntityType::Amulet { sanity_bonus: old_am.1 },
                            status_effects: Vec::new(),
                        },
                        1,
                    ));
                }
                self.equipped_amulet = Some((item.name.clone(), sanity_bonus));
                self.add_log(format!("> Equipas amuleto {}.", item.name), LogType::Info);
                item_used = true;
            }
            _ => {
                self.add_log(
                    format!("> No puedes usar {} así.", item.name),
                    LogType::Warning,
                );
            }
        }

        if item_used {
            if self.inventory[index].1 > 1 {
                self.inventory[index].1 -= 1;
            } else {
                self.inventory.remove(index);
            }
            return true;
        }
        false
    }

    /// Descarta un objeto del inventario en la posición actual del héroe.
    pub fn drop_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }

        if self.entities.iter().any(|e| {
            matches!(
                e.e_type,
                EntityType::Item | EntityType::Key | EntityType::Weapon { .. }
            ) && e.pos == self.hero_pos
        }) {
            self.add_log(
                "> Ya hay un objeto en el suelo aquí.".into(),
                LogType::Warning,
            );
            return false;
        }

        let mut item = self.inventory[index].0.clone();
        item.pos = self.hero_pos;

        if self.inventory[index].1 > 1 {
            self.inventory[index].1 -= 1;
            self.add_log(format!("> Sueltas un(a) {}.", item.name), LogType::Info);
        } else {
            self.inventory.remove(index);
            self.add_log(format!("> Sueltas {}.", item.name), LogType::Info);
        }

        self.entities.push(item);
        true
    }

    /// Añade un mensaje al historial, manteniendo un tamaño máximo.
    pub fn add_log(&mut self, text: String, l_type: LogType) {
        self.logs.push(LogMessage { text, l_type });
        if self.logs.len() > 5 {
            self.logs.remove(0);
        }
    }

    /// Procesa la lógica de turno para todas las entidades enemigas (IA).
    pub fn process_enemy_turns(&mut self) {
        let hx = self.hero_pos.x as isize;
        let hy = self.hero_pos.y as isize;
        let mut messages = Vec::new();

        // Procesar efectos de estado del héroe
        let mut i = 0;
        while i < self.hero_status_effects.len() {
            let dmg = self.hero_status_effects[i].damage_per_turn;
            if dmg > 0 {
                self.hero_hp = (self.hero_hp - dmg).max(0);
                self.add_log(
                    format!("> Sufres {} daño por efecto de estado.", dmg),
                    LogType::Warning,
                );
            }
            self.hero_status_effects[i].duration -= 1;
            if self.hero_status_effects[i].duration == 0 {
                self.hero_status_effects.remove(i);
            } else {
                i += 1;
            }
        }

        // Decrementar invisibilidad
        if self.invisible_turns > 0 {
            self.invisible_turns -= 1;
            if self.invisible_turns == 0 {
                self.add_log("> La invisibilidad se disipa.".into(), LogType::Info);
            }
        }

        // Desgaste de cordura por turno
        if self.rng.gen_bool(0.15) && self.sanity > 0 {
            self.sanity -= 1;
            if self.sanity == 0 {
                self.add_log("> TUS PENSAMIENTOS SE COLAPSAN EN EL SILENCIO.".into(), LogType::Warning);
            }
        }

        for i in 0..self.entities.len() {
            let (mut current_state, ai, ex, ey, name, pacified) = match &self.entities[i].e_type {
                EntityType::Mob { state, ai, pacified, .. } => (
                    state.clone(),
                    ai.clone(),
                    self.entities[i].pos.x as isize,
                    self.entities[i].pos.y as isize,
                    self.entities[i].name.clone(),
                    *pacified,
                ),
                _ => continue,
            };

            if pacified {
                continue;
            }

            let dist = (hx - ex).abs() + (hy - ey).abs();

            if current_state == EnemyState::Asleep && dist < 4 {
                current_state = EnemyState::Aggressive;
                messages.push((format!("> {} despierta!", name), LogType::Warning));
            }

            match current_state {
                EnemyState::Asleep => {}
                EnemyState::Wandering => {
                    if ai != EnemyAI::Stationary {
                        let dx = self.rng.gen_range(-1..=1);
                        let dy = self.rng.gen_range(-1..=1);
                        self.move_mob(i, dx, dy);
                    }
                    if dist < 6 && self.has_los((ex, ey), (hx, hy)) {
                        current_state = EnemyState::Aggressive;
                    }
                }
                EnemyState::Aggressive => {
                    if dist == 1 {
                        if self.invisible_turns > 0 {
                            messages.push((
                                format!("> {} no puede verte en las sombras.", name),
                                LogType::Info,
                            ));
                        } else if let EntityType::Mob {
                            min_dmg, max_dmg, ..
                        } = self.entities[i].e_type
                        {
                            let mut dmg = self.rng.gen_range(min_dmg..=max_dmg);
                            if self.parry_active {
                                dmg = (dmg / 2).max(1);
                                self.parry_active = false;
                                messages.push((
                                    format!("> ¡PARRY! Desvías el golpe de {} (recibes sólo {} daño)", name, dmg),
                                    LogType::Info,
                                ));
                            } else {
                                messages.push((
                                    format!("> {} te golpea ({} daño)", name, dmg),
                                    LogType::Warning,
                                ));
                            }
                            self.hero_hp = (self.hero_hp - dmg).max(0);
                        }
                    } else {
                        match ai {
                            EnemyAI::Melee | EnemyAI::Wandering => {
                                let mx = (hx - ex).signum();
                                let my = (hy - ey).signum();
                                self.move_mob(i, mx, my);
                            }
                            EnemyAI::Coward => {
                                let mx = (ex - hx).signum();
                                let my = (ey - hy).signum();
                                self.move_mob(i, mx, my);
                            }
                            EnemyAI::Stationary => {}
                        }
                    }

                    if dist > 10 {
                        current_state = EnemyState::Wandering;
                    }
                }
            }

            if let EntityType::Mob { ref mut state, .. } = self.entities[i].e_type {
                *state = current_state;
            }
        }
        for (m, t) in messages {
            self.add_log(m, t);
        }
    }

    /// Mueve a un mob validando colisiones con el mapa y otras entidades.
    fn move_mob(&mut self, idx: usize, dx: isize, dy: isize) {
        let new_pos = match Self::offset_point(self.entities[idx].pos, dx, dy) {
            Some(p) => p,
            None => return,
        };

        if new_pos.y < self.map.len()
            && new_pos.x < self.map[0].len()
            && self.map[new_pos.y][new_pos.x] == '.'
        {
            if new_pos != self.hero_pos {
                if !self
                    .entities
                    .iter()
                    .enumerate()
                    .any(|(i, e)| i != idx && e.pos == new_pos)
                {
                    self.entities[idx].pos = new_pos;
                }
            }
        }
    }

    /// Calcula el Campo de Visión (FOV) del héroe basado en un radio definido.
    pub fn calculate_fov(&mut self) {
        let hx = self.hero_pos.x as isize;
        let hy = self.hero_pos.y as isize;
        for row in &mut self.visible {
            for val in row {
                *val = false;
            }
        }
        for y in (hy - self.fov_radius)..=(hy + self.fov_radius) {
            for x in (hx - self.fov_radius)..=(hx + self.fov_radius) {
                if x >= 0 && x < self.map[0].len() as isize && y >= 0 && y < self.map.len() as isize
                {
                    if (x - hx).pow(2) + (y - hy).pow(2) <= self.fov_radius.pow(2)
                        && self.has_los((hx, hy), (x, y))
                    {
                        self.visible[y as usize][x as usize] = true;
                        self.explored[y as usize][x as usize] = true;
                    }
                }
            }
        }
    }

    /// Comprueba si existe línea de visión (LOS) entre dos puntos (Algoritmo de Bresenham).
    pub fn has_los(&self, p0: (isize, isize), p1: (isize, isize)) -> bool {
        let (mut x, mut y) = p0;
        let (x1, y1) = p1;
        let (dx, dy) = ((x1 - x).abs(), -(y1 - y).abs());
        let (sx, sy) = (
            if p0.0 < x1 { 1 } else { -1 },
            if p0.1 < y1 { 1 } else { -1 },
        );
        let mut err = dx + dy;
        loop {
            if x == x1 && y == y1 {
                return true;
            }
            let tile = self.map[y as usize][x as usize];
            if (x != p0.0 || y != p0.1) && tile != '.' && tile != '>' {
                return false;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Muestra información detallada sobre un tile específico inspeccionado por el usuario.
    pub fn inspect_tile(&mut self, tx: u16, ty: u16) {
        if tx == 0 || ty == 0 {
            return;
        }
        let mouse_pos = Point::new(tx as usize - 1, ty as usize - 1);
        if mouse_pos.y >= self.map.len() || mouse_pos.x >= self.map[0].len() {
            return;
        }

        if self.visible[mouse_pos.y][mouse_pos.x] {
            if let Some(e) = self.entities.iter().find(|e| e.pos == mouse_pos) {
                self.add_log(format!("> INFO: {}", e.name), LogType::Info);
            } else if self.map[mouse_pos.y][mouse_pos.x] == '>' {
                self.add_log("> INFO: Escaleras hacia abajo.".into(), LogType::Info);
            } else {
                self.add_log("> INFO: Terreno despejado.".into(), LogType::Info);
            }
        }
    }

    /// Guarda el estado actual de la partida en el archivo especificado.
    pub fn save_to_file(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let save_data = SaveData {
            hero_pos: self.hero_pos,
            hero_hp: self.hero_hp,
            hero_max_hp: self.hero_max_hp,
            sanity: self.sanity,
            max_sanity: self.max_sanity,
            level: self.level,
            xp: self.xp,
            next_level_xp: self.next_level_xp,
            strength: self.strength,
            agility: self.agility,
            willpower: self.willpower,
            hero_status_effects: self.hero_status_effects.clone(),
            parry_active: self.parry_active,
            invisible_turns: self.invisible_turns,
            equipped_weapon: self.equipped_weapon.clone(),
            equipped_armor: self.equipped_armor.clone(),
            equipped_helmet: self.equipped_helmet.clone(),
            equipped_ring: self.equipped_ring.clone(),
            equipped_amulet: self.equipped_amulet.clone(),
            logs: self.logs.clone(),
            map: self.map.clone(),
            visible: self.visible.clone(),
            explored: self.explored.clone(),
            entities: self.entities.clone(),
            inventory: self.inventory.clone(),
            seed: self.seed,
            depth: self.depth,
        };

        let json = serde_json::to_string_pretty(&save_data)?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Carga el estado guardado desde el archivo especificado.
    pub fn load_from_file(filepath: &str) -> Result<App, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(filepath)?;
        let save_data: SaveData = serde_json::from_str(&content)?;

        let rng = ChaCha8Rng::seed_from_u64(save_data.seed);

        let mut title_menu_state = ListState::default();
        title_menu_state.select(Some(0));

        let mut bestiary_state = ListState::default();
        bestiary_state.select(Some(0));

        let mut app = App {
            hero_pos: save_data.hero_pos,
            hero_hp: save_data.hero_hp,
            hero_max_hp: save_data.hero_max_hp,
            sanity: save_data.sanity,
            max_sanity: save_data.max_sanity,
            level: save_data.level,
            xp: save_data.xp,
            next_level_xp: save_data.next_level_xp,
            strength: save_data.strength,
            agility: save_data.agility,
            willpower: save_data.willpower,
            hero_status_effects: save_data.hero_status_effects,
            parry_active: save_data.parry_active,
            invisible_turns: save_data.invisible_turns,
            equipped_weapon: save_data.equipped_weapon,
            equipped_armor: save_data.equipped_armor,
            equipped_helmet: save_data.equipped_helmet,
            equipped_ring: save_data.equipped_ring,
            equipped_amulet: save_data.equipped_amulet,
            logs: save_data.logs,
            map: save_data.map,
            visible: save_data.visible,
            explored: save_data.explored,
            fov_radius: 6,
            entities: save_data.entities,
            inventory: save_data.inventory,
            seed: save_data.seed,
            depth: save_data.depth,

            drop_mode: false,
            show_descend_prompt: false,

            state: GameState::Playing,
            title_menu_state,
            bestiary_state,
            rng,
        };

        app.add_log("> Partida cargada exitosamente.".into(), LogType::Info);
        Ok(app)
    }

    /// Transforma los muros básicos en glifos de dibujo de caja para una mejor estética.
    pub fn smooth_walls(&mut self) {
        let mut new_map = self.map.clone();
        let height = self.map.len();
        let width = self.map[0].len();
        for y in 0..height {
            for x in 0..width {
                if self.map[y][x] == '#' {
                    let mut mask = 0;
                    if y > 0 && self.map[y - 1][x] == '#' {
                        mask += 1;
                    }
                    if y < height - 1 && self.map[y + 1][x] == '#' {
                        mask += 2;
                    }
                    if x < width - 1 && self.map[y][x + 1] == '#' {
                        mask += 4;
                    }
                    if x > 0 && self.map[y][x - 1] == '#' {
                        mask += 8;
                    }
                    let ch = match mask {
                        1 | 2 | 3 => '║',
                        4 | 8 | 12 => '═',
                        5 => '╚',
                        6 => '╔',
                        9 => '╝',
                        10 => '╗',
                        7 => '╠',
                        11 => '╣',
                        13 => '╩',
                        14 => '╦',
                        15 => '╬',
                        _ => '■',
                    };
                    new_map[y][x] = ch;
                }
            }
        }
        self.map = new_map;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let app = App::new(Some(12345), None, None, 1, None);
        assert_eq!(app.seed, 12345);
        assert_eq!(app.depth, 1);
        assert_eq!(app.hero_hp, 20);
        assert_eq!(app.state, GameState::TitleScreen);
    }

    #[test]
    fn test_start_new_game() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();
        assert_eq!(app.state, GameState::Playing);
        assert!(app.visible[app.hero_pos.y][app.hero_pos.x]);
    }

    #[test]
    fn test_try_move_invalid() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();
        // Trying to move out of bounds (negative offset when at 0,0 if hero was at 0,0) or against a wall
        // Set hero position surrounded by walls
        app.hero_pos = Point::new(0, 0);
        app.map[0][0] = '.';
        app.map[0][1] = '#';
        app.map[1][0] = '#';

        assert!(!app.try_move(-1, 0)); // Out of bounds negative
        assert!(!app.try_move(1, 0));  // Hit wall '#'
    }

    #[test]
    fn test_save_and_load_file() {
        let test_file = "test_savegame.json";
        let mut app = App::new(Some(54321), None, None, 1, None);
        app.start_new_game();
        app.hero_hp = 12;

        app.save_to_file(test_file).expect("Failed to save game");

        let loaded_app = App::load_from_file(test_file).expect("Failed to load game");
        assert_eq!(loaded_app.seed, 54321);
        assert_eq!(loaded_app.hero_hp, 12);
        assert_eq!(loaded_app.state, GameState::Playing);

        let _ = std::fs::remove_file(test_file);
    }

    #[test]
    fn test_use_item_healing_potion() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.hero_hp = 5;
        let potion = Entity {
            pos: Point::new(0, 0),
            glyph: '!',
            color: Color::Magenta,
            name: "Poción de Curación".to_string(),
            e_type: EntityType::Item,
            status_effects: Vec::new(),
        };
        app.inventory.push((potion, 1));

        assert!(app.use_item(0));
        assert_eq!(app.hero_hp, 20); // Healed by 15 up to max 20
        assert!(app.inventory.is_empty());
    }

    #[test]
    fn test_drop_item() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        let initial_entities_count = app.entities.len();
        let potion = Entity {
            pos: Point::new(0, 0),
            glyph: '!',
            color: Color::Magenta,
            name: "Poción de Curación".to_string(),
            e_type: EntityType::Item,
            status_effects: Vec::new(),
        };
        app.inventory.push((potion, 1));

        assert!(app.drop_item(0));
        assert!(app.inventory.is_empty());
        assert_eq!(app.entities.len(), initial_entities_count + 1);
        assert_eq!(app.entities.last().unwrap().pos, app.hero_pos);
    }

    #[test]
    fn test_talking_wall_interaction() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();
        let wall_pos = Point::new(app.hero_pos.x + 1, app.hero_pos.y);
        app.map[wall_pos.y][wall_pos.x] = '.';
        app.entities.push(Entity {
            pos: wall_pos,
            glyph: 'W',
            color: Color::Magenta,
            name: "Pared Parlante".to_string(),
            e_type: EntityType::TalkingWall {
                message: "Un secreto te aguarda.".to_string(),
                whispered: false,
            },
            status_effects: Vec::new(),
        });

        // Hero interacts with talking wall by trying to move into it
        let moved = app.try_move(1, 0);
        assert!(moved);
        assert_ne!(app.hero_pos, wall_pos); // Cannot step into wall
        assert!(app.logs.iter().any(|log| log.text.contains("Un secreto te aguarda.")));
    }

    #[test]
    fn test_echo_altar_interaction() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();
        let altar_pos = Point::new(app.hero_pos.x + 1, app.hero_pos.y);
        app.map[altar_pos.y][altar_pos.x] = '.';
        app.entities.push(Entity {
            pos: altar_pos,
            glyph: 'A',
            color: Color::Red,
            name: "Altar de Ecos".to_string(),
            e_type: EntityType::EchoAltar { used: false },
            status_effects: Vec::new(),
        });

        let initial_hp = app.hero_hp;
        let moved = app.try_move(1, 0);
        assert!(moved);
        assert_eq!(app.hero_hp, initial_hp - 5); // 5 HP traded
        assert!(app.explored[0][0]); // Map revealed
    }

    #[test]
    fn test_spirit_negotiation() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();
        let thief_pos = Point::new(app.hero_pos.x + 1, app.hero_pos.y);
        app.map[thief_pos.y][thief_pos.x] = '.';
        app.entities.push(Entity {
            pos: thief_pos,
            glyph: 'L',
            color: Color::Blue,
            name: "Ladrón".to_string(),
            e_type: EntityType::Mob {
                hp: 18,
                max_hp: 18,
                state: EnemyState::Wandering,
                ai: EnemyAI::Coward,
                min_dmg: 2,
                max_dmg: 5,
                defense: 2,
                pacified: false,
            },
            status_effects: Vec::new(),
        });

        let initial_sanity = app.sanity;
        let action = app.try_move(1, 0);
        assert!(action);
        assert_eq!(app.sanity, initial_sanity - 10);
        if let EntityType::Mob { pacified, .. } = &app.entities.last().unwrap().e_type {
            assert!(pacified);
        } else {
            panic!("Expected Mob entity");
        }
    }

    #[test]
    fn test_use_pushback_and_parry() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();

        // Test Parry
        assert!(app.use_parry());
        assert!(app.parry_active);

        // Test Pushback with adjacent mob
        let target_pos = Point::new(app.hero_pos.x + 1, app.hero_pos.y);
        app.map[target_pos.y][target_pos.x] = '.';
        let empty_pos = Point::new(app.hero_pos.x + 2, app.hero_pos.y);
        app.map[empty_pos.y][empty_pos.x] = '.';

        app.entities.push(Entity {
            pos: target_pos,
            glyph: 'g',
            color: Color::Red,
            name: "Gnoll".to_string(),
            e_type: EntityType::Mob {
                hp: 20,
                max_hp: 20,
                state: EnemyState::Aggressive,
                ai: EnemyAI::Melee,
                min_dmg: 4,
                max_dmg: 6,
                defense: 1,
                pacified: false,
            },
            status_effects: Vec::new(),
        });

        assert!(app.use_pushback());
        assert_eq!(app.entities.last().unwrap().pos, empty_pos);
    }

    #[test]
    fn test_scroll_usage() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();

        let scroll = Entity {
            pos: Point::new(0, 0),
            glyph: '?',
            color: Color::LightCyan,
            name: "Pergamino de Rayo".to_string(),
            e_type: EntityType::Scroll { scroll_type: ScrollType::Lightning },
            status_effects: Vec::new(),
        };
        app.inventory.push((scroll, 1));

        let mob_pos = Point::new(app.hero_pos.x + 1, app.hero_pos.y);
        app.entities.push(Entity {
            pos: mob_pos,
            glyph: 'g',
            color: Color::Red,
            name: "Gnoll".to_string(),
            e_type: EntityType::Mob {
                hp: 20,
                max_hp: 20,
                state: EnemyState::Aggressive,
                ai: EnemyAI::Melee,
                min_dmg: 4,
                max_dmg: 6,
                defense: 1,
                pacified: false,
            },
            status_effects: Vec::new(),
        });

        assert!(app.use_item(0));
        assert!(app.inventory.is_empty());
        if let EntityType::Mob { hp, .. } = app.entities.last().unwrap().e_type {
            assert_eq!(hp, 8); // 20 - 12 = 8
        }
    }

    #[test]
    fn test_door_interaction() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();

        let door_pos = Point::new(app.hero_pos.x + 1, app.hero_pos.y);
        app.map[door_pos.y][door_pos.x] = '.';
        app.entities.push(Entity {
            pos: door_pos,
            glyph: '+',
            color: Color::Yellow,
            name: "Puerta de Madera".to_string(),
            e_type: EntityType::Door {
                locked: false,
                secret: false,
                open: false,
            },
            status_effects: Vec::new(),
        });

        // First bump opens door
        let action = app.try_move(1, 0);
        assert!(action);
        if let EntityType::Door { open, .. } = app.entities.last().unwrap().e_type {
            assert!(open);
        } else {
            panic!("Expected Door entity");
        }

        // Second bump moves hero onto opened door tile
        let moved = app.try_move(1, 0);
        assert!(moved);
        assert_eq!(app.hero_pos, door_pos);
    }

    #[test]
    fn test_hazard_interaction() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();

        let hazard_pos = Point::new(app.hero_pos.x + 1, app.hero_pos.y);
        app.map[hazard_pos.y][hazard_pos.x] = '.';
        app.entities.push(Entity {
            pos: hazard_pos,
            glyph: '^',
            color: Color::DarkGray,
            name: "Trampa de Pinchos".to_string(),
            e_type: EntityType::Hazard { hazard_type: HazardType::Spikes },
            status_effects: Vec::new(),
        });

        let initial_hp = app.hero_hp;
        let action = app.try_move(1, 0);
        assert!(action);
        assert_eq!(app.hero_pos, hazard_pos);
        assert_eq!(app.hero_hp, initial_hp - 4);
    }

    #[test]
    fn test_add_xp_and_level_up() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();

        assert_eq!(app.level, 1);
        app.add_xp(60); // 60 >= 50 (next_level_xp)
        assert_eq!(app.level, 2);
        assert_eq!(app.strength, 6);
        assert_eq!(app.agility, 6);
        assert_eq!(app.willpower, 6);
    }

    #[test]
    fn test_equip_armor_and_helmet() {
        let mut app = App::new(Some(12345), None, None, 1, None);
        app.start_new_game();

        let armor = Entity {
            pos: Point::new(0, 0),
            glyph: '[',
            color: Color::Gray,
            name: "Cota de Malla".to_string(),
            e_type: EntityType::Armor { defense: 4 },
            status_effects: Vec::new(),
        };
        app.inventory.push((armor, 1));

        assert!(app.use_item(0));
        assert!(app.inventory.is_empty());
        assert_eq!(app.equipped_armor, Some(("Cota de Malla".to_string(), 4)));
    }
}
