//! Render de cada pantalla sobre un backend de prueba.
//!
//! No comprueba estética, sino que cada pantalla se dibuje sin romperse y
//! muestre lo que promete: los paneles de `ui.rs` calculan anchos con
//! `saturating_sub` y restas de `Rect`, así que una terminal chica es la forma
//! más fácil de hacer estallar un índice.

use ratatui::{backend::TestBackend, Terminal};
use soul48::app::{App, GameState};
use soul48::menus::Menus;
use soul48::settings::Glifos;
use soul48::title;
use soul48::ui::{bestiary_ui, game_over_ui, options_ui, ui};

/// Todo el texto del buffer, sin estilos, para poder buscar en él.
fn texto(terminal: &Terminal<TestBackend>) -> String {
    let buffer = terminal.backend().buffer();
    buffer
        .content()
        .iter()
        .map(|c| c.symbol())
        .collect::<Vec<_>>()
        .join("")
}

fn partida() -> App {
    let mut app = App::new(Some(99), None, None, 1, None);
    app.start_new_game();
    app
}

#[test]
fn la_pantalla_de_juego_muestra_sus_paneles() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let app = partida();
    terminal.draw(|f| ui(f, &app)).unwrap();

    let pantalla = texto(&terminal);
    for esperado in [
        "SOUL 48",
        "LAS CRIPTAS",
        "TU ALMA",
        "LO QUE LLEVÁS PUESTO",
        "LO QUE CARGÁS",
        "LO QUE TE RODEA",
        "LO QUE SE DICE",
        "moverte",
    ] {
        assert!(
            pantalla.contains(esperado),
            "la pantalla de juego no muestra «{}»",
            esperado
        );
    }
}

#[test]
fn el_modo_soltar_se_anuncia_en_pantalla() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut app = partida();
    app.drop_mode = true;
    terminal.draw(|f| ui(f, &app)).unwrap();

    let pantalla = texto(&terminal);
    assert!(pantalla.contains("LO QUE SUELTAS"));
    assert!(pantalla.contains("cancelar"));
}

#[test]
fn el_modal_de_descenso_se_dibuja_sobre_el_mapa() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut app = partida();
    app.show_descend_prompt = true;
    terminal.draw(|f| ui(f, &app)).unwrap();

    let pantalla = texto(&terminal);
    assert!(pantalla.contains("LAS ESCALERAS"), "no se ve el modal");
    assert!(pantalla.contains("quedarte"));
}

#[test]
fn las_pantallas_aguantan_una_terminal_chica() {
    // el mínimo razonable: si acá no estalla, no estalla en ningún lado
    for (ancho, alto) in [(80, 24), (60, 20), (40, 15)] {
        let mut terminal = Terminal::new(TestBackend::new(ancho, alto)).unwrap();
        let mut menus = Menus::default();
        let mut app = partida();

        terminal
            .draw(|f| title::ui(f, &mut menus.titulo, &None, &app.settings))
            .unwrap();
        terminal.draw(|f| ui(f, &app)).unwrap();
        terminal.draw(|f| game_over_ui(f, &app)).unwrap();
        terminal
            .draw(|f| bestiary_ui(f, &mut menus.bestiario, &app.settings))
            .unwrap();
        terminal
            .draw(|f| options_ui(f, &app.settings, &mut menus.opciones))
            .unwrap();

        app.show_descend_prompt = true;
        terminal.draw(|f| ui(f, &app)).unwrap();
    }
}

#[test]
fn el_modo_ascii_no_deja_glifos_de_caja() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut app = partida();
    app.settings.glifos = Glifos::Ascii;
    terminal.draw(|f| ui(f, &app)).unwrap();

    let pantalla = texto(&terminal);
    // los muros del mapa: en ascii tienen que caer a '#'
    for glifo in ['║', '═', '╔', '╗', '╚', '╝', '╬'] {
        assert!(
            !pantalla.contains(glifo),
            "en modo ascii sigue apareciendo el glifo de caja «{}»",
            glifo
        );
    }
}

