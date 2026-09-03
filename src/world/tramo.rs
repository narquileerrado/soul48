//! Los cuatro tramos del descenso.
//!
//! Los 48 pisos se jugaban y se veían idénticos: sólo cambiaban los números.
//! Cada tramo de doce le da al piso una paleta, un pool de criaturas, sus
//! propios susurros y el Guardián que lo cierra, para que bajar se sienta un
//! viaje y no una repetición.

use crate::theme;
use ratatui::style::Color;

/// Un tramo del descenso: doce pisos con identidad propia.
pub struct Tramo {
    pub nombre: &'static str,
    /// Primer y último piso, ambos incluidos.
    pub rango: (u32, u32),
    /// El color de los muros en este tramo.
    pub muro: Color,
    /// El color del suelo en este tramo.
    pub suelo: Color,
    /// Lo que dicen las paredes acá abajo.
    pub susurros: &'static [&'static str],
    /// El Guardián que cierra el tramo, en su último piso.
    pub jefe: &'static str,
    /// Lo que se lee al pisar el primer piso del tramo.
    pub entrada: &'static str,
}

/// De arriba hacia abajo: el umbral queda atrás y el silencio espera.
pub static TRAMOS: [Tramo; 4] = [
    Tramo {
        nombre: "LAS CRIPTAS",
        rango: (1, 12),
        muro: theme::MURO,
        suelo: theme::SUELO,
        susurros: &[
            "Acordaos: vuestra voz fue lo primero que os quitaron.",
            "Los cofres dorados respiran a veces, quando nadie los mira.",
            "Quien diere su sangre al Altar de los Ecos, verá lo que está encubierto.",
            "Arriba aún queda luz. Ninguno sube.",
        ],
        jefe: "Osario Mayor",
        entrada: "Huele el aire todavía a tierra rebuelta. Camposanto fue aquesto.",
    },
    Tramo {
        nombre: "LAS CATACUMBAS",
        rango: (13, 24),
        muro: theme::MURO_CATACUMBA,
        suelo: theme::SUELO_CATACUMBA,
        susurros: &[
            "Contad los sótanos. El número que os falta es el que os aguarda.",
            "Acá abaxo guarda el agua los nombres mejor que la piedra.",
            "El que labró aquestas paredes tampoco tenía voz.",
            "Quarenta y ocho. Vos lo dixistes, que yo no.",
        ],
        jefe: "Guardador de los Nombres",
        entrada: "Suda la piedra. Alguno labró nombres en los muros, y después los borró.",
    },
    Tramo {
        nombre: "EL ABISMO",
        rango: (25, 36),
        muro: theme::MURO_ABISMO,
        suelo: theme::SUELO_ABISMO,
        susurros: &[
            "Ya no baxáis vos: déxanos caer.",
            "Vuestro seso es lo solo que aún no le habéis entregado.",
            "Acá no tienen las sombras quién las haga.",
            "Poco falta. Y aquesto habría de espantaros.",
        ],
        jefe: "Boca del Abismo",
        entrada: "Acabóse la piedra labrada. Lo que pisáis no lo fabricó nadie.",
    },
    Tramo {
        nombre: "EL SILENCIO",
        rango: (37, 48),
        muro: theme::MURO_SILENCIO,
        suelo: theme::SUELO_SILENCIO,
        susurros: &[
            "En el sótano quarenta y ocho aguarda el Archidemonio, con vuestra voz en la boca.",
            "Aun yo hablo más baxo aquí.",
            "Conoceréis vuestra voz en oyéndola. Ahí está el daño.",
            "Ninguna pared deste tramo dize verdad. Ni aquesta tampoco.",
        ],
        jefe: "Pregonero Mayor",
        entrada: "Callan las paredes. Estáis en su casa.",
    },
];

/// El tramo al que pertenece un piso. Los pisos fuera de rango caen en el
/// último: pasado el 48 ya no hay más abismo que el Silencio.
pub fn de_piso(depth: u32) -> &'static Tramo {
    TRAMOS
        .iter()
        .find(|t| depth >= t.rango.0 && depth <= t.rango.1)
        .unwrap_or(&TRAMOS[TRAMOS.len() - 1])
}

/// Índice del tramo de un piso, para indexar los pesos de aparición.
pub fn indice_de_piso(depth: u32) -> usize {
    TRAMOS
        .iter()
        .position(|t| depth >= t.rango.0 && depth <= t.rango.1)
        .unwrap_or(TRAMOS.len() - 1)
}

/// El piso en el que aparece el Guardián de un tramo.
///
/// Es el último piso con jefe del tramo que no sea el final del descenso: el
/// piso 48 ya tiene dueño, así que el Guardián del Silencio se planta en el 42
/// y te deja pasar sabiendo lo que sigue.
pub fn piso_del_guardian(tramo: &Tramo) -> u32 {
    let cada = crate::balance::descenso::CADA_CUANTOS_JEFE;
    let mut piso = tramo.rango.1 - (tramo.rango.1 % cada);
    if piso == crate::balance::descenso::PISO_FINAL {
        piso -= cada;
    }
    piso
}

/// Si en este piso te espera el Guardián nombrado de su tramo.
pub fn cierra_tramo(depth: u32) -> bool {
    TRAMOS.iter().any(|t| piso_del_guardian(t) == depth)
}
