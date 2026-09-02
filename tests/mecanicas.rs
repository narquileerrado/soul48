//! Escenarios de varios turnos sobre una sala controlada.
//!
//! Cada test arma su propio escenario con `App::arena`, así ninguno depende
//! de lo que `MapBuilder` haya generado para una semilla determinada.

use soul48::app::{
    App, EnemyAI, EnemyState, Entity, EntityType, HazardType, LogType, Point, ScrollType,
    StatusEffect, StatusEffectType,
};
use soul48::map_builder::MapBuilder;
use soul48::theme;

/// Un mob de prueba con estadísticas explícitas.
fn mob(pos: Point, nombre: &str, hp: i32, dmg: (i32, i32), defense: i32, ai: EnemyAI) -> Entity {
    Entity {
        pos,
        glyph: 'g',
        color: theme::ROJO_ALTAR,
        name: nombre.to_string(),
        e_type: EntityType::Mob {
            hp,
            max_hp: hp,
            state: EnemyState::Aggressive,
            ai,
            min_dmg: dmg.0,
            max_dmg: dmg.1,
            defense,
            pacified: false,
        },
        status_effects: Vec::new(),
    }
}

fn objeto(pos: Point, nombre: &str, e_type: EntityType) -> Entity {
    Entity {
        pos,
        glyph: '?',
        color: theme::HUESO,
        name: nombre.to_string(),
        e_type,
        status_effects: Vec::new(),
    }
}

fn hp_de(app: &App, nombre: &str) -> Option<i32> {
    app.entities
        .iter()
        .find(|e| e.name == nombre)
        .and_then(|e| {
            if let EntityType::Mob { hp, .. } = e.e_type {
                Some(hp)
            } else {
                None
            }
        })
}

/// Un mob que baja a 0 por daño de pergamino tiene que morir, desaparecer del
/// mapa y dar experiencia, igual que si lo hubieras matado cuerpo a cuerpo.
#[test]
fn pergamino_de_rayo_mata_y_da_xp() {
    let mut app = App::arena(7);
    let objetivo = Point::new(app.player.pos.x + 2, app.player.pos.y);
    app.entities
        .push(mob(objetivo, "Gnoll", 10, (4, 6), 1, EnemyAI::Melee));

    app.inventory.push((
        objeto(
            Point::new(0, 0),
            "Pergamino de Rayo",
            EntityType::Scroll {
                scroll_type: ScrollType::Lightning,
            },
        ),
        1,
    ));

    let xp_antes = app.player.xp;
    assert!(app.use_item(0));

    assert!(
        hp_de(&app, "Gnoll").is_none(),
        "el Gnoll sobrevivió al rayo con {:?} de vida",
        hp_de(&app, "Gnoll")
    );
    assert!(
        app.player.xp > xp_antes,
        "matar con un pergamino no dio experiencia"
    );
}

/// Lo mismo para la embestida contra la pared: es daño, y el daño mata.
#[test]
fn embestida_contra_la_pared_puede_matar() {
    let mut app = App::arena(7);
    // pegado al muro este de la arena, sin casilla libre detrás
    app.player.pos = Point::new(app.map[0].len() - 3, 5);
    let objetivo = Point::new(app.map[0].len() - 2, 5);
    app.entities
        .push(mob(objetivo, "Murciélago", 3, (1, 2), 0, EnemyAI::Melee));

    assert!(app.use_pushback());
    assert!(
        hp_de(&app, "Murciélago").is_none(),
        "el Murciélago quedó vivo con {:?} de vida tras la embestida",
        hp_de(&app, "Murciélago")
    );
}

