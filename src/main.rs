//! Punto de entrada: terminal, bucle de eventos y despacho.
//!
//! Toda la lógica vive en la biblioteca (`soul48::*`). Acá sólo se prende y se
//! apaga la terminal, se dibuja la pantalla que corresponde al estado y se
//! traduce cada tecla a una acción del juego.

use soul48::app::{App, GameState, LogType};
use soul48::bestiary;
use soul48::input::{self, AccionAjustes, AccionDescenso, AccionFin, AccionJuego, AccionMenu};
use soul48::menus::Menus;
use soul48::settings::{Settings, AJUSTES, RUTA_AJUSTES};
use soul48::title::{self, MainMenuOption};
use soul48::ui::{bestiary_ui, game_over_ui, options_ui, ui, victory_ui};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

/// Dónde queda el fragmento de alma entre partidas.
const RUTA_PARTIDA: &str = "savegame.json";

/// Cuánto espera el bucle por una tecla antes de volver a mirar.
///
/// El juego es por turnos: sin evento no hay nada nuevo que dibujar, así que
/// esperar bloqueado es lo correcto. Antes el bucle redibujaba la pantalla
/// entera cada 16 ms aunque no pasara nada.
const ESPERA_EVENTO: Duration = Duration::from_millis(250);

/// Cada cuánto avanza un carácter la cinta del relato del título.
///
/// Es lo único del juego que se mueve solo, así que sólo la pantalla de título
/// se redibuja sin que pase nada; el resto sigue esperando un evento.
const PASO_CINTA: Duration = Duration::from_millis(110);

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let resultado = correr(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    resultado
}

/// El bucle del juego, separado del montaje de la terminal para que un error
/// acá no deje la consola en modo raw.
fn correr<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut app = App::new(None, None, None, 1, None);
    // los ajustes son del jugador, no del estado del juego: se leen acá
    app.settings = Settings::load(RUTA_AJUSTES);
    let mut menus = Menus::default();
    // qué te espera si elegís RECOGER FRAGMENTOS
    let mut fragmento = App::peek_save(RUTA_PARTIDA);
    let mut redibujar = true;
    let comienzo = Instant::now();

    loop {
        if redibujar {
            let desplazamiento = (comienzo.elapsed().as_millis() / PASO_CINTA.as_millis()) as usize;
            match app.state {
                GameState::TitleScreen => {
                    terminal.draw(|f| {
                        title::ui(
                            f,
                            &mut menus.titulo,
                            &fragmento,
                            &app.settings,
                            desplazamiento,
                        )
                    })?;
                }
                GameState::Playing => {
                    terminal.draw(|f| ui(f, &app))?;
                }
                GameState::GameOver => {
                    terminal.draw(|f| game_over_ui(f, &app))?;
                }
                GameState::Victory => {
                    terminal.draw(|f| victory_ui(f, &app))?;
                }
                GameState::Bestiary => {
                    terminal.draw(|f| bestiary_ui(f, &mut menus.bestiario, &app.settings))?;
                }
                GameState::Options => {
                    terminal.draw(|f| options_ui(f, &app.settings, &mut menus.opciones))?;
                }
            }
            redibujar = false;
        }

        // en el título la cinta corre sola; en el resto no hay nada que animar
        let en_titulo = app.state == GameState::TitleScreen;
        let espera = if en_titulo { PASO_CINTA } else { ESPERA_EVENTO };
        if !event::poll(espera)? {
            redibujar = en_titulo;
            continue;
        }

        match event::read()? {
            Event::Key(key) if key.kind == event::KeyEventKind::Press => {
                redibujar = true;
                let seguir = match app.state {
                    GameState::TitleScreen => {
                        pantalla_titulo(&mut app, &mut menus, &mut fragmento, key.code)
                    }
                    GameState::Bestiary => pantalla_compendio(&mut app, &mut menus, key.code),
                    GameState::Options => pantalla_ajustes(&mut app, &mut menus, key.code),
                    GameState::Playing => pantalla_juego(&mut app, key.code),
                    GameState::GameOver | GameState::Victory => pantalla_fin(&mut app, key.code),
                };
                if !seguir {
                    return Ok(());
                }
            }
            Event::Mouse(mouse) => {
                if app.state == GameState::Playing
                    && mouse.kind == event::MouseEventKind::Down(event::MouseButton::Left)
                {
                    app.inspect_tile(mouse.column, mouse.row);
                    redibujar = true;
                }
            }
            Event::Resize(_, _) => redibujar = true,
            _ => {}
        }
    }
}

