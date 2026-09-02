//! Soul 48: The Talking Dead.
//!
//! El crate expone los módulos como biblioteca para que los tests de
//! integración de `tests/` puedan armar partidas completas; `main.rs` es
//! apenas el arranque de la terminal y el bucle de eventos.

// El mapa es una grilla `Vec<Vec<char>>` y se recorre por índice a propósito:
// iterar por referencia obliga a zips anidados que oscurecen el barrido en 2D.
#![allow(clippy::needless_range_loop)]

pub mod arte;
pub mod balance;
pub mod game;
/// `app` es el nombre histórico de este módulo.
pub use game as app;
pub mod bestiary;
pub mod input;
pub mod menus;
pub mod player;
pub mod settings;
pub mod sprite;
pub mod theme;
pub mod title;
pub mod ui;
pub mod world;