/// La armadura y el casco tienen que restar daño recibido.
#[test]
fn la_armadura_reduce_el_dano_recibido() {
    let escenario = |con_armadura: bool| -> i32 {
        let mut app = App::arena(11);
        app.player.stats.agility = 0; // sin esquiva, para aislar el efecto de la defensa
        if con_armadura {
            app.player.equipment.armor = Some(("Cota de Malla".to_string(), 4));
        }
        let al_lado = Point::new(app.player.pos.x + 1, app.player.pos.y);
        app.entities
            .push(mob(al_lado, "Gnoll", 100, (10, 10), 0, EnemyAI::Stationary));

        let hp_antes = app.player.hp;
        app.process_enemy_turns();
        hp_antes - app.player.hp
    };

    let sin = escenario(false);
    let con = escenario(true);
    assert!(sin > 0, "el enemigo adyacente no llegó a pegar");
    assert!(
        con < sin,
        "con armadura recibiste {} de daño y sin armadura {}: la armadura no hace nada",
        con,
        sin
    );
}

/// El amuleto sube la cordura máxima mientras está puesto.
#[test]
fn el_amuleto_sube_la_cordura_maxima() {
    let mut app = App::arena(3);
    let base = app.player.max_sanity_total();
    app.player.equipment.amulet = Some(("Amuleto de Claridad".to_string(), 20));
    assert_eq!(app.player.max_sanity_total(), base + 20);
}

/// Bajar de piso no puede borrar el nivel, los atributos ni el equipo.
#[test]
fn el_descenso_conserva_la_progresion() {
    let mut app = App::arena(5);
    app.add_xp(200); // fuerza varias subidas de nivel
    app.player.equipment.armor = Some(("Cota de Malla".to_string(), 4));
    app.player.equipment.amulet = Some(("Amuleto de Claridad".to_string(), 20));

    let (nivel, max_hp, fuerza) = (
        app.player.level,
        app.player.max_hp,
        app.player.stats.strength,
    );
    assert!(nivel > 1, "el escenario no llegó a subir de nivel");

    app.descend();
    let abajo = &app;

    assert_eq!(abajo.depth, 2);
    assert_eq!(abajo.player.level, nivel, "se perdió el nivel al descender");
    assert_eq!(abajo.player.max_hp, max_hp, "se perdió la vida máxima");
    assert_eq!(
        abajo.player.stats.strength, fuerza,
        "se perdieron los atributos"
    );
    assert_eq!(
        abajo.player.equipment.armor,
        Some(("Cota de Malla".to_string(), 4)),
        "se perdió la armadura equipada"
    );
    assert!(
        abajo.player.hp <= abajo.player.max_hp,
        "quedaste con {}/{} de vida",
        abajo.player.hp,
        abajo.player.max_hp
    );
    assert_eq!(
        abajo.map[abajo.player.pos.y][abajo.player.pos.x], '.',
        "apareciste dentro de un muro del piso nuevo"
    );
}

/// Un enemigo dormido a distancia despierta, se acerca y termina pegando.
#[test]
fn el_enemigo_despierta_se_acerca_y_ataca() {
    let mut app = App::arena(13);
    app.player.stats.agility = 0;
    let lejos = Point::new(app.player.pos.x + 3, app.player.pos.y);
    let mut dormido = mob(lejos, "Serpiente", 40, (3, 3), 0, EnemyAI::Melee);
    if let EntityType::Mob { ref mut state, .. } = dormido.e_type {
        *state = EnemyState::Asleep;
    }
    app.entities.push(dormido);

    let hp_inicial = app.player.hp;
    let mut distancias = Vec::new();
    for _ in 0..4 {
        app.process_enemy_turns();
        let p = app.entities[0].pos;
        distancias.push(
            (p.x as isize - app.player.pos.x as isize).abs()
                + (p.y as isize - app.player.pos.y as isize).abs(),
        );
    }

    assert!(
        distancias.last() < distancias.first(),
        "la serpiente nunca se acercó: {:?}",
        distancias
    );
    assert!(
        app.player.hp < hp_inicial,
        "la serpiente llegó al cuerpo a cuerpo y nunca pegó"
    );
}

