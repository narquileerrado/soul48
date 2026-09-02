//! Traducción de teclas a intenciones, una por pantalla.
//!
//! Antes esto era un `match` anidado de 250 líneas dentro del bucle de
//! `main.rs`, donde convivían la navegación del menú, los ajustes y el turno de
//! juego. Acá cada pantalla declara qué entiende; `main.rs` decide qué hacer
//! con eso.

use crossterm::event::KeyCode;

/// Menús con lista: criptas, compendio.
pub enum AccionMenu {
    Subir,
    Bajar,
    Elegir,
    Volver,
}

/// Pantalla de ajustes.
pub enum AccionAjustes {
    Subir,
    Bajar,
    Menos,
    Mas,
    Confirmar,
    Volver,
}

/// Exploración de la mazmorra.
pub enum AccionJuego {
    Mover(isize, isize),
    Embestir,
    Bloquear,
    ModoSoltar,
    /// Índice 0-8, ya traducido desde las teclas 1-9.
    Ranura(usize),
    Salir,
}

/// La pregunta de las escaleras.
pub enum AccionDescenso {
    Bajar,
    Quedarse,
}

/// Fin de la partida.
pub enum AccionFin {
    Reiniciar,
    Salir,
}

fn es_volver(code: KeyCode) -> bool {
    matches!(code, KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc)
}

pub fn menu(code: KeyCode) -> Option<AccionMenu> {
    match code {
        KeyCode::Up => Some(AccionMenu::Subir),
        KeyCode::Down => Some(AccionMenu::Bajar),
        KeyCode::Enter => Some(AccionMenu::Elegir),
        c if es_volver(c) => Some(AccionMenu::Volver),
        _ => None,
    }
}

pub fn ajustes(code: KeyCode) -> Option<AccionAjustes> {
    match code {
        KeyCode::Up => Some(AccionAjustes::Subir),
        KeyCode::Down => Some(AccionAjustes::Bajar),
        KeyCode::Left => Some(AccionAjustes::Menos),
        KeyCode::Right => Some(AccionAjustes::Mas),
        KeyCode::Enter => Some(AccionAjustes::Confirmar),
        c if es_volver(c) => Some(AccionAjustes::Volver),
        _ => None,
    }
}

pub fn juego(code: KeyCode) -> Option<AccionJuego> {
    match code {
        KeyCode::Up => Some(AccionJuego::Mover(0, -1)),
        KeyCode::Down => Some(AccionJuego::Mover(0, 1)),
        KeyCode::Left => Some(AccionJuego::Mover(-1, 0)),
        KeyCode::Right => Some(AccionJuego::Mover(1, 0)),
        KeyCode::Char('e') | KeyCode::Char('E') => Some(AccionJuego::Embestir),
        KeyCode::Char('b') | KeyCode::Char('B') => Some(AccionJuego::Bloquear),
        KeyCode::Char('d') | KeyCode::Char('D') => Some(AccionJuego::ModoSoltar),
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
            Some(AccionJuego::Ranura((c as u8 - b'1') as usize))
        }
        c if es_volver(c) => Some(AccionJuego::Salir),
        _ => None,
    }
}

pub fn descenso(code: KeyCode) -> Option<AccionDescenso> {
    match code {
        KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Enter => Some(AccionDescenso::Bajar),
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => Some(AccionDescenso::Quedarse),
        _ => None,
    }
}

pub fn fin(code: KeyCode) -> Option<AccionFin> {
    match code {
        KeyCode::Char('r') | KeyCode::Char('R') => Some(AccionFin::Reiniciar),
        c if es_volver(c) => Some(AccionFin::Salir),
        _ => None,
    }
}
