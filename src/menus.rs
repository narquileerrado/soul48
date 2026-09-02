//! Estado de navegación de los menús.
//!
//! Son cursores de widget de `ratatui`, no estado del juego: vivían dentro de
//! `App` y había que inicializarlos igual en `App::new` y en
//! `App::load_from_file`. Cargar una partida no debería tener que acordarse de
//! dónde estaba parado el cursor del bestiario.

use ratatui::widgets::ListState;

/// Los tres cursores de las pantallas con lista.
pub struct Menus {
    pub titulo: ListState,
    pub bestiario: ListState,
    pub opciones: ListState,
}

impl Default for Menus {
    fn default() -> Self {
        let cursor = || {
            let mut s = ListState::default();
            s.select(Some(0));
            s
        };
        Menus {
            titulo: cursor(),
            bestiario: cursor(),
            opciones: cursor(),
        }
    }
}

impl Menus {
    /// Mueve un cursor con envolvimiento en los dos extremos.
    pub fn mover(estado: &mut ListState, delta: isize, total: usize) {
        if total == 0 {
            return;
        }
        let actual = estado.selected().unwrap_or(0) as isize;
        let siguiente = (actual + delta).rem_euclid(total as isize);
        estado.select(Some(siguiente as usize));
    }
}
