use crate::map_builder;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ratatui::{style::Color, widgets::ListState};

#[derive(Clone, PartialEq)]
pub enum GameState {
    TitleScreen,
    Playing,
    GameOver,
    Bestiary,
}

#[derive(Clone, PartialEq)]
pub enum EnemyState {
    Asleep,
    Wandering,
    Aggressive,
}

#[derive(Clone, PartialEq)]
pub enum EnemyAI {
    Melee,
    Wandering,
    Coward,
    Stationary,
}

#[derive(Clone, PartialEq)]
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
}

pub enum LogType {
    Info,
    Combat,
    Item,
    Warning,
}

pub struct LogMessage {
    pub text: String,
    pub l_type: LogType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct Entity {
    pub pos: Point,
    pub glyph: char,
    pub color: Color,
    pub name: String,
    pub e_type: EntityType,
}

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

    // Flags de mecánicas
    pub should_descend: bool,
    pub drop_mode: bool,
    pub show_descend_prompt: bool,

    // Estado del juego
    pub state: GameState,
    pub title_menu_state: ListState,
    pub bestiary_state: ListState,
    rng: ChaCha8Rng,
}

impl App {
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

        // Determinamos el estado inicial. Si bajamos de piso, ya no mostramos el menú principal
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

            should_descend: false,
            drop_mode: false,
            show_descend_prompt: false,

            state: initial_state,
            title_menu_state,
            bestiary_state,
            rng,
        };

        // Si ya estamos jugando (ej: bajamos al piso 2), renderizamos paredes y visión inmediatamente
        if app.state == GameState::Playing {
            app.smooth_walls();
            app.calculate_fov();
        }

        app
    }

    // Nueva función para manejar todas las interacciones con entidades
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
                // Aplicar defensa del enemigo
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

                        // Reemplazamos el cofre por un arma
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

        (move_allowed, true) // action_taken is always true if we interact
    }

    pub fn start_new_game(&mut self) {
        self.smooth_walls();
        self.calculate_fov();
        self.state = GameState::Playing;
        self.add_log(
            "> Un viaje traicionero comienza... PISO 1".into(),
            LogType::Info,
        );
    }

    pub fn try_move(&mut self, dx: isize, dy: isize) -> bool {
        let new_pos = Point::new(
            (self.hero_pos.x as isize + dx) as usize,
            (self.hero_pos.y as isize + dy) as usize,
        );

        if new_pos.y >= self.map.len() || new_pos.x >= self.map[0].len() {
            return false;
        }

        let tile = self.map[new_pos.y][new_pos.x];
        if tile != '.' && tile != '>' {
            return false;
        }

        if tile == '>' {
            self.show_descend_prompt = true;
            return true; // Es una acción válida que consume un turno
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
            action_taken = true; // El simple hecho de moverse es una acción
        }

        action_taken
    }

    pub fn confirm_descent(&mut self, confirmed: bool) {
        if confirmed {
            self.should_descend = true;
        }
        self.show_descend_prompt = false;
    }

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

    pub fn add_log(&mut self, text: String, l_type: LogType) {
        self.logs.push(LogMessage { text, l_type });
        if self.logs.len() > 5 {
            self.logs.remove(0);
        }
    }

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

            // Lógica de Despertar
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

    fn move_mob(&mut self, idx: usize, dx: isize, dy: isize) {
        let new_pos = Point::new(
            (self.entities[idx].pos.x as isize + dx) as usize,
            (self.entities[idx].pos.y as isize + dy) as usize,
        );

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
                    if (x - hx).pow(2) + (y - hy).pow(2) <= self.fov_radius.pow(2) {
                        if self.has_los((hx, hy), (x, y)) {
                            self.visible[y as usize][x as usize] = true;
                            self.explored[y as usize][x as usize] = true;
                        }
                    }
                }
            }
        }
    }

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
