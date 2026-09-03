//! Tests de las mecánicas básicas, heredados de cuando vivían dentro de
//! `app.rs`. Los escenarios de varios turnos están en `mecanicas.rs`.

use ratatui::style::Color;

/// El nombre de la criatura que se puede calmar en vez de pelear. Sale del
/// catálogo para que renombrarla no rompa el test en silencio.
fn negociable() -> String {
    soul48::bestiary::BESTIARIO
        .iter()
        .find(|e| e.negociable)
        .expect("ninguna criatura es negociable")
        .short_name
        .to_string()
}
use soul48::app::*;

#[test]
fn test_app_initialization() {
    let app = App::new(Some(12345), None, None, 1, None);
    assert_eq!(app.seed, 12345);
    assert_eq!(app.depth, 1);
    assert_eq!(app.player.hp, 20);
    assert_eq!(app.state, GameState::TitleScreen);
}

#[test]
fn test_start_new_game() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.start_new_game();
    assert_eq!(app.state, GameState::Playing);
    assert!(app.visible[app.player.pos.y][app.player.pos.x]);
}

#[test]
fn test_try_move_invalid() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.start_new_game();
    // Trying to move out of bounds (negative offset when at 0,0 if hero was at 0,0) or against a wall
    // Set hero position surrounded by walls
    app.player.pos = Point::new(0, 0);
    app.map[0][0] = '.';
    app.map[0][1] = '#';
    app.map[1][0] = '#';

    assert!(!app.try_move(-1, 0)); // Out of bounds negative
    assert!(!app.try_move(1, 0)); // Hit wall '#'
}

#[test]
fn test_save_and_load_file() {
    let test_file = "test_savegame.json";
    let mut app = App::new(Some(54321), None, None, 1, None);
    app.start_new_game();
    app.player.hp = 12;

    app.save_to_file(test_file).expect("Failed to save game");

    let loaded_app = App::load_from_file(test_file).expect("Failed to load game");
    assert_eq!(loaded_app.seed, 54321);
    assert_eq!(loaded_app.player.hp, 12);
    assert_eq!(loaded_app.state, GameState::Playing);

    let _ = std::fs::remove_file(test_file);
}

#[test]
fn test_use_item_healing_potion() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.player.hp = 5;
    let potion = Entity {
        pos: Point::new(0, 0),
        glyph: '!',
        color: soul48::theme::HUESO,
        name: soul48::bestiary::POCION.to_string(),
        e_type: EntityType::Item,
        status_effects: Vec::new(),
    };
    app.inventory.push((potion, 1));

    assert!(app.use_item(0));
    assert_eq!(app.player.hp, 20); // Healed by 15 up to max 20
    assert!(app.inventory.is_empty());
}

#[test]
fn test_drop_item() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    let initial_entities_count = app.entities.len();
    let potion = Entity {
        pos: Point::new(0, 0),
        glyph: '!',
        color: soul48::theme::HUESO,
        name: soul48::bestiary::POCION.to_string(),
        e_type: EntityType::Item,
        status_effects: Vec::new(),
    };
    app.inventory.push((potion, 1));

    assert!(app.drop_item(0));
    assert!(app.inventory.is_empty());
    assert_eq!(app.entities.len(), initial_entities_count + 1);
    assert_eq!(app.entities.last().unwrap().pos, app.player.pos);
}

#[test]
fn test_talking_wall_interaction() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.start_new_game();
    let wall_pos = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.map[wall_pos.y][wall_pos.x] = '.';
    app.entities.push(Entity {
        pos: wall_pos,
        glyph: 'W',
        color: soul48::theme::HUESO,
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
    assert_ne!(app.player.pos, wall_pos); // Cannot step into wall
    assert!(app
        .logs
        .iter()
        .any(|log| log.text.contains("Un secreto te aguarda.")));
}

#[test]
fn test_echo_altar_interaction() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.start_new_game();
    let altar_pos = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.map[altar_pos.y][altar_pos.x] = '.';
    app.entities.push(Entity {
        pos: altar_pos,
        glyph: 'A',
        color: Color::Red,
        name: "Altar de Ecos".to_string(),
        e_type: EntityType::EchoAltar { used: false },
        status_effects: Vec::new(),
    });

    let initial_hp = app.player.hp;
    let moved = app.try_move(1, 0);
    assert!(moved);
    assert_eq!(app.player.hp, initial_hp - 5); // 5 HP traded
    assert!(app.explored[0][0]); // Map revealed
}

