mod app;
mod bestiary;
mod map_builder;
mod title;
mod ui;

use app::{App, GameState, LogType};
use title::MainMenuOption;
use ui::{bestiary_ui, game_over_ui, ui};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

/// Punto de entrada principal de la aplicación.
/// Gestiona el ciclo de vida del terminal, el bucle de eventos y el renderizado.
fn main() -> Result<(), Box<dyn Error>> {
    // Configuración inicial del terminal en modo raw
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Inicialización del estado de la aplicación
    let mut app = App::new(None, None, None, 1, None);

    loop {
        // --- 1. RENDERIZADO ---
        // Dibuja la interfaz correspondiente según el estado actual del juego
        match app.state {
            GameState::TitleScreen => {
                terminal.draw(|f| title::ui(f, &mut app.title_menu_state))?;
            }
            GameState::Playing => {
                terminal.draw(|f| ui(f, &app))?;
            }
            GameState::GameOver => {
                terminal.draw(|f| game_over_ui(f, &app))?;
            }
            GameState::Bestiary => {
                terminal.draw(|f| bestiary_ui(f, &mut app.bestiary_state))?;
            }
        }

        // --- 2. MANEJO DE EVENTOS (POLLEO NO BLOQUEANTE) ---
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match app.state {
                        // Gestión de navegación y selección en el menú principal
                        GameState::TitleScreen => match key.code {
                            KeyCode::Up => {
                                let i = match app.title_menu_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            MainMenuOption::all().len() - 1
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.title_menu_state.select(Some(i));
                            }
                            KeyCode::Down => {
                                let i = match app.title_menu_state.selected() {
                                    Some(i) => {
                                        if i >= MainMenuOption::all().len() - 1 {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.title_menu_state.select(Some(i));
                            }
                            KeyCode::Enter => {
                                if let Some(i) = app.title_menu_state.selected() {
                                    match MainMenuOption::all()[i] {
                                        MainMenuOption::StartGame => {
                                            app.start_new_game();
                                        }
                                        MainMenuOption::Bestiary => {
                                            app.state = GameState::Bestiary;
                                        }
                                        MainMenuOption::Quit => break,
                                        _ => {}
                                    }
                                }
                            }
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            _ => {}
                        },

                        // Navegación dentro del bestiario de criaturas
                        GameState::Bestiary => match key.code {
                            KeyCode::Up => {
                                let i = match app.bestiary_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            crate::bestiary::get_bestiary().len() - 1
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.bestiary_state.select(Some(i));
                            }
                            KeyCode::Down => {
                                let i = match app.bestiary_state.selected() {
                                    Some(i) => {
                                        if i >= crate::bestiary::get_bestiary().len() - 1 {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.bestiary_state.select(Some(i));
                            }
                            KeyCode::Char('q') | KeyCode::Esc => {
                                app.state = GameState::TitleScreen;
                            }
                            _ => {}
                        },

                        // Lógica principal durante la exploración de la mazmorra
                        GameState::Playing => {
                            if app.show_descend_prompt {
                                match key.code {
                                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Enter => {
                                        app.confirm_descent(true);
                                        let next_depth = app.depth + 1;
                                        let current_hp = Some(app.hero_hp);
                                        let current_inv = Some(app.inventory.clone());
                                        let current_weapon = app.equipped_weapon.clone();
                                        app = App::new(
                                            None,
                                            current_hp,
                                            current_inv,
                                            next_depth,
                                            current_weapon,
                                        );
                                        app.add_log(
                                            format!("> HAS DESCENDIDO AL NIVEL {}", next_depth),
                                            LogType::Info,
                                        );
                                    }
                                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                        app.confirm_descent(false);
                                        app.add_log(
                                            "> Decides quedarte en este nivel.".into(),
                                            LogType::Info,
                                        );
                                    }
                                    _ => {}
                                }
                                continue;
                            }

                            let mut action_taken = false;
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => break,
                                KeyCode::Up => {
                                    action_taken = app.try_move(0, -1);
                                }
                                KeyCode::Down => {
                                    action_taken = app.try_move(0, 1);
                                }
                                KeyCode::Left => {
                                    action_taken = app.try_move(-1, 0);
                                }
                                KeyCode::Right => {
                                    action_taken = app.try_move(1, 0);
                                }

                                KeyCode::Char('d') => {
                                    app.drop_mode = !app.drop_mode;
                                    if app.drop_mode {
                                        app.add_log(
                                            "> [DESCARTAR] Pulsa 1-9 (o 'd' para cancelar).".into(),
                                            LogType::Warning,
                                        );
                                    } else {
                                        app.add_log(
                                            "> Modo descartar cancelado.".into(),
                                            LogType::Info,
                                        );
                                    }
                                }

                                KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                                    let idx = (c as u8 - b'1') as usize;
                                    if app.drop_mode {
                                        action_taken = app.drop_item(idx);
                                        app.drop_mode = false;
                                    } else {
                                        action_taken = app.use_item(idx);
                                    }
                                }
                                _ => {}
                            }

                            // Si el jugador realizó una acción, se procesa el turno de los enemigos
                            if action_taken {
                                if app.drop_mode {
                                    app.drop_mode = false;
                                    app.add_log(
                                        "> Modo descartar cancelado.".into(),
                                        LogType::Info,
                                    );
                                }

                                if !app.show_descend_prompt && !app.should_descend {
                                    app.process_enemy_turns();
                                    app.calculate_fov();

                                    // Verificación de estado de salud del héroe
                                    if app.hero_hp <= 0 {
                                        app.state = GameState::GameOver;
                                    }
                                }
                            }
                        }

                        // Gestión de la pantalla de fin de juego (muerte)
                        GameState::GameOver => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                app = App::new(None, None, None, 1, None);
                            }
                            _ => {}
                        },
                    }
                }
            } else if let Event::Mouse(mouse_event) = event::read()? {
                // Inspección de tiles mediante clic izquierdo
                if app.state == GameState::Playing {
                    if mouse_event.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
                        app.inspect_tile(mouse_event.column, mouse_event.row);
                    }
                }
            }
        }
    }

    // Restauración del estado original del terminal al salir
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