/// Misma semilla y misma secuencia de acciones: mismo resultado.
#[test]
fn la_semilla_es_determinista() {
    let partida = || -> (i32, u32, usize, Point) {
        let mut app = App::new(Some(4242), None, None, 1, None);
        app.start_new_game();
        for _ in 0..30 {
            app.try_move(1, 0);
            app.try_move(0, 1);
            app.process_enemy_turns();
        }
        (
            app.player.hp,
            app.player.xp,
            app.entities.len(),
            app.player.pos,
        )
    };
    assert_eq!(partida(), partida(), "la partida no es reproducible");
}

/// Ninguna entidad puede quedar tapada por otra ni encima de la escalera:
/// `try_move` sólo ve la primera de la pila.
#[test]
fn la_generacion_no_apila_entidades() {
    for seed in 0..25u64 {
        for depth in [1u32, 5, 48] {
            let construido = MapBuilder::new(seed, depth);
            let mut vistas = std::collections::HashSet::new();
            for e in &construido.entities {
                assert!(
                    vistas.insert((e.pos.x, e.pos.y)),
                    "semilla {} piso {}: dos entidades en {:?} (la segunda es {})",
                    seed,
                    depth,
                    e.pos,
                    e.name
                );
                assert_ne!(
                    construido.map[e.pos.y][e.pos.x], '>',
                    "semilla {} piso {}: {} tapa la escalera",
                    seed, depth, e.name
                );
                assert_eq!(
                    construido.map[e.pos.y][e.pos.x], '.',
                    "semilla {} piso {}: {} quedó dentro de un muro",
                    seed, depth, e.name
                );
            }
        }
    }
}

/// El historial guarda más de lo que la pantalla muestra: el ajuste de
/// líneas visibles no puede borrar datos.
#[test]
fn el_ajuste_de_pantalla_no_borra_el_historial() {
    let mut app = App::arena(2);
    app.settings.lineas_susurro = 3;
    for i in 0..20 {
        app.add_log(format!("mensaje {}", i), LogType::Info);
    }
    assert!(
        app.logs.len() > 3,
        "el historial quedó recortado a {} mensajes por un ajuste de pantalla",
        app.logs.len()
    );
}

/// Desequipar no puede crear slots por encima del tope alcanzable con 1-9.
#[test]
fn desequipar_respeta_el_tope_del_inventario() {
    let mut app = App::arena(17);
    for i in 0..9 {
        app.inventory.push((
            objeto(
                Point::new(0, 0),
                &format!("Baratija {}", i),
                EntityType::Item,
            ),
            1,
        ));
    }
    app.player.equipment.armor = Some(("Cota vieja".to_string(), 2));
    // la nueva armadura entra desde el suelo, no desde el inventario lleno
    app.inventory[8] = (
        objeto(
            Point::new(0, 0),
            "Cota nueva",
            EntityType::Armor { defense: 5 },
        ),
        1,
    );

    app.use_item(8);
    assert!(
        app.inventory.len() <= 9,
        "el inventario quedó con {} objetos: los que pasan de 9 son inalcanzables",
        app.inventory.len()
    );
}

/// Los efectos de estado se consumen y se van.
#[test]
fn los_efectos_de_estado_expiran() {
    let mut app = App::arena(19);
    app.player.status_effects.push(StatusEffect {
        effect_type: StatusEffectType::Poison,
        duration: 2,
        damage_per_turn: 2,
    });
    let hp = app.player.hp;
    app.process_enemy_turns();
    app.process_enemy_turns();
    assert!(app.player.status_effects.is_empty(), "el veneno no expiró");
    assert_eq!(hp - app.player.hp, 4, "el veneno no hizo su daño por turno");
}