#[test]
fn test_spirit_negotiation() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.start_new_game();
    let thief_pos = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.map[thief_pos.y][thief_pos.x] = '.';
    app.entities.push(Entity {
        pos: thief_pos,
        glyph: 'L',
        color: Color::Blue,
        name: negociable(),
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

    let initial_sanity = app.player.sanity;
    let action = app.try_move(1, 0);
    assert!(action);
    assert_eq!(app.player.sanity, initial_sanity - 10);
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
    assert!(app.player.parry_active);

    // Test Pushback with adjacent mob
    let target_pos = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.map[target_pos.y][target_pos.x] = '.';
    let empty_pos = Point::new(app.player.pos.x + 2, app.player.pos.y);
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
        e_type: EntityType::Scroll {
            scroll_type: ScrollType::Lightning,
        },
        status_effects: Vec::new(),
    };
    app.inventory.push((scroll, 1));

    let mob_pos = Point::new(app.player.pos.x + 1, app.player.pos.y);
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

    let door_pos = Point::new(app.player.pos.x + 1, app.player.pos.y);
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
    assert_eq!(app.player.pos, door_pos);
}

#[test]
fn test_hazard_interaction() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.start_new_game();

    let hazard_pos = Point::new(app.player.pos.x + 1, app.player.pos.y);
    app.map[hazard_pos.y][hazard_pos.x] = '.';
    app.entities.push(Entity {
        pos: hazard_pos,
        glyph: '^',
        color: Color::DarkGray,
        name: "Trampa de Pinchos".to_string(),
        e_type: EntityType::Hazard {
            hazard_type: HazardType::Spikes,
        },
        status_effects: Vec::new(),
    });

    let initial_hp = app.player.hp;
    let action = app.try_move(1, 0);
    assert!(action);
    assert_eq!(app.player.pos, hazard_pos);
    assert_eq!(app.player.hp, initial_hp - 4);
}

#[test]
fn test_add_xp_and_level_up() {
    let mut app = App::new(Some(12345), None, None, 1, None);
    app.start_new_game();

    assert_eq!(app.player.level, 1);
    app.add_xp(60); // 60 >= 50 (next_level_xp)
    assert_eq!(app.player.level, 2);
    assert_eq!(app.player.stats.strength, 6);
    assert_eq!(app.player.stats.agility, 6);
    assert_eq!(app.player.stats.willpower, 6);
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
    assert_eq!(
        app.player.equipment.armor,
        Some(("Cota de Malla".to_string(), 4))
    );
}

#[test]
fn test_floor_48_boss_encounter() {
    let app = App::new(Some(12345), None, None, 48, None);
    assert_eq!(app.depth, 48);
    let boss_exists = app
        .entities
        .iter()
        .any(|e| e.name == "ARCHIDEMONIO DEL SILENCIO");
    assert!(boss_exists);
}

/* ───────────────────────────── los cuatro tramos ───────────────────────────── */

/// Los 48 pisos tienen que estar cubiertos, sin huecos ni solapamientos.
#[test]
fn los_tramos_cubren_el_descenso_entero() {
    use soul48::world::tramo::{self, TRAMOS};

    for piso in 1..=48u32 {
        let t = tramo::de_piso(piso);
        assert!(
            piso >= t.rango.0 && piso <= t.rango.1,
            "el piso {} cayó en «{}», que va del {} al {}",
            piso,
            t.nombre,
            t.rango.0,
            t.rango.1
        );
    }

    // sin huecos ni solapamientos entre tramos consecutivos
    assert_eq!(TRAMOS[0].rango.0, 1, "el descenso no empieza en el piso 1");
    assert_eq!(
        TRAMOS[TRAMOS.len() - 1].rango.1,
        48,
        "el descenso no termina en el piso 48"
    );
    for par in TRAMOS.windows(2) {
        assert_eq!(
            par[1].rango.0,
            par[0].rango.1 + 1,
            "«{}» y «{}» no se tocan",
            par[0].nombre,
            par[1].nombre
        );
    }
}

/// Un tramo sin criaturas con peso deja pisos desiertos.
#[test]
fn cada_tramo_tiene_con_que_poblarse() {
    use soul48::bestiary::BESTIARIO;
    use soul48::world::tramo::TRAMOS;

    for (i, t) in TRAMOS.iter().enumerate() {
        let pool: Vec<&str> = BESTIARIO
            .iter()
            .filter(|e| e.spawn_weight[i] > 0)
            .map(|e| e.short_name)
            .collect();
        assert!(
            pool.len() >= 3,
            "«{}» sólo tiene {:?} para poblarse",
            t.nombre,
            pool
        );
    }
}

