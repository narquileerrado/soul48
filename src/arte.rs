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

/* ──────────────────────── criaturas de los tramos hondos ──────────────────────── */

/// Nunca viene sola: hocico, oreja y una cola que no termina.
pub const RATA: Sprite = Sprite {
    arte: &[
        "................",
        "................",
        "....4...........",
        "...444..........",
        "...4444444......",
        "..443oo333444...",
        "..4333333333334.",
        ".43333333333334.",
        ".43333333333334.",
        "..433333333334..",
        "...4444444444...",
        "....4..4..4..1..",
        "..............1.",
        "...............1",
        "................",
        "................",
    ],
};

/// No es un esqueleto: son varios, mal repartidos.
pub const OSARIO: Sprite = Sprite {
    arte: &[
        "................",
        "......hhhh......",
        ".....hhhhhh.....",
        ".....hoohooh....",
        ".....hhhhhhh....",
        "......h.h.h.....",
        "....hhhhhhhh....",
        "...h44444444h...",
        "..h4444444444h..",
        "..h44h4444h44h..",
        "...4444444444...",
        "....h44444h.....",
        "...h444..444h...",
        "...h44....44h...",
        "..hh4h....h4hh..",
        "................",
    ],
};

/// Una sombra sin nadie que la proyecte.
pub const SOMBRA: Sprite = Sprite {
    arte: &[
        "................",
        "......4444......",
        ".....433334.....",
        "....43333334....",
        "....4o3333o4....",
        "....43333334....",
        "...4333333334...",
        "...4333333334...",
        "..433333333334..",
        "..433333333334..",
        "..433333333334..",
        "...4333333334...",
        "...1433333341...",
        "....1.4334.1....",
        "......1..1......",
        "................",
    ],
};

/// No te muerde: te escucha, y algo de lo que ibas a decir deja de estar.
pub const DEVORADOR: Sprite = Sprite {
    arte: &[
        "................",
        "...4444444444...",
        "..433333333334..",
        ".43333333333334.",
        ".433oo3333oo334.",
        ".43333333333334.",
        ".43333333333334.",
        "..hhhhhhhhhhhh..",
        "..h4h4h4h4h4h4..",
        "..hhhhhhhhhhhh..",
        ".43333333333334.",
        ".43333333333334.",
        "..433333333334..",
        "...4444444444...",
        "....1.1..1.1....",
        "................",
    ],
};

/// Muchas gargantas y ninguna boca.
pub const CORO: Sprite = Sprite {
    arte: &[
        "................",
        "..4444....4444..",
        ".433334..433334.",
        ".43oo34..43oo34.",
        ".433334..433334.",
        ".4hhhh4..4hhhh4.",
        "..4444....4444..",
        "................",
        "....4444444.....",
        "...433333334....",
        "...43oo3oo34....",
        "...433333334....",
        "...4hhhhhhh4....",
        "....4444444.....",
        "................",
        "................",
    ],
};

/// El que anuncia lo que viene, aunque no lo diga.
pub const HERALDO: Sprite = Sprite {
    arte: &[
        "................",
        "......4444......",
        ".....433334.....",
        "....43333334....",
        "....43333334....",
        "....4.oo.o34....",
        "....43333334....",
        "...4333333334...",
        "...433hhh33334..",
        "..43333h3333334.",
        "..4333333333334.",
        "..4333333333334.",
        ".43333333333334.",
        ".43333333333334.",
        ".44444444444444.",
        "................",
    ],
};

/* ──────────────────────────────── los jefes ──────────────────────────────── */

/// Lo que queda cuando una cripta entera decide levantarse junta.
pub const OSARIO_MAYOR: Sprite = Sprite {
    arte: &[
        "................",
        "...hhh....hhh...",
        "..hoohh..hhooh..",
        "..hhhhh..hhhhh..",
        "...hhh....hhh...",
        "....hhhhhhhh....",
        "..hhhhhhhhhhhh..",
        ".h444444444444h.",
        "h44444444444444h",
        "h44h44444444h44h",
        ".h4444444444h4h.",
        "..h444444444h...",
        "..hh4444444hh...",
        ".hh44h..h44hh...",
        ".h44h....h44h...",
        "................",
    ],
};