/// Una puerta cerrada corta la visión; abierta, deja pasar la mirada.
#[test]
fn la_puerta_cerrada_bloquea_la_vision() {
    let mut app = App::arena(23);
    let puerta = Point::new(app.player.pos.x + 1, app.player.pos.y);
    let detras = Point::new(app.player.pos.x + 2, app.player.pos.y);

    app.entities.push(objeto(
        puerta,
        "Puerta de Madera",
        EntityType::Door {
            locked: false,
            secret: false,
            open: false,
        },
    ));
    app.calculate_fov();
    assert!(
        !app.visible[detras.y][detras.x],
        "se ve a través de una puerta cerrada"
    );

    if let EntityType::Door { ref mut open, .. } = app.entities[0].e_type {
        *open = true;
    }
    app.calculate_fov();
    assert!(
        app.visible[detras.y][detras.x],
        "la puerta abierta sigue tapando la vista"
    );
}

/// El compendio y lo que aparece en el mapa tienen que decir lo mismo.
#[test]
fn el_compendio_no_le_miente_al_jugador() {
    use soul48::bestiary::BESTIARIO;

    // en el piso 1 no hay bonus de dificultad: los números salen tal cual
    let mut vistas = std::collections::HashMap::new();
    for seed in 0..60u64 {
        for e in MapBuilder::new(seed, 1).entities {
            if let EntityType::Mob {
                max_hp,
                min_dmg,
                max_dmg,
                defense,
                ..
            } = e.e_type
            {
                vistas.insert(e.name.clone(), (max_hp, min_dmg, max_dmg, defense, e.glyph));
            }
        }
    }
    assert!(!vistas.is_empty(), "no apareció ninguna criatura");

    for (nombre, (hp, min_d, max_d, def, glifo)) in vistas {
        let Some(ficha) = BESTIARIO.iter().find(|b| b.short_name == nombre) else {
            continue; // los jefes no están en el compendio
        };
        assert_eq!(
            ficha.base_hp, hp,
            "{}: vida distinta a la del compendio",
            nombre
        );
        assert_eq!(
            ficha.base_damage,
            (min_d, max_d),
            "{}: daño distinto al del compendio",
            nombre
        );
        assert_eq!(
            ficha.base_defense, def,
            "{}: defensa distinta a la del compendio",
            nombre
        );
        assert_eq!(ficha.glyph, glifo, "{}: glifo distinto", nombre);
    }
}

/// Una partida larga: cientos de turnos, varios pisos, sin romper invariantes.
///
/// No comprueba una mecánica puntual sino que nada se desmadre: vida y cordura
/// dentro de sus topes, el héroe siempre sobre suelo transitable, el inventario
/// dentro de las nueve ranuras y ningún mob vivo con vida negativa.
#[test]
fn una_partida_larga_mantiene_los_invariantes() {
    for seed in [1u64, 77, 2024] {
        let mut app = App::new(Some(seed), None, None, 1, None);
        app.start_new_game();

        let pasos = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for turno in 0..600 {
            let (dx, dy) = pasos[turno % pasos.len()];
            app.try_move(dx, dy);

            if turno % 17 == 0 && !app.inventory.is_empty() {
                app.use_item(turno % app.inventory.len());
            }
            if turno % 23 == 0 {
                app.use_pushback();
            }
            if app.show_descend_prompt {
                app.confirm_descent();
                app.descend();
            }
            app.process_enemy_turns();
            app.calculate_fov();

            assert!(
                app.player.hp <= app.player.max_hp,
                "semilla {}: vida {}/{}",
                seed,
                app.player.hp,
                app.player.max_hp
            );
            assert!(
                app.player.sanity <= app.player.max_sanity_total(),
                "semilla {}: cordura por encima de su techo",
                seed
            );
            assert!(
                app.inventory.len() <= 9,
                "semilla {}: {} objetos en el inventario",
                seed,
                app.inventory.len()
            );
            assert!(
                app.es_transitable(app.player.pos),
                "semilla {}: el héroe terminó dentro de un muro",
                seed
            );
            for e in &app.entities {
                if let EntityType::Mob { hp, .. } = e.e_type {
                    assert!(
                        hp > 0,
                        "semilla {}: {} sigue vivo con {} de vida",
                        seed,
                        e.name,
                        hp
                    );
                }
            }

            if app.player.hp <= 0 {
                break;
            }
        }
    }
}

