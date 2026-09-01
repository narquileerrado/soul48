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
    },
    Item,
    Weapon {
        min_dmg: i32,
        max_dmg: i32,
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub pos: Point,
    pub glyph: char,
    pub color: Color,
    pub name: String,
    pub e_type: EntityType,
}

/// Estructura serializable para la persistencia del juego en disco.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub hero_pos: Point,
    pub hero_hp: i32,
    pub hero_max_hp: i32,
    pub equipped_weapon: Option<(String, i32, i32)>,
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
    pub equipped_weapon: Option<(String, i32, i32)>,
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
            equipped_weapon: weapon,
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
                hp, state, defense, ..
            } => {
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
                    entity_index_to_remove = Some(index);
                }
                self.entities[index] = entity_clone;
                move_allowed = false;
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
            EntityType::Item | EntityType::Key | EntityType::Weapon { .. } => {
                let is_stackable =
                    matches!(entity_clone.e_type, EntityType::Item | EntityType::Key);

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
                        },
                        1,
                    ));
                }
                self.equipped_weapon = Some((item.name.clone(), min_dmg, max_dmg));
                self.add_log(format!("> Equipas {}.", item.name), LogType::Info);
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

        for i in 0..self.entities.len() {
            let (mut current_state, ai, ex, ey, name) = match &self.entities[i].e_type {
                EntityType::Mob { state, ai, .. } => (
                    state.clone(),
                    ai.clone(),
                    self.entities[i].pos.x as isize,
                    self.entities[i].pos.y as isize,
                    self.entities[i].name.clone(),
                ),
                _ => continue,
            };

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
                        if let EntityType::Mob {
                            min_dmg, max_dmg, ..
                        } = self.entities[i].e_type
                        {
                            let dmg = self.rng.gen_range(min_dmg..=max_dmg);
                            self.hero_hp = (self.hero_hp - dmg).max(0);
                            messages.push((
                                format!("> {} te golpea ({} daño)", name, dmg),
                                LogType::Warning,
                            ));
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
            equipped_weapon: self.equipped_weapon.clone(),
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
            equipped_weapon: save_data.equipped_weapon,
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
        });

        let initial_hp = app.hero_hp;
        let moved = app.try_move(1, 0);
        assert!(moved);
        assert_eq!(app.hero_hp, initial_hp - 5); // 5 HP traded
        assert!(app.explored[0][0]); // Map revealed
    }
}
