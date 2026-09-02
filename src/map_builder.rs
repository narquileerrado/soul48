use crate::app::{EnemyAI, EnemyState, Entity, EntityType, HazardType, Point, ScrollType, SpecialRoomType};
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
                            color: crate::theme::HUESO,
                            name: "Poción de Curación".to_string(),
                            e_type: EntityType::Item,
                            status_effects: Vec::new(),
                        });
                    }

                    // Probabilidad de aparición de equipamiento (Armaduras, Cascos, Anillos, Amuletos)
                    if rng.gen_range(0..100) < 15 {
                        let mut eq_pos = new_center;
                        if eq_pos.x > 1 && map[eq_pos.y][eq_pos.x - 1] == '.' {
                            eq_pos.x -= 1;
                        }
                        let eq_entity = match rng.gen_range(0..4) {
                            0 => Entity {
                                pos: eq_pos,
                                glyph: '[',
                                color: Color::Gray,
                                name: "Cota de Malla".into(),
                                e_type: EntityType::Armor { defense: 3 + depth as i32 },
                                status_effects: Vec::new(),
                            },
                            1 => Entity {
                                pos: eq_pos,
                                glyph: '^',
                                color: Color::Yellow,
                                name: "Yelmo de Hierro".into(),
                                e_type: EntityType::Helmet { defense: 2 + depth as i32 },
                                status_effects: Vec::new(),
                            },
                            2 => Entity {
                                pos: eq_pos,
                                glyph: '=',
                                color: Color::Yellow,
                                name: "Anillo de Fuerza".into(),
                                e_type: EntityType::Ring { stat_bonus: 2 },
                                status_effects: Vec::new(),
                            },
                            _ => Entity {
                                pos: eq_pos,
                                glyph: '"',
                                color: Color::LightCyan,
                                name: "Amuleto de Claridad".into(),
                                e_type: EntityType::Amulet { sanity_bonus: 20 },
                                status_effects: Vec::new(),
                            },
                        };
                        entities.push(eq_entity);
                    }

                    // Probabilidad de aparición de pergaminos mágicos
                    if rng.gen_range(0..100) < 15 {
                        let mut scroll_pos = new_center;
                        if scroll_pos.y + 1 < map_height && map[scroll_pos.y + 1][scroll_pos.x] == '.' {
                            scroll_pos.y += 1;
                        }
                        let (s_type, s_name) = match rng.gen_range(0..4) {
                            0 => (ScrollType::Lightning, "Pergamino de Rayo"),
                            1 => (ScrollType::Fireball, "Pergamino de Bola de Fuego"),
                            2 => (ScrollType::Teleport, "Pergamino de Teletransporte"),
                            _ => (ScrollType::Invisibility, "Pergamino de Invisibilidad"),
                        };
                        entities.push(Entity {
                            pos: scroll_pos,
                            glyph: '?',
                            color: Color::LightCyan,
                            name: s_name.to_string(),
                            e_type: EntityType::Scroll { scroll_type: s_type },
                            status_effects: Vec::new(),
                        });
                    }
                }
                rooms.push(new_room);
            }
        }

        // Colocación especial de Boss en el Piso 48 (Archidemonio) o en Pisos Múltiplos de 5
        if depth == 48 && !rooms.is_empty() {
            let boss_pos = rooms.last().unwrap().center();
            entities.push(Entity {
                pos: boss_pos,
                glyph: 'D',
                color: Color::LightRed,
                name: "ARCHIDEMONIO DEL SILENCIO".into(),
                e_type: EntityType::Mob {
                    hp: 150,
                    max_hp: 150,
                    state: EnemyState::Aggressive,
                    ai: EnemyAI::Melee,
                    min_dmg: 8,
                    max_dmg: 16,
                    defense: 6,
                    pacified: false,
                },
                status_effects: Vec::new(),
            });
        } else if depth % 5 == 0 && rooms.len() > 1 {
            let boss_pos = rooms.last().unwrap().center();
            entities.push(Entity {
                pos: boss_pos,
                glyph: 'B',
                color: Color::Rgb(255, 60, 60),
                name: format!("Guardián del Piso {}", depth),
                e_type: EntityType::Mob {
                    hp: 50 + (depth as i32 * 3),
                    max_hp: 50 + (depth as i32 * 3),
                    state: EnemyState::Aggressive,
                    ai: EnemyAI::Melee,
                    min_dmg: 5 + (depth as i32 / 2),
                    max_dmg: 10 + (depth as i32 / 2),
                    defense: 4 + (depth as i32 / 5),
                    pacified: false,
                },
                status_effects: Vec::new(),
            });
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
                status_effects: Vec::new(),
            });
            let key_pos = rooms[2].center();
            entities.push(Entity {
                pos: key_pos,
                glyph: 'k',
                color: crate::theme::HUESO,
                name: "Llave de Hierro".into(),
                e_type: EntityType::Key,
                status_effects: Vec::new(),
            });
        }

        // Generación de Puertas en entradas de habitaciones
        for room in rooms.iter().skip(1) {
            let door_pos = Point::new(room.x1, room.center().y);
            if map[door_pos.y][door_pos.x] == '.' {
                let is_locked = rng.gen_bool(0.2);
                let is_secret = rng.gen_bool(0.1);
                entities.push(Entity {
                    pos: door_pos,
                    glyph: if is_secret { '║' } else if is_locked { '+' } else { '+' },
                    color: if is_locked { Color::Red } else { Color::Rgb(160, 100, 40) },
                    name: if is_secret { "Muro Sospechoso".into() } else if is_locked { "Puerta Cerrada con Llave".into() } else { "Puerta de Madera".into() },
                    e_type: EntityType::Door {
                        locked: is_locked,
                        secret: is_secret,
                        open: false,
                    },
                    status_effects: Vec::new(),
                });
            }
        }

        // Generación de Peligros Ambientales (Pinchos, Ácido, Fuego, Aceite)
        for room in rooms.iter().skip(1) {
            if rng.gen_bool(0.3) {
                let h_pos = Point::new(room.x1 + 1, room.y1 + 1);
                if map[h_pos.y][h_pos.x] == '.' {
                    let (h_type, glyph, color, name) = match rng.gen_range(0..4) {
                        0 => (HazardType::Spikes, '^', Color::DarkGray, "Trampa de Pinchos"),
                        1 => (HazardType::Acid, '~', Color::Green, "Pozo de Ácido"),
                        2 => (HazardType::Oil, 'o', Color::Rgb(100, 100, 50), "Charco de Aceite"),
                        _ => (HazardType::Fire, '&', Color::Red, "Fuego"),
                    };
                    entities.push(Entity {
                        pos: h_pos,
                        glyph,
                        color,
                        name: name.into(),
                        e_type: EntityType::Hazard { hazard_type: h_type },
                        status_effects: Vec::new(),
                    });
                }
            }
        }

        // Generación de Salas Especiales (Armería, Biblioteca, Círculo Ritual)
        if rooms.len() > 5 {
            let s_room = &rooms[5];
            let marker_pos = s_room.center();
            let room_type = match rng.gen_range(0..3) {
                0 => SpecialRoomType::Armory,
                1 => SpecialRoomType::Library,
                _ => SpecialRoomType::RitualCircle,
            };
            entities.push(Entity {
                pos: marker_pos,
                glyph: 'R',
                color: Color::Rgb(255, 215, 0),
                name: "Marca de Sala Especial".into(),
                e_type: EntityType::SpecialRoomMarker { room_type },
                status_effects: Vec::new(),
            });
        }

        // Colocación de Pared Parlante
        if rooms.len() > 3 {
            let wall_room = &rooms[3];
            let wall_pos = Point::new(wall_room.x1, wall_room.y1);
            let whispers = [
                "Recuerda... tu voz fue lo primero que te robaron.",
                "En el piso 48, el Archidemonio aguarda tu ascenso.",
                "Los cofres dorados a veces respiran cuando no los miras.",
                "Ofrecer tu sangre al Altar de Ecos revelará la verdad oculta.",
            ];
            let msg = whispers[rng.gen_range(0..whispers.len())].to_string();

            entities.push(Entity {
                pos: wall_pos,
                glyph: 'W',
                color: crate::theme::VIOLETA,
                name: "Pared de los Lamentos".into(),
                e_type: EntityType::TalkingWall {
                    message: msg,
                    whispered: false,
                },
                status_effects: Vec::new(),
            });
        }

        // Colocación de Altar de Ecos
        if rooms.len() > 4 {
            let altar_pos = rooms[4].center();
            entities.push(Entity {
                pos: altar_pos,
                glyph: 'A',
                color: crate::theme::ROJO_ALTAR,
                name: "Altar de Ecos".into(),
                e_type: EntityType::EchoAltar { used: false },
                status_effects: Vec::new(),
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
                color: Color::Rgb(110, 110, 110),
                hp: 6,
                defense: 0,
                damage: (1, 2),
                ai: EnemyAI::Wandering,
                spawn_weight: 30,
            },
            EnemyTemplate {
                name: "Serpiente",
                glyph: 's',
                color: Color::Rgb(78, 154, 78),
                hp: 12,
                defense: 1,
                damage: (2, 4),
                ai: EnemyAI::Melee,
                spawn_weight: 25,
            },
            EnemyTemplate {
                name: "Ladrón",
                glyph: 'L',
                color: Color::Rgb(92, 127, 209),
                hp: 18,
                defense: 2,
                damage: (2, 5),
                ai: EnemyAI::Coward,
                spawn_weight: 20,
            },
            EnemyTemplate {
                name: "Gnoll",
                glyph: 'g',
                color: Color::Rgb(184, 106, 40),
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
                pacified: false,
            },
            status_effects: Vec::new(),
        }
    }
}