/// Devuelve `false` cuando hay que cerrar el juego.
fn pantalla_titulo(
    app: &mut App,
    menus: &mut Menus,
    fragmento: &mut title::Fragmento,
    code: KeyCode,
) -> bool {
    let total = MainMenuOption::all().len();
    match input::menu(code) {
        Some(AccionMenu::Subir) => Menus::mover(&mut menus.titulo, -1, total),
        Some(AccionMenu::Bajar) => Menus::mover(&mut menus.titulo, 1, total),
        Some(AccionMenu::Volver) => return false,
        Some(AccionMenu::Elegir) => {
            let i = menus.titulo.selected().unwrap_or(0);
            match MainMenuOption::all()[i] {
                MainMenuOption::StartGame => app.start_new_game(),
                MainMenuOption::Bestiary => app.state = GameState::Bestiary,
                MainMenuOption::LoadGame => match App::load_from_file(RUTA_PARTIDA) {
                    Ok(cargada) => {
                        // la jornada cambia; los ajustes son de quien juega
                        let ajustes = app.settings.clone();
                        *app = cargada;
                        app.settings = ajustes;
                    }
                    Err(e) => app.add_log(
                        format!("> No se pudo cobrar el fragmento: {}", e),
                        LogType::Warning,
                    ),
                },
                MainMenuOption::Options => app.state = GameState::Options,
                MainMenuOption::Quit => return false,
            }
            *fragmento = App::peek_save(RUTA_PARTIDA);
        }
        None => {}
    }
    true
}

fn pantalla_compendio(app: &mut App, menus: &mut Menus, code: KeyCode) -> bool {
    let total = bestiary::get_bestiary().len();
    match input::menu(code) {
        Some(AccionMenu::Subir) => Menus::mover(&mut menus.bestiario, -1, total),
        Some(AccionMenu::Bajar) => Menus::mover(&mut menus.bestiario, 1, total),
        Some(AccionMenu::Volver) => app.state = GameState::TitleScreen,
        _ => {}
    }
    true
}

fn pantalla_ajustes(app: &mut App, menus: &mut Menus, code: KeyCode) -> bool {
    let total = AJUSTES.len();
    let actual = menus.opciones.selected().unwrap_or(0);
    match input::ajustes(code) {
        Some(AccionAjustes::Subir) => Menus::mover(&mut menus.opciones, -1, total),
        Some(AccionAjustes::Bajar) => Menus::mover(&mut menus.opciones, 1, total),
        Some(AccionAjustes::Menos) => app.settings.ajustar(actual, -1),
        Some(AccionAjustes::Mas) => app.settings.ajustar(actual, 1),
        Some(AccionAjustes::Confirmar) => {
            if actual == total - 1 {
                app.settings.restablecer();
            } else {
                app.settings.ajustar(actual, 1);
            }
        }
        Some(AccionAjustes::Volver) => {
            app.settings.save(RUTA_AJUSTES);
            app.state = GameState::TitleScreen;
        }
        None => {}
    }
    true
}

fn pantalla_juego(app: &mut App, code: KeyCode) -> bool {
    // la pregunta de las escaleras se come el turno entero
    if app.show_descend_prompt {
        match input::descenso(code) {
            Some(AccionDescenso::Bajar) => {
                app.confirm_descent();
                app.descend();
                // El piso nuevo es el punto de control. La corrida es larga y
                // sin esto se guardaba sólo al salir con Q.
                if let Err(e) = app.save_to_file(RUTA_PARTIDA) {
                    app.add_log(
                        format!("> No pudo asentarse el fragmento: {}", e),
                        LogType::Warning,
                    );
                }
            }
            Some(AccionDescenso::Quedarse) => {
                app.confirm_descent();
                app.add_log(
                    "> Determináis quedaros en aqueste sótano.".into(),
                    LogType::Info,
                );
            }
            None => {}
        }
        return true;
    }

    let mut hubo_accion = false;
    match input::juego(code) {
        Some(AccionJuego::Mover(dx, dy)) => hubo_accion = app.try_move(dx, dy),
        Some(AccionJuego::Embestir) => hubo_accion = app.use_pushback(),
        Some(AccionJuego::Bloquear) => hubo_accion = app.use_parry(),
        Some(AccionJuego::ModoSoltar) => {
            app.drop_mode = !app.drop_mode;
            let aviso = if app.drop_mode {
                "> [DEXAR] Pulsad 1-9 (o «d» para dexarlo estar)."
            } else {
                "> Dexado está el modo de dexar."
            };
            app.add_log(aviso.into(), LogType::Warning);
        }
        Some(AccionJuego::Ranura(idx)) => {
            if app.drop_mode {
                hubo_accion = app.drop_item(idx);
                app.drop_mode = false;
            } else {
                hubo_accion = app.use_item(idx);
            }
        }
        Some(AccionJuego::Salir) => {
            if app.settings.guardado_automatico {
                let _ = app.save_to_file(RUTA_PARTIDA);
            }
            return false;
        }
        None => {}
    }

    if hubo_accion {
        if app.drop_mode {
            app.drop_mode = false;
            app.add_log("> Dexado está el modo de dexar.".into(), LogType::Info);
        }
        if !app.show_descend_prompt {
            app.process_enemy_turns();
            app.calculate_fov();
            if app.player.hp <= 0 {
                app.state = GameState::GameOver;
            }
        }
        // Morir o llegar al final disuelven el fragmento: no se recarga la
        // partida anterior para volver a intentar el mismo piso.
        if matches!(app.state, GameState::GameOver | GameState::Victory) {
            App::borrar_save(RUTA_PARTIDA);
        }
    }
    true
}

fn pantalla_fin(app: &mut App, code: KeyCode) -> bool {
    match input::fin(code) {
        Some(AccionFin::Reiniciar) => {
            App::borrar_save(RUTA_PARTIDA);
            *app = App::new(None, None, None, 1, None);
            app.start_new_game();
        }
        Some(AccionFin::Salir) => return false,
        None => {}
    }
    true
}
