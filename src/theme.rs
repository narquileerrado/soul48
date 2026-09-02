//! Sistema de color de Soul 48.
//!
//! Cada color tiene un significado y uno solo:
//!   oro      -> foco, objetivo, selección
//!   violeta  -> la voz: paredes que hablan, susurros, lore
//!   rojo     -> sangre: daño, altar, muerte
//!   azul     -> vos: el héroe y su alma
//! El resto es hueso y ceniza.

use ratatui::style::Color;

pub const ORO: Color = Color::Rgb(212, 175, 55);
pub const ORO_APAGADO: Color = Color::Rgb(107, 90, 34);
pub const VIOLETA: Color = Color::Rgb(180, 140, 200);
pub const ROJO_ALTAR: Color = Color::Rgb(255, 100, 100);
pub const AZUL_ALMA: Color = Color::Rgb(100, 200, 255);
pub const AZUL_APAGADO: Color = Color::Rgb(47, 95, 120);
pub const VIOLETA_APAGADO: Color = Color::Rgb(92, 70, 102);
pub const AMBAR: Color = Color::Rgb(224, 138, 60);
pub const HUESO: Color = Color::Rgb(232, 224, 208);
pub const CENIZA: Color = Color::Rgb(138, 129, 117);
pub const CENIZA_HONDA: Color = Color::Rgb(74, 69, 62);
pub const MURO: Color = Color::Rgb(111, 106, 97);
pub const SUELO: Color = Color::Rgb(58, 53, 47);
pub const PENUMBRA: Color = Color::Rgb(14, 12, 11);
/* --- paletas de tramo: cada doce pisos el descenso cambia de piel --- */
/// Las Criptas usan `MURO` y `SUELO`, la paleta base.
pub const MURO_CATACUMBA: Color = Color::Rgb(96, 108, 99);
pub const SUELO_CATACUMBA: Color = Color::Rgb(48, 58, 50);
pub const MURO_ABISMO: Color = Color::Rgb(88, 82, 112);
pub const SUELO_ABISMO: Color = Color::Rgb(42, 38, 58);
pub const MURO_SILENCIO: Color = Color::Rgb(120, 96, 104);
pub const SUELO_SILENCIO: Color = Color::Rgb(56, 40, 46);

/* --- colores de criaturas: viven acá y no repetidos en cada catálogo --- */
pub const MURCIELAGO: Color = Color::Rgb(110, 110, 110);
pub const VERDE_VENENO: Color = Color::Rgb(78, 154, 78);
pub const AZUL_LADRON: Color = Color::Rgb(92, 127, 209);
pub const PARDO_GNOLL: Color = Color::Rgb(184, 106, 40);

/// Dorado de tesoro: cofres y mímicos. Más apagado que el oro del cromo,
/// que está reservado a lo que la interfaz quiere que mires.
pub const COFRE: Color = Color::Rgb(196, 160, 0);

/// Mezcla un color hacia la penumbra.
///
/// `brillo` va de 0 (se lo traga la oscuridad) a 100 (color pleno). Es lo que
/// permite recordar un altar o una escalera en una versión apagada de su
/// propio color en vez de un gris uniforme.
pub fn recordado(base: Color, brillo: u8) -> Color {
    let (r, g, b) = match base {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Black => (0, 0, 0),
        Color::White => (232, 224, 208),
        Color::Red | Color::LightRed => (255, 100, 100),
        Color::Green | Color::LightGreen => (78, 154, 78),
        Color::Yellow | Color::LightYellow => (196, 160, 0),
        Color::Blue | Color::LightBlue => (92, 127, 209),
        Color::Magenta | Color::LightMagenta => (180, 140, 200),
        Color::Cyan | Color::LightCyan => (100, 200, 255),
        Color::Gray => (111, 106, 97),
        Color::DarkGray => (110, 110, 110),
        _ => (138, 129, 117),
    };
    let t = brillo.min(100) as u32;
    let mezcla = |c: u8, fondo: u32| ((fondo * (100 - t) + c as u32 * t) / 100) as u8;
    Color::Rgb(mezcla(r, 14), mezcla(g, 12), mezcla(b, 11))
}