/// Matar al Archidemonio termina la corrida, y la termina bien.
#[test]
fn matar_al_archidemonio_gana_la_partida() {
    use soul48::app::GameState;
    use soul48::bestiary::ARCHIDEMONIO;

    let mut app = App::arena(31);
    let al_lado = Point::new(app.player.pos.x + 1, app.player.pos.y);
    // con 1 de vida cae de un golpe, sea cual sea la tirada del arma
    app.entities
        .push(mob(al_lado, ARCHIDEMONIO, 1, (8, 16), 0, EnemyAI::Melee));

    assert_eq!(app.state, GameState::Playing);
    app.try_move(1, 0);

    assert_eq!(
        app.state,
        GameState::Victory,
        "el Archidemonio cayó y la partida siguió como si nada"
    );
    assert!(
        !app.entities.iter().any(|e| e.name == ARCHIDEMONIO),
        "el Archidemonio sigue en el mapa"
    );
}

/// El jefe final también cae por magia, y también termina la corrida.
#[test]
fn el_archidemonio_tambien_cae_por_pergamino() {
    use soul48::app::GameState;
    use soul48::bestiary::ARCHIDEMONIO;

    let mut app = App::arena(32);
    let cerca = Point::new(app.player.pos.x + 2, app.player.pos.y);
    app.entities
        .push(mob(cerca, ARCHIDEMONIO, 5, (8, 16), 0, EnemyAI::Melee));
    app.inventory.push((
        objeto(
            Point::new(0, 0),
            "Pergamino de Rayo",
            EntityType::Scroll {
                scroll_type: ScrollType::Lightning,
            },
        ),
        1,
    ));

    app.use_item(0);
    assert_eq!(app.state, GameState::Victory);
}

/// El descenso deja un punto de control cargable, y morir lo disuelve.
#[test]
fn el_fragmento_se_guarda_al_bajar_y_se_disuelve_al_morir() {
    let ruta = "target/test_permadeath.json";
    App::borrar_save(ruta);

    let mut app = App::new(Some(41), None, None, 1, None);
    app.start_new_game();
    app.descend();
    app.save_to_file(ruta).expect("no se pudo guardar");

    let (piso, _, _, _) = App::peek_save(ruta).expect("el punto de control no quedó");
    assert_eq!(piso, 2, "el punto de control no guardó el piso nuevo");
    let recuperada = App::load_from_file(ruta).expect("no se pudo recargar");
    assert_eq!(recuperada.depth, 2);

    App::borrar_save(ruta);
    assert!(
        App::peek_save(ruta).is_none(),
        "el fragmento sobrevivió a la muerte"
    );
}

/// Borrar dos veces no es un error: que no exista es el caso normal.
#[test]
fn borrar_un_fragmento_que_no_esta_no_rompe() {
    let ruta = "target/test_inexistente.json";
    App::borrar_save(ruta);
    App::borrar_save(ruta);
    assert!(App::peek_save(ruta).is_none());
}

/// Cada tramo genera criaturas de su pool y ninguna de otro.
#[test]
fn cada_tramo_puebla_con_su_propio_pool() {
    use soul48::bestiary::BESTIARIO;
    use soul48::world::tramo;

    // un piso representativo de cada tramo
    for piso in [3u32, 15, 28, 40] {
        let t = tramo::indice_de_piso(piso);
        let mut vistas = std::collections::HashSet::new();
        for seed in 0..40u64 {
            for e in MapBuilder::new(seed, piso).entities {
                if matches!(e.e_type, EntityType::Mob { .. }) {
                    vistas.insert(e.name.clone());
                }
            }
        }
        assert!(!vistas.is_empty(), "el piso {} salió desierto", piso);

        for nombre in &vistas {
            let Some(ficha) = BESTIARIO.iter().find(|b| b.short_name == nombre) else {
                continue; // los jefes no salen del pool
            };
            assert!(
                ficha.spawn_weight[t] > 0,
                "«{}» apareció en el piso {}, donde su peso es 0",
                nombre,
                piso
            );
        }
    }
}

