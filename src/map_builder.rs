use crate::app::{
    EnemyAI, EnemyState, Entity, EntityType, HazardType, Point, ScrollType, SpecialRoomType,
};
use crate::balance;
use crate::bestiary;
use crate::world::tramo;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

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
        let tramo = tramo::de_piso(depth);
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
                                color: crate::theme::HUESO,
                                name: "Cota de Malla".into(),
                                e_type: EntityType::Armor {
                                    defense: 3 + depth as i32,
                                },
                                status_effects: Vec::new(),
                            },
                            1 => Entity {
                                pos: eq_pos,
                                glyph: '^',
                                color: crate::theme::HUESO,
                                name: "Yelmo de Hierro".into(),
                                e_type: EntityType::Helmet {
                                    defense: 2 + depth as i32,
                                },
                                status_effects: Vec::new(),
                            },
                            2 => Entity {
                                pos: eq_pos,
                                glyph: '=',
                                color: crate::theme::HUESO,
                                name: "Anillo de Fuerza".into(),
                                e_type: EntityType::Ring { stat_bonus: 2 },
                                status_effects: Vec::new(),
                            },
                            _ => Entity {
                                pos: eq_pos,
                                glyph: '"',
                                color: crate::theme::VIOLETA,
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
                        if scroll_pos.y + 1 < map_height
                            && map[scroll_pos.y + 1][scroll_pos.x] == '.'
                        {
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
                            color: crate::theme::VIOLETA,
                            name: s_name.to_string(),
                            e_type: EntityType::Scroll {
                                scroll_type: s_type,
                            },
                            status_effects: Vec::new(),
                        });
                    }
                }
                rooms.push(new_room);
            }
        }

        // Un jefe cada seis pisos. Los de fin de tramo llevan el nombre del
        // tramo y lo cierran; los intermedios son un eco más flojo. Antes eran
        // nueve Guardianes con nombre autogenerado y ninguna identidad.
        if depth == balance::descenso::PISO_FINAL && !rooms.is_empty() {
            let boss_pos = rooms.last().unwrap().center();
            entities.push(Entity {
                pos: boss_pos,
                glyph: 'D',
                color: crate::theme::VIOLETA,
                name: bestiary::ARCHIDEMONIO.into(),
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
        } else if depth.is_multiple_of(balance::descenso::CADA_CUANTOS_JEFE) && rooms.len() > 1 {
            let boss_pos = rooms.last().unwrap().center();
            let cierra = tramo::cierra_tramo(depth);
            // el Guardián del tramo pega más fuerte que el eco de mitad
            let escala = if cierra { 1.0 } else { 0.7 };
            let vida = ((50 + depth as i32 * 3) as f32 * escala) as i32;
            entities.push(Entity {
                pos: boss_pos,
                glyph: 'B',
                color: crate::theme::ROJO_ALTAR,
                name: if cierra {
                    tramo.jefe.to_string()
                } else {
                    format!("Eco del {}", tramo.jefe)
                },
                e_type: EntityType::Mob {
                    hp: vida,
                    max_hp: vida,
                    state: EnemyState::Aggressive,
                    ai: EnemyAI::Melee,
                    min_dmg: ((5 + depth as i32 / 2) as f32 * escala) as i32,
                    max_dmg: ((10 + depth as i32 / 2) as f32 * escala) as i32,
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
                color: crate::theme::COFRE,
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
                // El pasaje secreto se disfraza de muro. La puerta con llave
                // comparte el glifo '+' con la de madera y se distingue por el
                // oro: es la misma puerta, cerrada.
                let (glyph, color, name) = if is_secret {
                    ('║', crate::theme::MURO, "Muro Sospechoso")
                } else if is_locked {
                    ('+', crate::theme::ORO, "Puerta Cerrada con Llave")
                } else {
                    ('+', crate::theme::CENIZA, "Puerta de Madera")
                };
                entities.push(Entity {
                    pos: door_pos,
                    glyph,
                    color,
                    name: name.into(),
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
                        0 => (
                            HazardType::Spikes,
                            '^',
                            crate::theme::AMBAR,
                            "Trampa de Pinchos",
                        ),
                        1 => (HazardType::Acid, '~', crate::theme::AMBAR, "Pozo de Ácido"),
                        2 => (
                            HazardType::Oil,
                            'o',
                            crate::theme::AMBAR,
                            "Charco de Aceite",
                        ),
                        _ => (HazardType::Fire, '&', crate::theme::AMBAR, "Fuego"),
                    };
                    entities.push(Entity {
                        pos: h_pos,
                        glyph,
                        color,
                        name: name.into(),
                        e_type: EntityType::Hazard {
                            hazard_type: h_type,
                        },
                        status_effects: Vec::new(),
                    });
                }
            }
        }

        // Generación de Salas Especiales (Armería, Biblioteca, Círculo Ritual)
        //
        // La marca sola no alcanzaba: la sala anunciaba pergaminos arcanos o
        // metal templado en el historial y por dentro estaba tan vacía como
        // cualquier otra. Ahora el anuncio describe lo que hay.
        if rooms.len() > 5 {
            let s_room = &rooms[5];
            let marker_pos = s_room.center();
            let room_type = match rng.gen_range(0..3) {
                0 => SpecialRoomType::Armory,
                1 => SpecialRoomType::Library,
                _ => SpecialRoomType::RitualCircle,
            };

            let botin = Self::botin_de_sala(&mut rng, &room_type, depth);
            entities.push(Entity {
                pos: marker_pos,
                glyph: 'R',
                color: crate::theme::ORO,
                name: "Marca de Sala Especial".into(),
                e_type: EntityType::SpecialRoomMarker { room_type },
                status_effects: Vec::new(),
            });
            // el contenido se reparte por la sala; `casilla_libre` termina de
            // acomodar lo que caiga encima de algo
            for (i, mut e) in botin.into_iter().enumerate() {
                e.pos = Point::new(
                    (s_room.x1 + 1 + i).min(s_room.x2.saturating_sub(1)),
                    s_room.y1 + 1,
                );
                entities.push(e);
            }
        }

        // Colocación de Pared Parlante
        if rooms.len() > 3 {
            let wall_room = &rooms[3];
            let wall_pos = Point::new(wall_room.x1, wall_room.y1);
            // cada tramo tiene sus propias voces: la pared del piso 3 y la del
            // piso 40 ya no dicen lo mismo
            let susurros = tramo.susurros;
            let msg = susurros[rng.gen_range(0..susurros.len())].to_string();

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

        // Colocación del punto de salida (escaleras). Va antes de resolver
        // colisiones para que la casilla quede reservada.
        if let Some(last_room) = rooms.last() {
            let stairs_pos = last_room.center();
            map[stairs_pos.y][stairs_pos.x] = '>';
        }

        // Casi todo se genera apuntando al centro de una sala: el enemigo, el
        // cofre, el altar, la marca de sala especial y el jefe compiten por la
        // misma casilla. Como `try_move` resuelve con la *primera* entidad que
        // encuentra en una posición, todo lo que quedaba debajo era inalcanzable
        // —y en el piso 48 el jefe tapaba la escalera—. Acá cada entidad se
        // corre a la casilla transitable libre más cercana.
        let mut ocupadas: HashSet<(usize, usize)> = HashSet::new();
        ocupadas.insert((hero_start.x, hero_start.y));
        let mut acomodadas = Vec::with_capacity(entities.len());
        for mut e in entities {
            // sin lugar cerca, la entidad no llega a existir: es preferible a
            // que tape a otra
            if let Some(p) = Self::casilla_libre(e.pos, &map, &ocupadas) {
                e.pos = p;
                ocupadas.insert((p.x, p.y));
                acomodadas.push(e);
            }
        }

        MapBuilder {
            map,
            hero_start,
            entities: acomodadas,
        }
    }

    /// Casilla transitable libre más cercana a `origen`, buscando en anillos.
    ///
    /// Devuelve `None` si en todo el radio no hay lugar.
    fn casilla_libre(
        origen: Point,
        map: &[Vec<char>],
        ocupadas: &HashSet<(usize, usize)>,
    ) -> Option<Point> {
        const RADIO_MAX: isize = 8;
        let (alto, ancho) = (map.len() as isize, map[0].len() as isize);

        for radio in 0..=RADIO_MAX {
            for dy in -radio..=radio {
                for dx in -radio..=radio {
                    // sólo el borde del anillo: el interior ya se miró
                    if dx.abs().max(dy.abs()) != radio {
                        continue;
                    }
                    let (x, y) = (origen.x as isize + dx, origen.y as isize + dy);
                    if x < 0 || y < 0 || x >= ancho || y >= alto {
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if map[y][x] == '.' && !ocupadas.contains(&(x, y)) {
                        return Some(Point::new(x, y));
                    }
                }
            }
        }
        None
    }

    /// Lo que guarda cada sala especial.
    fn botin_de_sala(rng: &mut ChaCha8Rng, tipo: &SpecialRoomType, depth: u32) -> Vec<Entity> {
        let bonus = depth as i32;
        match tipo {
            // metal templado: armas y protección
            SpecialRoomType::Armory => vec![
                Entity {
                    pos: Point::new(0, 0),
                    glyph: '/',
                    color: crate::theme::AZUL_ALMA,
                    name: format!("Espada de la Armería +{}", bonus),
                    e_type: EntityType::Weapon {
                        min_dmg: 4 + bonus,
                        max_dmg: 9 + bonus,
                    },
                    status_effects: Vec::new(),
                },
                Entity {
                    pos: Point::new(0, 0),
                    glyph: '[',
                    color: crate::theme::HUESO,
                    name: "Coraza de la Armería".into(),
                    e_type: EntityType::Armor { defense: 4 + bonus },
                    status_effects: Vec::new(),
                },
            ],
            // pergaminos arcanos en los estantes
            SpecialRoomType::Library => (0..3)
                .map(|_| {
                    let (tipo, nombre) = match rng.gen_range(0..4) {
                        0 => (ScrollType::Lightning, "Pergamino de Rayo"),
                        1 => (ScrollType::Fireball, "Pergamino de Bola de Fuego"),
                        2 => (ScrollType::Teleport, "Pergamino de Teletransporte"),
                        _ => (ScrollType::Invisibility, "Pergamino de Invisibilidad"),
                    };
                    Entity {
                        pos: Point::new(0, 0),
                        glyph: '?',
                        color: crate::theme::VIOLETA,
                        name: nombre.into(),
                        e_type: EntityType::Scroll { scroll_type: tipo },
                        status_effects: Vec::new(),
                    }
                })
                .collect(),
            // energía oscura: lo que da, lo cobra
            SpecialRoomType::RitualCircle => vec![
                Entity {
                    pos: Point::new(0, 0),
                    glyph: '"',
                    color: crate::theme::VIOLETA,
                    name: "Amuleto del Círculo".into(),
                    e_type: EntityType::Amulet { sanity_bonus: 40 },
                    status_effects: Vec::new(),
                },
                Entity {
                    pos: Point::new(0, 0),
                    glyph: '&',
                    color: crate::theme::AMBAR,
                    name: "Fuego Ritual".into(),
                    e_type: EntityType::Hazard {
                        hazard_type: HazardType::Fire,
                    },
                    status_effects: Vec::new(),
                },
            ],
        }
    }

    /// Elige una criatura del catálogo por peso y la escala a la profundidad.
    ///
    /// Las estadísticas salen de `bestiary::BESTIARIO`: son las mismas que
    /// muestra el compendio, no una copia que puede quedar desfasada.
    fn spawn_random_enemy(rng: &mut ChaCha8Rng, pos: Point, depth: u32) -> Entity {
        // El pool depende del tramo: la Rata no llega al Abismo y el Heraldo
        // no sube a las Criptas.
        let t = tramo::indice_de_piso(depth);
        let candidatas: Vec<&bestiary::BestiaryEntry> = bestiary::BESTIARIO
            .iter()
            .filter(|e| e.spawn_weight[t] > 0)
            .collect();

        let peso_total: i32 = candidatas.iter().map(|e| e.spawn_weight[t]).sum();
        let mut tirada = rng.gen_range(0..peso_total);

        let mut elegida = candidatas[0];
        for criatura in candidatas.iter() {
            if tirada < criatura.spawn_weight[t] {
                elegida = criatura;
                break;
            }
            tirada -= criatura.spawn_weight[t];
        }

        let dificultad = (depth as i32 - 1) * 2;

        Entity {
            pos,
            glyph: elegida.glyph,
            color: elegida.color,
            name: elegida.short_name.to_string(),
            e_type: EntityType::Mob {
                hp: elegida.base_hp + dificultad,
                max_hp: elegida.base_hp + dificultad,
                state: if elegida.ai == EnemyAI::Wandering {
                    EnemyState::Wandering
                } else {
                    EnemyState::Asleep
                },
                ai: elegida.ai.clone(),
                min_dmg: elegida.base_damage.0 + (dificultad / 4),
                max_dmg: elegida.base_damage.1 + (dificultad / 4),
                defense: elegida.base_defense + (dificultad / 6),
                pacified: false,
            },
            status_effects: Vec::new(),
        }
    }
}