#[test]
fn el_compendio_muestra_la_criatura_elegida() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut menus = Menus::default();
    let app = partida();

    menus.bestiario.select(Some(1));
    terminal
        .draw(|f| bestiary_ui(f, &mut menus.bestiario, &app.settings))
        .unwrap();

    let pantalla = texto(&terminal);
    assert!(pantalla.contains("EL COMPENDIO DE LAS SOMBRAS"));
    assert!(
        pantalla.contains("SERPIENTE DE MÉDULA"),
        "no se ve la ficha"
    );
    assert!(pantalla.contains("VITALIDAD"));
}

#[test]
fn los_ajustes_describen_la_fila_elegida() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut menus = Menus::default();
    let app = partida();

    terminal
        .draw(|f| options_ui(f, &app.settings, &mut menus.opciones))
        .unwrap();
    let pantalla = texto(&terminal);
    assert!(pantalla.contains("SINTONIZAR ALMA"));
    assert!(pantalla.contains("LA PENUMBRA"));
    assert!(pantalla.contains("QUÉ HACE"));
}

#[test]
fn el_historial_muestra_las_ultimas_voces() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut app = partida();
    app.settings.lineas_susurro = 3;
    for i in 0..30 {
        app.add_log(format!("voz numero {}", i), soul48::app::LogType::Info);
    }
    terminal.draw(|f| ui(f, &app)).unwrap();

    let pantalla = texto(&terminal);
    assert!(
        pantalla.contains("voz numero 29"),
        "el historial no muestra el mensaje más reciente"
    );
    assert!(
        !pantalla.contains("voz numero 0 "),
        "el historial quedó anclado en los mensajes viejos"
    );
}

#[test]
fn la_pantalla_de_fin_dice_hasta_donde_llegaste() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut app = partida();
    app.player.hp = 0;
    app.state = GameState::GameOver;
    terminal.draw(|f| game_over_ui(f, &app)).unwrap();

    let pantalla = texto(&terminal);
    assert!(pantalla.contains("HAS CAÍDO"));
    assert!(pantalla.contains("PISO ALCANZADO"));
}

/// Las criptas, con y sin fragmento guardado.
#[test]
fn el_menu_principal_anuncia_el_ultimo_fragmento() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut menus = Menus::default();
    let app = partida();

    terminal
        .draw(|f| title::ui(f, &mut menus.titulo, &None, &app.settings))
        .unwrap();
    assert!(texto(&terminal).contains("sin partida guardada"));

    terminal
        .draw(|f| {
            title::ui(
                f,
                &mut menus.titulo,
                &Some((7, 12, 40, 4242)),
                &app.settings,
            )
        })
        .unwrap();
    let pantalla = texto(&terminal);
    assert!(
        pantalla.contains("piso 7"),
        "no se ve el piso del fragmento"
    );
    assert!(pantalla.contains("12/40"), "no se ve el alma del fragmento");
    assert!(
        pantalla.contains("4242"),
        "no se ve la semilla del fragmento"
    );
}

/// El menú arranca por lo que uno viene a hacer: empezar, después continuar.
#[test]
fn el_menu_principal_empieza_por_lo_primero() {
    use soul48::title::MainMenuOption;

    let etiquetas: Vec<&str> = MainMenuOption::all().iter().map(|o| o.as_str()).collect();
    assert_eq!(
        etiquetas,
        vec![
            "DESCENDER AL ABISMO",
            "RECOGER FRAGMENTOS",
            "COMPENDIO DE SOMBRAS",
            "SINTONIZAR ALMA",
            "VOLVER AL SILENCIO",
        ]
    );
}

/// El título se dibuja con medio bloque y sobrevive el ajuste GLIFOS.
#[test]
fn el_titulo_se_dibuja_en_bloques() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut menus = Menus::default();
    let mut app = partida();

    terminal
        .draw(|f| title::ui(f, &mut menus.titulo, &None, &app.settings))
        .unwrap();
    let unicode = texto(&terminal);
    assert!(
        unicode.contains('█') || unicode.contains('▀'),
        "el título no se está dibujando con medio bloque"
    );
    assert!(
        unicode.contains("t h e   t a l k i n g   d e a d"),
        "falta el subtítulo"
    );

    // en ascii no puede quedar ni un bloque: la fuente puede no tenerlos
    app.settings.glifos = Glifos::Ascii;
    terminal
        .draw(|f| title::ui(f, &mut menus.titulo, &None, &app.settings))
        .unwrap();
    let plano = texto(&terminal);
    for bloque in ['█', '▀', '▄', '↑', '↓', '⏎'] {
        assert!(
            !plano.contains(bloque),
            "en modo ascii sigue apareciendo «{}»",
            bloque
        );
    }
}