/// Un enemigo envenenado se muere solo, y da su experiencia igual.
#[test]
fn un_mob_envenenado_muere_solo() {
    let mut app = App::arena(37);
    let lejos = Point::new(app.player.pos.x + 5, app.player.pos.y);
    let mut victima = mob(lejos, "Gnoll", 4, (1, 1), 0, EnemyAI::Stationary);
    victima.status_effects.push(StatusEffect {
        effect_type: StatusEffectType::Poison,
        duration: 5,
        damage_per_turn: 2,
    });
    app.entities.push(victima);

    let xp_antes = app.player.xp;
    app.process_enemy_turns();
    app.process_enemy_turns();

    assert!(
        !app.entities.iter().any(|e| e.name == "Gnoll"),
        "el veneno no llegó a matarlo"
    );
    assert!(
        app.player.xp > xp_antes,
        "morir de veneno no dio experiencia"
    );
}

/// La ceguera achica el mundo a lo que tenés encima.
#[test]
fn la_ceguera_recorta_el_campo_de_vision() {
    let ver_todo = |cegado: bool| -> usize {
        let mut app = App::arena(43);
        if cegado {
            app.player.status_effects.push(StatusEffect {
                effect_type: StatusEffectType::Blindness,
                duration: 4,
                damage_per_turn: 0,
            });
        }
        app.calculate_fov();
        app.visible.iter().flatten().filter(|v| **v).count()
    };

    let normal = ver_todo(false);
    let cegado = ver_todo(true);
    assert!(normal > 0);
    assert!(
        cegado < normal,
        "cegado ves {} casillas y normal {}: la ceguera no hace nada",
        cegado,
        normal
    );
}

/// La serpiente envenena al golpear: el efecto sale del catálogo.
#[test]
fn la_serpiente_deja_su_marca() {
    let mut app = App::arena(47);
    app.player.stats.agility = 0; // sin esquiva, el golpe entra
    let al_lado = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.entities.push(mob(
        al_lado,
        "Serpiente",
        100,
        (3, 3),
        0,
        EnemyAI::Stationary,
    ));

    // unos turnos: alcanza para que pegue al menos una vez
    for _ in 0..5 {
        app.process_enemy_turns();
    }
    assert!(
        app.player
            .status_effects
            .iter()
            .any(|e| e.effect_type == StatusEffectType::Poison),
        "la serpiente golpeó y no envenenó"
    );
}

/// Un enemigo detrás de una esquina tiene que doblarla.
///
/// Con el movimiento greedy anterior el mob elegía siempre el paso que más lo
/// acercaba en línea recta, chocaba contra el muro y se quedaba vibrando ahí:
/// pararse detrás de un recodo era invulnerabilidad gratis.
#[test]
fn el_enemigo_dobla_la_esquina() {
    let mut app = App::arena(53);
    let (hx, hy) = (app.player.pos.x, app.player.pos.y);

    // un muro en L entre el héroe y el enemigo: la línea recta está cortada
    for dy in 0..4 {
        app.map[hy + dy][hx + 2] = '#';
    }
    let enemigo = Point::new(hx + 4, hy);
    app.entities
        .push(mob(enemigo, "Gnoll", 100, (1, 1), 0, EnemyAI::Melee));

    // en pasos de rey: los mobs se mueven en ocho direcciones y la adyacencia
    // para pegar cuenta las diagonales
    let dist = |app: &App| -> usize {
        let p = app.entities[0].pos;
        (p.x as isize - app.player.pos.x as isize)
            .abs()
            .max((p.y as isize - app.player.pos.y as isize).abs()) as usize
    };

    let inicial = dist(&app);
    let mut minima = inicial;
    for _ in 0..25 {
        app.process_enemy_turns();
        minima = minima.min(dist(&app));
    }

    assert!(
        minima < inicial,
        "el enemigo nunca rodeó el muro: quedó a {} pasos, igual que al empezar",
        minima
    );
    assert!(
        minima <= 1,
        "el enemigo rodeó a medias y se quedó a {} pasos",
        minima
    );
}