/// Tachó los nombres de las paredes uno por uno, y se quedó cuidando el trabajo.
pub const CUSTODIO: Sprite = Sprite {
    arte: &[
        "................",
        ".....444444.....",
        "....43333334....",
        "....4o3333o4....",
        "....43333334....",
        "...4433333344...",
        "...4333333334...",
        "..433333333334..",
        "..4hhhhhhhhhh4..",
        "..4h.h.h.h.hh4..",
        "..4hhhhhhhhhh4..",
        "..4h.h.h.h.hh4..",
        "..4hhhhhhhhhh4..",
        "..433333333334..",
        "..444444444444..",
        "................",
    ],
};

/// La parte del Abismo que se molestó en tener forma.
pub const BOCA: Sprite = Sprite {
    arte: &[
        "................",
        "..444444444444..",
        ".43333333333334.",
        "4333333333333334",
        "433hhhhhhhhhh334",
        "43hh33333333hh34",
        "43h3333333333h34",
        "43h33oo33oo33h34",
        "43h3333333333h34",
        "43hh33333333hh34",
        "433hhhhhhhhhh334",
        "4333333333333334",
        ".43333333333334.",
        "..44444444444...",
        "................",
        "................",
    ],
};

/// El último antes del final, y el único que sabe lo que estás por escuchar.
pub const HERALDO_MAYOR: Sprite = Sprite {
    arte: &[
        "................",
        "...g........g...",
        "...gg......gg...",
        "....gg4444gg....",
        "....43333334....",
        "...4333333334...",
        "...4.oo..oo.4...",
        "...4333333334...",
        "..433hhhhh3334..",
        "..4333hhh33334..",
        "..43333333334...",
        ".4333333333334..",
        ".43333333333334.",
        ".43333333333334.",
        "4444444444444444",
        "................",
    ],
};

/// Te sacó la voz y se quedó con ella cuarenta y ocho pisos más abajo.
pub const ARCHIDEMONIO: Sprite = Sprite {
    arte: &[
        "4..............4",
        "44............44",
        "444..........444",
        "4444........4444",
        ".4444444444444..",
        "..44333333344...",
        "..43oo3333oo34..",
        "..433333333334..",
        "..43333rr333334.",
        "..4333333333334.",
        ".43hhhhhhhhhh34.",
        ".43h.h.h.h.hh34.",
        ".433hhhhhhhh334.",
        ".43333333333334.",
        "..444444444444..",
        "...4........4...",
    ],
};

/// El retrato que le corresponde a cada criatura, por su nombre corto.
///
/// Va por nombre y no por glifo porque los cuatro Guardianes de tramo comparten
/// la `B` en el mapa: buscándolos por glifo, los cuatro mostraban el mismo
/// retrato.
pub fn de_criatura(nombre: &str) -> Option<&'static Sprite> {
    Some(match nombre {
        "Murciélago" => &MURCIELAGO,
        "Serpiente" => &SERPIENTE,
        "Ladrón" => &LADRON,
        "Gnoll" => &GNOLL,
        "Cofre Sospechoso" => &MIMICO,
        "Rata" => &RATA,
        "Osario" => &OSARIO,
        "Sombra" => &SOMBRA,
        "Devorador" => &DEVORADOR,
        "Coro" => &CORO,
        "Heraldo" => &HERALDO,
        "Osario Mayor" => &OSARIO_MAYOR,
        "Custodio de los Nombres" => &CUSTODIO,
        "Boca del Abismo" => &BOCA,
        "Heraldo Mayor" => &HERALDO_MAYOR,
        crate::bestiary::ARCHIDEMONIO => &ARCHIDEMONIO,
        _ => return None,
    })
}
