//! Números que definen cómo se siente el juego, en un solo lugar.
//!
//! Antes estaban sueltos entre la lógica: el daño del rayo escrito a mano en
//! `use_item`, el costo del altar en `interact_with_entity`, la experiencia de
//! cada criatura en un `match` sobre su nombre. Acá se ajusta el balance sin
//! leer la lógica, y la lógica se lee sin tropezar con números mágicos.

/// Combate cuerpo a cuerpo.
pub mod combate {
    /// Daño de los puños cuando no hay arma equipada.
    pub const PUNOS: (i32, i32) = (1, 3);
    /// Probabilidad de golpe crítico.
    pub const PROB_CRITICO: f64 = 0.2;
    /// Multiplicador del golpe crítico.
    pub const MULT_CRITICO: i32 = 2;
    /// Todo golpe hace al menos esto, por mucha defensa que haya enfrente.
    pub const DANO_MINIMO: i32 = 1;
    /// Daño de la embestida cuando el enemigo no tiene hacia dónde retroceder.
    pub const EMBESTIDA_CONTRA_MURO: i32 = 5;
    /// El bloqueo divide el daño recibido por esto.
    pub const DIVISOR_PARRY: i32 = 2;
}

/// Cómo se traducen los atributos del héroe en números de combate.
pub mod atributos {
    /// Fuerza que trae el héroe de fábrica; sólo el excedente suma daño, así
    /// que empezar la partida no regala golpes gratis.
    pub const BASE: i32 = 5;
    /// Cada punto de fuerza por encima de la base suma este daño.
    pub const DANO_POR_FUERZA: i32 = 1;
    /// Cada punto de agilidad por encima de la base da esta probabilidad de
    /// esquivar un golpe.
    pub const ESQUIVA_POR_AGILIDAD: f64 = 0.02;
    /// Tope de esquiva: nunca sos intocable.
    pub const ESQUIVA_MAXIMA: f64 = 0.35;
    /// Cada punto de voluntad por encima de la base recorta el desgaste de
    /// cordura en esta proporción.
    pub const AGUANTE_POR_VOLUNTAD: f64 = 0.04;
}

/// Cordura: el medidor de la voz.
pub mod cordura {
    /// Probabilidad base de perder un punto de cordura por turno.
    pub const PROB_DESGASTE: f64 = 0.15;
    /// Cordura que cuesta calmar a un espíritu.
    pub const COSTO_NEGOCIACION: i32 = 10;
    /// Cordura mínima para poder negociar.
    pub const MINIMA_PARA_NEGOCIAR: i32 = 20;
    /// Por debajo de esta cordura empiezan las alucinaciones.
    pub const UMBRAL_ALUCINACION: i32 = 25;
}

/// Experiencia y subida de nivel.
pub mod progresion {
    /// Experiencia necesaria para el nivel 2.
    pub const XP_PRIMER_NIVEL: u32 = 50;
    /// Cada nivel exige esta proporción más que el anterior.
    pub const FACTOR_XP: f32 = 1.5;
    /// Vida máxima que suma cada nivel.
    pub const VIDA_POR_NIVEL: i32 = 5;
    /// Cordura máxima que suma cada nivel.
    pub const CORDURA_POR_NIVEL: i32 = 10;
    /// Vida máxima inicial del héroe.
    pub const VIDA_INICIAL: i32 = 20;
    /// Cordura máxima inicial del héroe.
    pub const CORDURA_INICIAL: i32 = 100;
}

/// Objetos consumibles.
pub mod objetos {
    /// Vida que devuelve una poción.
    pub const CURA_POCION: i32 = 15;
    /// Daño del Pergamino de Rayo y su alcance en distancia Manhattan.
    pub const RAYO: (i32, isize) = (12, 5);
    /// Daño de la Bola de Fuego y su alcance.
    pub const BOLA_DE_FUEGO: (i32, isize) = (15, 3);
    /// Turnos que dura la invisibilidad.
    pub const TURNOS_INVISIBLE: usize = 8;
    /// Intentos de encontrar destino para el teletransporte.
    pub const INTENTOS_TELEPORT: usize = 100;
    /// Slots del inventario: las teclas son 1-9.
    pub const SLOTS_INVENTARIO: usize = 9;
}

/// Peligros del terreno y presencias.
pub mod terreno {
    /// Daño de la trampa de pinchos.
    pub const PINCHOS: i32 = 4;
    /// Daño del pozo de ácido y el veneno que deja (turnos, daño por turno).
    pub const ACIDO: (i32, usize, i32) = (6, 3, 2);
    /// Daño del fuego y la quemadura que deja (turnos, daño por turno).
    pub const FUEGO: (i32, usize, i32) = (8, 2, 3);
    /// Vida que cobra el Altar de Ecos por revelar el piso.
    pub const COSTO_ALTAR: i32 = 5;
}

/// Percepción y memoria.
pub mod percepcion {
    /// Radio del campo de visión del héroe.
    pub const RADIO_FOV: isize = 6;
    /// Distancia a la que un enemigo dormido se despierta.
    pub const DISTANCIA_DESPERTAR: isize = 4;
    /// Distancia a la que un enemigo errante detecta al héroe con línea de visión.
    pub const DISTANCIA_DETECCION: isize = 6;
    /// Distancia a la que un enemigo agresivo pierde el rastro.
    pub const DISTANCIA_PERDER_RASTRO: isize = 10;
    /// Mensajes que retiene el historial, independiente de cuántos se muestren.
    pub const TOPE_HISTORIAL: usize = 200;
}