/// El cobarde huye de verdad, no se traba contra la pared de atrás.
#[test]
fn el_cobarde_se_aleja() {
    let mut app = App::arena(59);
    let al_lado = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.entities
        .push(mob(al_lado, "Ladrón", 100, (1, 1), 0, EnemyAI::Coward));

    for _ in 0..6 {
        app.process_enemy_turns();
    }
    let p = app.entities[0].pos;
    let dist = (p.x as isize - app.player.pos.x as isize).abs()
        + (p.y as isize - app.player.pos.y as isize).abs();
    assert!(dist > 1, "el cobarde se quedó pegado al héroe");
}

/// Una puerta cerrada corta el camino igual que un muro.
#[test]
fn el_camino_no_atraviesa_puertas_cerradas() {
    let mut app = App::arena(61);
    let (hx, hy) = (app.player.pos.x, app.player.pos.y);

    // pasillo de una casilla, tapado por una puerta cerrada
    for y in 0..app.map.len() {
        if y != hy {
            app.map[y][hx + 2] = '#';
        }
    }
    app.entities.push(objeto(
        Point::new(hx + 2, hy),
        "Puerta Cerrada con Llave",
        EntityType::Door {
            locked: true,
            secret: false,
            open: false,
        },
    ));

    let campo = app.flow_field();
    assert!(
        campo.distancia(Point::new(hx + 4, hy)).is_none(),
        "el camino atraviesa una puerta cerrada"
    );
}

/// El aceite hace lo que anuncia: te lleva un paso de más.
#[test]
fn el_aceite_te_hace_resbalar() {
    let mut app = App::arena(67);
    let charco = Point::new(app.player.pos.x + 1, app.player.pos.y);
    let mas_alla = Point::new(app.player.pos.x + 2, app.player.pos.y);
    app.entities.push(objeto(
        charco,
        "Charco de Aceite",
        EntityType::Hazard {
            hazard_type: HazardType::Oil,
        },
    ));

    app.try_move(1, 0);
    assert_eq!(
        app.player.pos, mas_alla,
        "el aceite no arrastró: quedaste en {:?}",
        app.player.pos
    );
}

/// Con fuego al lado, el aceite prende en vez de resbalar.
#[test]
fn el_aceite_prende_si_hay_fuego_al_lado() {
    let mut app = App::arena(71);
    let charco = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.entities.push(objeto(
        charco,
        "Charco de Aceite",
        EntityType::Hazard {
            hazard_type: HazardType::Oil,
        },
    ));
    app.entities.push(objeto(
        Point::new(charco.x + 1, charco.y),
        "Fuego",
        EntityType::Hazard {
            hazard_type: HazardType::Fire,
        },
    ));

    let hp = app.player.hp;
    app.try_move(1, 0);
    assert!(app.player.hp < hp, "el aceite no prendió con fuego al lado");
    assert!(
        app.player
            .status_effects
            .iter()
            .any(|e| e.effect_type == StatusEffectType::Burn),
        "prendió y no te quemó"
    );
}

/// El pergamino de teletransporte no se gasta si no encuentra dónde dejarte.
#[test]
fn el_teletransporte_no_se_gasta_al_vacio() {
    let mut app = App::arena(73);
    // un piso sin una sola casilla de suelo: no hay destino posible
    for fila in app.map.iter_mut() {
        for casilla in fila.iter_mut() {
            *casilla = '#';
        }
    }
    app.inventory.push((
        objeto(
            Point::new(0, 0),
            "Pergamino de Teletransporte",
            EntityType::Scroll {
                scroll_type: ScrollType::Teleport,
            },
        ),
        1,
    ));

    app.use_item(0);
    assert_eq!(
        app.inventory.len(),
        1,
        "el pergamino se gastó sin teletransportar a nadie"
    );
}

