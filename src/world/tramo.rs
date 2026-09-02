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
            "Recuerda... tu voz fue lo primero que te robaron.",
            "Los cofres dorados a veces respiran cuando no los miras.",
            "Ofrecer tu sangre al Altar de Ecos revelará la verdad oculta.",
            "Arriba todavía hay luz. Nadie sube.",
        ],
        jefe: "Osario Mayor",
        entrada: "El aire todavía huele a tierra removida. Esto fue un cementerio.",
    },
    Tramo {
        nombre: "LAS CATACUMBAS",
        rango: (13, 24),
        muro: theme::MURO_CATACUMBA,
        suelo: theme::SUELO_CATACUMBA,
        susurros: &[
            "Contá los pisos. El número que te falta es el que te espera.",
            "Acá abajo el agua guarda mejor los nombres que la piedra.",
            "El que grabó estas paredes tampoco tenía voz.",
            "Cuarenta y ocho. Lo dijiste vos, no yo.",
        ],
        jefe: "Custodio de los Nombres",
        entrada: "La piedra suda. Alguien grabó nombres en las paredes y después los tachó.",
    },
    Tramo {
        nombre: "EL ABISMO",
        rango: (25, 36),
        muro: theme::MURO_ABISMO,
        suelo: theme::SUELO_ABISMO,
        susurros: &[
            "Ya no estás bajando. Te están dejando caer.",
            "Tu cordura es lo único que todavía no le entregaste.",
            "Acá las sombras no tienen quién las proyecte.",
            "Falta poco. Eso debería asustarte.",
        ],
        jefe: "Boca del Abismo",
        entrada: "Se terminó la piedra tallada. Lo que pisás no lo construyó nadie.",
    },
    Tramo {
        nombre: "EL SILENCIO",
        rango: (37, 48),
        muro: theme::MURO_SILENCIO,
        suelo: theme::SUELO_SILENCIO,
        susurros: &[
            "En el piso 48, el Archidemonio aguarda con tu voz en la boca.",
            "Hasta yo hablo más bajo acá.",
            "Vas a reconocer tu voz cuando la escuches. Ese es el problema.",
            "Ninguna pared de este tramo dice la verdad. Tampoco esta.",
        ],
        jefe: "Heraldo Mayor",
        entrada: "Las paredes dejan de hablar. Estás en su casa.",
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

/// Si un piso es el último de su tramo, y por lo tanto lleva su Guardián.
pub fn cierra_tramo(depth: u32) -> bool {
    TRAMOS.iter().any(|t| t.rango.1 == depth)
}