/// Cada tramo tiene su paleta y sus voces: si dos comparten todo, no hay tramos.
#[test]
fn cada_tramo_tiene_identidad_propia() {
    use soul48::world::tramo::TRAMOS;

    for t in TRAMOS.iter() {
        assert!(!t.susurros.is_empty(), "«{}» no tiene voces", t.nombre);
        assert!(!t.jefe.is_empty(), "«{}» no tiene Guardián", t.nombre);
        assert!(!t.entrada.is_empty(), "«{}» no se presenta", t.nombre);
    }
    for par in TRAMOS.windows(2) {
        assert_ne!(
            par[0].muro, par[1].muro,
            "«{}» y «{}» pintan los muros igual",
            par[0].nombre, par[1].nombre
        );
        assert_ne!(par[0].jefe, par[1].jefe);
    }
}

/// El último piso de cada tramo lleva su Guardián nombrado.
#[test]
fn los_guardianes_cierran_su_tramo() {
    use soul48::world::tramo;

    for piso in 1..=48u32 {
        assert_eq!(
            tramo::cierra_tramo(piso),
            // el 48 es del Archidemonio: el Guardián del Silencio cierra en el 42
            matches!(piso, 12 | 24 | 36 | 42),
            "el piso {} no coincide con el fin de tramo",
            piso
        );
    }
}

/* ───────────────────────────── los retratos ───────────────────────────── */

/// Toda criatura del Compendio tiene que tener su retrato.
///
/// `arte::de_criatura` devuelve `Option` y el Compendio cae al formato sin
/// retrato, así que una ficha sin dibujo no rompe nada: se degrada en silencio.
#[test]
fn ninguna_criatura_se_queda_sin_retrato() {
    use soul48::arte;
    use soul48::bestiary::BESTIARIO;

    let sin: Vec<&str> = BESTIARIO
        .iter()
        .filter(|e| arte::de_criatura(e.short_name).is_none())
        .map(|e| e.short_name)
        .collect();
    assert!(sin.is_empty(), "sin retrato: {:?}", sin);
}

/// Cada criatura tiene el suyo: los cuatro Guardianes comparten la `B` en el
/// mapa, así que buscarlos por glifo les daba el mismo dibujo a todos.
#[test]
fn cada_criatura_tiene_su_propio_retrato() {
    use soul48::arte;
    use soul48::bestiary::BESTIARIO;

    let mut vistos: Vec<(*const _, &str)> = Vec::new();
    for e in BESTIARIO.iter() {
        let sprite = arte::de_criatura(e.short_name).expect("sin retrato");
        let puntero = sprite as *const _;
        if let Some((_, otro)) = vistos.iter().find(|(p, _)| *p == puntero) {
            panic!("«{}» y «{}» comparten retrato", e.short_name, otro);
        }
        vistos.push((puntero, e.short_name));
    }
}

/// El arte es texto: mientras las filas midan lo mismo, cualquier dibujo entra.
/// Una fila corta deja un agujero transparente que nadie ve hasta abrir la ficha.
#[test]
fn los_retratos_son_rectangulares() {
    use soul48::arte;
    use soul48::bestiary::BESTIARIO;

    for e in BESTIARIO.iter() {
        let sprite = arte::de_criatura(e.short_name).expect("sin retrato");
        let anchos: std::collections::HashSet<usize> =
            sprite.arte.iter().map(|f| f.chars().count()).collect();
        assert_eq!(
            anchos.len(),
            1,
            "el retrato de «{}» tiene filas de distinto largo: {:?}",
            e.short_name,
            anchos
        );
    }
}

/// Una partida recién hecha no puede depender del `settings.json` de nadie.
///
/// `App::new` leía los ajustes del disco, con que los tests pasaban o fallaban
/// según lo que tuviera configurado quien los corriera: un `settings.json` con
/// GLIFOS en ascii hacía fallar el render del título sin que nada lo explicara.
#[test]
fn una_partida_nueva_no_lee_los_ajustes_del_disco() {
    use soul48::settings::{Glifos, Settings};

    let app = App::new(Some(1), None, None, 1, None);
    let por_defecto = Settings::default();
    assert_eq!(app.settings.glifos, Glifos::Unicode);
    assert_eq!(app.settings.penumbra, por_defecto.penumbra);
    assert_eq!(app.settings.lineas_susurro, por_defecto.lineas_susurro);
}
