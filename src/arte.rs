//! Los píxeles.
//!
//! Formato: una línea por fila, un carácter por píxel.
//!   `.`      transparente
//!   `1`-`4`  rampa de tonos derivada del color de la criatura (1 sombra, 4 pleno)
//!   `o`      acento (ojos)
//!   `h c d`  hueso, ceniza, ceniza honda
//!   `m`      muro
//!   `v r a g` violeta, rojo altar, azul alma, oro
//!
//! Editar el arte es editar texto: mientras todas las filas midan lo mismo,
//! cualquier dibujo entra.

use crate::sprite::Sprite;

pub const MURCIELAGO: Sprite = Sprite {
    arte: &[
        "................",
        "......4..4......",
        "......4444......",
        ".....14oo41.....",
        "....12444421....",
        "...1233443321...",
        "..123333333321..",
        ".12333333333321.",
        ".12333333333321.",
        "..123333333321..",
        "...1233333321...",
        "....12322321....",
        ".....12..21.....",
        "......1..1......",
        "................",
        "................",
    ],
};

pub const SERPIENTE: Sprite = Sprite {
    arte: &[
        "................",
        "..........44444.",
        ".........4333334",
        ".........43oo334",
        ".........4333334",
        "..........44344.",
        "............34..",
        "..........4334..",
        ".......44334....",
        "....443334......",
        "..4433334.......",
        "..433334........",
        "..43334.....444.",
        "..4334...4443334",
        "..4433444333334.",
        "...44433333344..",
    ],
};

pub const LADRON: Sprite = Sprite {
    arte: &[
        "................",
        ".....433334.....",
        "....43333334....",
        "...4333333334...",
        "...433oo.oo334..",
        "...4333333334...",
        "....43333334....",
        ".....433334.....",
        "...433333334....",
        "..43333333334...",
        "..433333333334..",
        "..433333333334..",
        "..4333333333334.",
        "..433333333334..",
        "...4444...4444..",
        "................",
    ],
};

pub const GNOLL: Sprite = Sprite {
    arte: &[
        "................",
        "..44........44..",
        "..434......434..",
        "..4334....4334..",
        "...4334444334...",
        "..433333333334..",
        "..43oo3333oo34..",
        "..433333333334..",
        "...43333333334..",
        "....433hhhh334..",
        ".....4hhhhhh4...",
        "......4hhhh4....",
        "...4444444444...",
        "..433333333334..",
        "..433333333334..",
        "................",
    ],
};

pub const MIMICO: Sprite = Sprite {
    arte: &[
        "................",
        "...4444444444...",
        "..433333333334..",
        "..43hhhhhhhh34..",
        "...4hhhhhhhh4...",
        "...4hh.hh.hh4...",
        "..4..........4..",
        "..4.oo....oo.4..",
        "..4..........4..",
        "...4hh.hh.hh4...",
        "....hhhhhhhh....",
        "..433333333334..",
        "..4333gggg3334..",
        "..433333333334..",
        "...4444444444...",
        "................",
    ],
};

/// El umbral del piso 48: un arco de piedra, una vela y un alma que sube.
pub const PORTAL: Sprite = Sprite {
    arte: &[
        "................................",
        "......mmmmmmmmmmmmmmmmmm........",
        ".....mmm..............mmm.......",
        "....mm..................mm......",
        "....mm.......gg.........mm......",
        "....mm......gooog.......mm......",
        "....mm.......gg.........mm......",
        "....mm........a.........mm......",
        "....mm.......aaa........mm......",
        "....mm......aa.aa.......mm......",
        "....mm......a...a.......mm......",
        "..mmmmmm..............mmmmmm....",
    ],
};

/// Lo que queda cuando el alma se apaga.
pub const CALAVERA: Sprite = Sprite {
    arte: &[
        "................................",
        "..........hhhhhhhhhhhh..........",
        ".........hhhhhhhhhhhhhh.........",
        "........hhhhhhhhhhhhhhhh........",
        "........hhh.rrrr.rrrr.hhh.......",
        "........hh.rroor.roorr.hh.......",
        "........hhh.rrrr.rrrr.hhh.......",
        ".........hhhhhhhhhhhhhh.........",
        "..........hhhhh..hhhhh..........",
        "..........hhhh.hh.hhhh..........",
        "...........hhhhhhhhhh...........",
        "..........hhhhhhhhhhhh..........",
        "..........h.h.h.h.h.hh..........",
        "..........hhhhhhhhhhhh..........",
        "...........dddddddddd...........",
        "................................",
    ],
};

/// El retrato que le corresponde a cada criatura, por su glifo en el mapa.
pub fn de_criatura(glifo: char) -> Option<&'static Sprite> {
    match glifo {
        'b' => Some(&MURCIELAGO),
        's' => Some(&SERPIENTE),
        'L' => Some(&LADRON),
        'g' => Some(&GNOLL),
        'C' => Some(&MIMICO),
        _ => None,
    }
}