/// La pantalla se compacta en una terminal baja en vez de recortarse.
#[test]
fn el_menu_principal_entra_en_una_terminal_baja() {
    let mut menus = Menus::default();
    let app = partida();

    for (ancho, alto) in [(120, 40), (80, 24), (60, 20), (40, 15)] {
        let mut terminal = Terminal::new(TestBackend::new(ancho, alto)).unwrap();
        terminal
            .draw(|f| {
                title::ui(
                    f,
                    &mut menus.titulo,
                    &Some((7, 12, 40, 4242)),
                    &app.settings,
                )
            })
            .unwrap();
        let pantalla = texto(&terminal);
        assert!(
            pantalla.contains("DESCENDER AL ABISMO"),
            "a {}x{} se perdió la primera opción",
            ancho,
            alto
        );
    }
}

/// La pantalla de victoria existe y dice hasta dónde llegaste.
#[test]
fn la_pantalla_de_victoria_celebra_el_final() {
    use soul48::ui::victory_ui;

    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut app = partida();
    app.state = GameState::Victory;
    app.depth = 48;
    terminal.draw(|f| victory_ui(f, &app)).unwrap();

    let pantalla = texto(&terminal);
    assert!(pantalla.contains("EL ORIGEN"));
    assert!(pantalla.contains("HAS RECUPERADO TU VOZ"));
    assert!(pantalla.contains("PISO ALCANZADO"));
    assert!(
        pantalla.contains("SEMILLA"),
        "falta la semilla de la corrida"
    );
}

/// La derrota avisa que el fragmento se disuelve: la corrida es de ida.
#[test]
fn la_derrota_avisa_que_no_hay_vuelta() {
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
    let mut app = partida();
    app.player.hp = 0;
    app.state = GameState::GameOver;
    terminal.draw(|f| game_over_ui(f, &app)).unwrap();

    assert!(texto(&terminal).contains("El fragmento se disuelve"));
}

/// Las dos pantallas de cierre también aguantan una terminal chica.
#[test]
fn el_cierre_aguanta_una_terminal_chica() {
    use soul48::ui::victory_ui;

    for (ancho, alto) in [(80, 24), (60, 20), (40, 15)] {
        let mut terminal = Terminal::new(TestBackend::new(ancho, alto)).unwrap();
        let app = partida();
        terminal.draw(|f| game_over_ui(f, &app)).unwrap();
        terminal.draw(|f| victory_ui(f, &app)).unwrap();
    }
}

/// Cada tramo pinta el mapa con su propia paleta y se nombra en el panel.
#[test]
fn cada_tramo_se_ve_distinto() {
    use ratatui::style::Color;
    use soul48::world::tramo::TRAMOS;

    /// Los colores que efectivamente se pintaron en el buffer.
    fn colores(terminal: &Terminal<TestBackend>) -> std::collections::HashSet<Color> {
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|c| c.fg)
            .collect()
    }

    let mut anteriores = None;
    for t in TRAMOS.iter() {
        let piso = t.rango.0;
        let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
        let mut app = App::new(Some(5150), None, None, piso, None);
        app.start_new_game();
        terminal.draw(|f| ui(f, &app)).unwrap();

        assert!(
            texto(&terminal).contains(t.nombre),
            "el piso {} no se anuncia como «{}»",
            piso,
            t.nombre
        );
        assert!(
            colores(&terminal).contains(&t.muro),
            "el piso {} no usa el muro de «{}»",
            piso,
            t.nombre
        );

        if let Some(previos) = anteriores {
            assert_ne!(
                previos,
                colores(&terminal),
                "«{}» se ve igual que el tramo anterior",
                t.nombre
            );
        }
        anteriores = Some(colores(&terminal));
    }
}
