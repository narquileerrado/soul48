//! Dibujo de todas las pantallas del juego.
//!
//! `widgets` tiene las piezas compartidas —barras, medidores, separadores,
//! recortes de texto—; cada pantalla vive en su propio archivo.

pub mod compendio;
pub mod juego;
pub mod opciones;
pub mod widgets;

pub use compendio::bestiary_ui;
pub use juego::{game_over_ui, ui};
pub use opciones::options_ui;