/// Los jefes tienen ficha, y la ficha dice lo que el jugador se cruza.
#[test]
fn los_jefes_estan_en_el_compendio() {
    use soul48::bestiary::BESTIARIO;
    use soul48::world::tramo::{self, TRAMOS};

    for t in TRAMOS.iter() {
        let ficha = BESTIARIO
            .iter()
            .find(|e| e.short_name == t.jefe)
            .unwrap_or_else(|| panic!("«{}» no tiene ficha en el Compendio", t.jefe));
        assert!(
            ficha.spawn_weight.iter().all(|p| *p == 0),
            "«{}» puede salir por spawn aleatorio",
            t.jefe
        );

        // lo que genera el piso tiene que coincidir con lo que dice la ficha
        let piso = tramo::piso_del_guardian(t);
        let generado = MapBuilder::new(9001, piso)
            .entities
            .into_iter()
            .find(|e| e.name == t.jefe)
            .unwrap_or_else(|| panic!("el piso {} no puso a «{}»", piso, t.jefe));
        if let EntityType::Mob { max_hp, .. } = generado.e_type {
            assert_eq!(
                max_hp, ficha.base_hp,
                "«{}» aparece con {} de vida y el Compendio dice {}",
                t.jefe, max_hp, ficha.base_hp
            );
        }
    }
}

/// Una corrida completa hasta el piso 48, sin romper nada por el camino.
///
/// No juega bien: baja piso por piso hasta el final para verificar que la
/// generación aguanta los 48 pisos, que cada tramo se puebla, que los cuatro
/// Guardianes aparecen donde corresponde y que el Archidemonio espera abajo.
#[test]
fn el_descenso_completo_llega_hasta_el_archidemonio() {
    use soul48::bestiary::ARCHIDEMONIO;
    use soul48::world::tramo::{self, TRAMOS};

    let mut app = App::new(Some(808), None, None, 1, None);
    app.start_new_game();
    // invulnerable: acá se prueba el mundo, no el combate
    app.player.max_hp = 100_000;

    let mut guardianes = Vec::new();
    let mut vio_archidemonio = false;

    while app.depth < 48 {
        app.player.hp = app.player.max_hp;
        let t = tramo::de_piso(app.depth);

        assert!(
            app.entities
                .iter()
                .any(|e| matches!(e.e_type, soul48::app::EntityType::Mob { .. })),
            "el piso {} («{}») salió sin una sola criatura",
            app.depth,
            t.nombre
        );
        assert!(
            app.es_transitable(app.player.pos),
            "el piso {} te dejó dentro de un muro",
            app.depth
        );

        for e in &app.entities {
            if e.name == ARCHIDEMONIO {
                vio_archidemonio = true;
            }
            if TRAMOS.iter().any(|t| t.jefe == e.name) {
                guardianes.push((app.depth, e.name.clone()));
            }
        }
        app.descend();
    }

    // el último piso: acá espera el final
    assert!(
        app.entities.iter().any(|e| e.name == ARCHIDEMONIO),
        "el piso 48 no tiene Archidemonio"
    );
    assert!(
        !vio_archidemonio,
        "el Archidemonio apareció antes de tiempo"
    );

    assert_eq!(
        guardianes.len(),
        TRAMOS.len(),
        "faltan Guardianes en el descenso: {:?}",
        guardianes
    );
    for (piso, nombre) in &guardianes {
        let t = tramo::de_piso(*piso);
        assert_eq!(
            &t.jefe, nombre,
            "en el piso {} apareció «{}», que no es el Guardián de «{}»",
            piso, nombre, t.nombre
        );
    }
}
