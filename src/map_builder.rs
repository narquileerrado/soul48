use crate::app::{EnemyAI, EnemyState, Entity, EntityType, Point};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ratatui::style::Color;

/// Representa una región rectangular en el mapa, utilizada para generar habitaciones.
struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    /// Crea un nuevo rectángulo dadas sus coordenadas de origen, ancho y alto.
    fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    /// Calcula el punto central del rectángulo.
    fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    /// Determina si este rectángulo se solapa con otro.
    fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

/// Define las características base para un tipo de enemigo antes de ser instanciado.
struct EnemyTemplate {
    name: &'static str,
    glyph: char,
    color: Color,
    hp: i32,
    defense: i32,
    damage: (i32, i32),
    ai: EnemyAI,
    spawn_weight: i32,
}

/// Encargado de la generación procedimental del nivel, incluyendo geografía y entidades.
pub struct MapBuilder {
    pub map: Vec<Vec<char>>,
    pub hero_start: Point,
    pub entities: Vec<Entity>,
}

impl MapBuilder {
    /// Construye un nuevo nivel utilizando una semilla aleatoria y ajustando la dificultad según la profundidad.
    pub fn new(seed: u64, depth: u32) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let map_width = 60;
        let map_height = 25;
        let mut map = vec![vec!['#'; map_width]; map_height];
        let mut rooms: Vec<Rect> = Vec::new();
        let mut entities = Vec::new();
        let mut hero_start = Point::new(0, 0);

        // Intento de generación de habitaciones aleatorias
        for _ in 0..12 {
            let w = rng.gen_range(5..=10);
            let h = rng.gen_range(5..=10);
            let x = rng.gen_range(1..map_width - w - 1);
            let y = rng.gen_range(1..map_height - h - 1);
            let new_room = Rect::new(x, y, w, h);

            if !rooms.iter().any(|r| new_room.intersect(r)) {
                // Tallar la habitación en el mapa de muros
                for ry in new_room.y1..=new_room.y2 {
                    for rx in new_room.x1..=new_room.x2 {
                        map[ry][rx] = '.';
                    }
                }
                let new_center = new_room.center();
                if rooms.is_empty() {
                    hero_start = new_center;
                } else {
                    // Conexión con la habitación anterior mediante túneles en L
                    let prev_center = rooms.last().unwrap().center();
                    if rng.gen_bool(0.5) {
                        for cx in std::cmp::min(prev_center.x, new_center.x)
                            ..=std::cmp::max(prev_center.x, new_center.x)
                        {
                            map[prev_center.y][cx] = '.';
                        }
                        for cy in std::cmp::min(prev_center.y, new_center.y)
                            ..=std::cmp::max(prev_center.y, new_center.y)
                        {
                            map[cy][new_center.x] = '.';
                        }
                    } else {
                        for cy in std::cmp::min(prev_center.y, new_center.y)
                            ..=std::cmp::max(prev_center.y, new_center.y)
                        {
                            map[cy][prev_center.x] = '.';
                        }
                        for cx in std::cmp::min(prev_center.x, new_center.x)
                            ..=std::cmp::max(prev_center.x, new_center.x)
                        {
                            map[new_center.y][cx] = '.';
                        }
                    }

                    // Probabilidad de aparición de enemigos según profundidad
                    if rng.gen_range(0..100) < (30 + depth * 2).min(65) {
                        entities.push(Self::spawn_random_enemy(&mut rng, new_center, depth));
                    }

                    // Probabilidad de aparición de objetos (pociones)
                    if rng.gen_range(0..100) < 20 {
                        let mut item_pos = new_center;
                        if item_pos.x + 1 < map_width && map[item_pos.y][item_pos.x + 1] == '.' {
                            item_pos.x += 1;
                        }
                        entities.push(Entity {
                            pos: item_pos,
                            glyph: '!',
                            color: Color::Magenta,
                            name: "Poción de Curación".to_string(),
                            e_type: EntityType::Item,
                        });
                    }
                }
                rooms.push(new_room);
            }
        }

        // Colocación estratégica de cofres y llaves
        if rooms.len() > 2 {
            let chest_pos = rooms[1].center();
            entities.push(Entity {
                pos: chest_pos,
                glyph: 'C',
                color: Color::Yellow,
                name: "Cofre de Madera".into(),
                e_type: EntityType::Chest { locked: true },
            });
            let key_pos = rooms[2].center();
            entities.push(Entity {
                pos: key_pos,
                glyph: 'k',
                color: Color::Rgb(200, 200, 0),
                name: "Llave de Hierro".into(),
                e_type: EntityType::Key,
            });
        }

        // Colocación del punto de salida (escaleras)
        if let Some(last_room) = rooms.last() {
            let stairs_pos = last_room.center();
            map[stairs_pos.y][stairs_pos.x] = '>';
        }

        MapBuilder {
            map,
            hero_start,
            entities,
        }
    }

    /// Selecciona y configura un enemigo aleatorio basado en pesos de aparición y dificultad.
    fn spawn_random_enemy(rng: &mut ChaCha8Rng, pos: Point, depth: u32) -> Entity {
        let catalog = vec![
            EnemyTemplate {
                name: "Murciélago",
                glyph: 'b',
                color: Color::DarkGray,
                hp: 6,
                defense: 0,
                damage: (1, 2),
                ai: EnemyAI::Wandering,
                spawn_weight: 30,
            },
            EnemyTemplate {
                name: "Serpiente",
                glyph: 's',
                color: Color::Green,
                hp: 12,
                defense: 1,
                damage: (2, 4),
                ai: EnemyAI::Melee,
                spawn_weight: 25,
            },
            EnemyTemplate {
                name: "Ladrón",
                glyph: 'L',
                color: Color::Blue,
                hp: 18,
                defense: 2,
                damage: (2, 5),
                ai: EnemyAI::Coward,
                spawn_weight: 20,
            },
            EnemyTemplate {
                name: "Gnoll",
                glyph: 'g',
                color: Color::Rgb(150, 75, 0),
                hp: 28,
                defense: 3,
                damage: (4, 7),
                ai: EnemyAI::Melee,
                spawn_weight: 15,
            },
            EnemyTemplate {
                name: "Cofre Sospechoso",
                glyph: 'C',
                color: Color::Yellow,
                hp: 45,
                defense: 5,
                damage: (6, 12),
                ai: EnemyAI::Stationary,
                spawn_weight: 10,
            },
        ];

        let total_weight: i32 = catalog.iter().map(|e| e.spawn_weight).sum();
        let mut roll = rng.gen_range(0..total_weight);

        let mut selected = &catalog[0];
        for template in catalog.iter() {
            if roll < template.spawn_weight {
                selected = template;
                break;
            }
            roll -= template.spawn_weight;
        }

        let difficulty_bonus = (depth as i32 - 1) * 2;

        Entity {
            pos,
            glyph: selected.glyph,
            color: selected.color,
            name: selected.name.to_string(),
            e_type: EntityType::Mob {
                hp: selected.hp + difficulty_bonus,
                max_hp: selected.hp + difficulty_bonus,
                state: if selected.ai == EnemyAI::Wandering {
                    EnemyState::Wandering
                } else {
                    EnemyState::Asleep
                },
                ai: selected.ai.clone(),
                min_dmg: selected.damage.0 + (difficulty_bonus / 4),
                max_dmg: selected.damage.1 + (difficulty_bonus / 4),
                defense: selected.defense + (difficulty_bonus / 6),
            },
        }
    }
}
