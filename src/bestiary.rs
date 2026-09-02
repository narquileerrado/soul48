//! El catálogo de criaturas: una sola fuente para el juego y el compendio.
//!
//! Antes las estadísticas estaban escritas dos veces —una en `EnemyTemplate`
//! dentro de `map_builder.rs` para generar los mobs, otra en `BestiaryEntry`
//! acá para mostrarlos— con los colores RGB copiados a mano en ambos lados,
//! aunque ya existían en `theme`. Cambiar la vida de una serpiente obligaba a
//! acordarse del otro archivo, y el compendio podía mentir sin que nada fallara.

use crate::app::{EnemyAI, StatusEffectType};
use ratatui::style::Color;

/// Todo lo que se sabe de una criatura: lo que la mueve y lo que se cuenta de ella.
pub struct BestiaryEntry {
    pub name: &'static str,
    /// Nombre corto con el que aparece en el mapa y en el historial.
    pub short_name: &'static str,
    pub scientific_name: &'static str,
    pub taxonomy: &'static str,
    pub description: &'static str,
    pub glyph: char,
    pub color: Color,
    pub base_hp: i32,
    pub base_damage: (i32, i32),
    pub base_defense: i32,
    pub behavior: &'static str,
    /// Cómo se comporta en el mapa.
    pub ai: EnemyAI,
    /// Peso de aparición en cada tramo, de arriba hacia abajo. Un `0` quiere
    /// decir que la criatura no vive en ese tramo.
    pub spawn_weight: [i32; 4],
    /// El efecto que deja al golpear, si deja alguno.
    pub aplica: Option<StatusEffectType>,
    /// Experiencia que deja al caer.
    pub xp: u32,
}

/// El catálogo. Es `static` y no una función que arma un `Vec`: lo consultan
/// el render y cada pulsación de flecha del compendio.
pub static BESTIARIO: [BestiaryEntry; 11] = [
    BestiaryEntry {
        name: "Murciélago de Cripta",
        short_name: "Murciélago",
        scientific_name: "Vespertilio Umbra",
        taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Mammalia | Orden: Chiroptera",
        description: "Morador de los techos olvidados, esta criatura no bebe sangre, sino el eco de los susurros de los muertos. Sus alas, finas como pergamino antiguo, baten sin sonido en la oscuridad absoluta. Se dice que son los ojos de aquellos que no pudieron ascender.",
        glyph: 'b',
        color: crate::theme::MURCIELAGO,
        base_hp: 6,
        base_damage: (1, 2),
        base_defense: 0,
        behavior: "Errático. Ataca solo cuando se siente acorralado o percibe debilidad.",
        ai: EnemyAI::Wandering,
        spawn_weight: [30, 10, 0, 0],
        aplica: None,
        xp: 10,
    },
    BestiaryEntry {
        name: "Serpiente de Médula",
        short_name: "Serpiente",
        scientific_name: "Serpens Venenosa",
        taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Reptilia | Orden: Squamata",
        description: "Fría como el mármol de una tumba, la serpiente se desliza entre las grietas de la realidad. Su veneno no pudre la carne, sino la voluntad, sumiendo a su víctima en un letargo del que pocos despiertan. Se alimentan del calor residual de las almas que aún palpitan.",
        glyph: 's',
        color: crate::theme::VERDE_VENENO,
        base_hp: 12,
        base_damage: (2, 4),
        base_defense: 1,
        behavior: "Agresiva. Persigue a su presa incansablemente una vez detectado el rastro.",
        ai: EnemyAI::Melee,
        spawn_weight: [25, 20, 5, 0],
        aplica: Some(StatusEffectType::Poison),
        xp: 15,
    },
    BestiaryEntry {
        name: "Ladrón de Ecos",
        short_name: "Ladrón",
        scientific_name: "Homo Furunculus",
        taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Mammalia | Familia: Hominidae (Degenerado)",
        description: "Antaño hombres que buscaron fortuna en los niveles prohibidos, ahora son sombras famélicas que solo conocen la codicia. Han perdido su voz y su nombre, conservando únicamente la habilidad de ocultarse y herir por la espalda. Temen a la luz del espíritu.",
        glyph: 'L',
        color: crate::theme::AZUL_LADRON,
        base_hp: 18,
        base_damage: (2, 5),
        base_defense: 2,
        behavior: "Cauto. Prefiere atacar y huir, evitando el enfrentamiento directo.",
        ai: EnemyAI::Coward,
        spawn_weight: [20, 20, 10, 0],
        aplica: None,
        xp: 20,
    },
    BestiaryEntry {
        name: "Gnoll Aullador",
        short_name: "Gnoll",
        scientific_name: "Hyaenanthropus Ferox",
        taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Mammalia | Orden: Carnivora (Abisal)",
        description: "Guerreros brutales con rostros de hiena, son los carceleros del piso 48. Su risa es un sonido que desgarra la cordura. No conocen el miedo ni la piedad, solo el hambre insaciable que los impulsa a devorar hasta el último fragmento de esencia vital.",
        glyph: 'g',
        color: crate::theme::PARDO_GNOLL,
        base_hp: 28,
        base_damage: (4, 7),
        base_defense: 3,
        behavior: "Implacable. Busca el combate directo y no retrocede ante el peligro.",
        ai: EnemyAI::Melee,
        spawn_weight: [10, 25, 20, 10],
        aplica: Some(StatusEffectType::Bleed),
        xp: 30,
    },
    BestiaryEntry {
        name: "Mímico de Caoba",
        short_name: "Cofre Sospechoso",
        scientific_name: "Mimicus Ligneus",
        taxonomy: "Reino: Desconocido | Filo: Amorphobionta | Clase: Pseudopoda | Orden: Insidiosa",
        description: "No es un cofre, sino una lengua que espera. Una forma de vida parásita que imita objetos de valor para atraer a los incautos. Su 'madera' es en realidad piel endurecida, y sus 'bisagras' son mandíbulas capaces de triturar el acero más templado.",
        glyph: 'C',
        color: crate::theme::COFRE,
        base_hp: 45,
        base_damage: (6, 12),
        base_defense: 5,
        behavior: "Estático. Espera pacientemente a que la curiosidad selle el destino del viajero.",
        ai: EnemyAI::Stationary,
        spawn_weight: [8, 15, 15, 10],
        aplica: None,
        xp: 45,
    },
    BestiaryEntry {
        name: "Rata de Osario",
        short_name: "Rata",
        scientific_name: "Rattus Ossuarius",
        taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Mammalia | Orden: Rodentia",
        description: "Nunca viene sola y nunca viene primero. Vive de lo que dejan los muertos y de lo que dejan los vivos que están por serlo. Los enterradores las toleraban: mientras hubiera ratas, había algo que comer, y eso quería decir que el pozo no estaba vacío del todo.",
        glyph: 'r',
        color: crate::theme::PARDO_GNOLL,
        base_hp: 4,
        base_damage: (1, 3),
        base_defense: 0,
        behavior: "En manada. Débil sola, insoportable de a cuatro.",
        ai: EnemyAI::Melee,
        spawn_weight: [25, 8, 0, 0],
        aplica: None,
        xp: 6,
    },
    BestiaryEntry {
        name: "Osario Errante",
        short_name: "Osario",
        scientific_name: "Ossuarium Ambulans",
        taxonomy: "Reino: Desconocido | Filo: Osteomorpha | Clase: Congregata | Orden: Perambulans",
        description: "No es un esqueleto: son varios, mal repartidos. Cuando una cripta se llena, los huesos aprenden a acomodarse solos y lo que se levanta camina con más piernas de las que le tocan. Golpearlo sirve de poco; hay que desarmarlo.",
        glyph: 'O',
        color: crate::theme::HUESO,
        base_hp: 40,
        base_damage: (3, 6),
        base_defense: 6,
        behavior: "Lento e implacable. No lo vas a esquivar dos veces en el mismo pasillo.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 22, 12, 5],
        aplica: None,
        xp: 40,
    },
    BestiaryEntry {
        name: "Sombra Muda",
        short_name: "Sombra",
        scientific_name: "Umbra Tacita",
        taxonomy: "Reino: Desconocido | Filo: Aphotica | Clase: Incorporea | Orden: Silentes",
        description: "Una sombra sin nadie que la proyecte. Se mueve más rápido de lo que deberías poder mirar y no hace ruido al hacerlo, porque no tiene con qué. Se dice que fueron viajeros que llegaron hasta acá y decidieron quedarse quietos para siempre.",
        glyph: 'v',
        color: crate::theme::VIOLETA_APAGADO,
        base_hp: 14,
        base_damage: (5, 9),
        base_defense: 1,
        behavior: "Rápida y frágil. Te llega antes de que la veas venir.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 5, 25, 20],
        aplica: Some(StatusEffectType::Bleed),
        xp: 35,
    },
    BestiaryEntry {
        name: "Devorador de Ecos",
        short_name: "Devorador",
        scientific_name: "Vorax Resonantiae",
        taxonomy: "Reino: Desconocido | Filo: Amorphobionta | Clase: Absorbens | Orden: Vorax",
        description: "No te muerde: te escucha. Cada vez que se acerca, algo de lo que ibas a decir deja de estar. Los que sobrevivieron a uno cuentan que lo peor no fue el encuentro, sino descubrir después las palabras que ya no tenían.",
        glyph: 'e',
        color: crate::theme::VIOLETA,
        base_hp: 30,
        base_damage: (2, 4),
        base_defense: 3,
        behavior: "Persigue en silencio. Lo que te saca no se cura con una poción.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 22, 18],
        aplica: Some(StatusEffectType::Confusion),
        xp: 50,
    },
    BestiaryEntry {
        name: "Coro de Lamentos",
        short_name: "Coro",
        scientific_name: "Chorus Lamentorum",
        taxonomy: "Reino: Desconocido | Filo: Aphotica | Clase: Resonantia | Orden: Plurivox",
        description: "Muchas gargantas y ninguna boca. Canta desde lejos y lo que canta duele donde estés parado, sin necesidad de tocarte. Es lo más parecido a una conversación que vas a encontrar en este tramo, y es de un solo lado.",
        glyph: 'c',
        color: crate::theme::AZUL_APAGADO,
        base_hp: 26,
        base_damage: (6, 10),
        base_defense: 2,
        behavior: "Estático. No se acerca: no le hace falta.",
        ai: EnemyAI::Stationary,
        spawn_weight: [0, 0, 8, 25],
        aplica: None,
        xp: 55,
    },
    BestiaryEntry {
        name: "Heraldo del Silencio",
        short_name: "Heraldo",
        scientific_name: "Praeco Silentii",
        taxonomy: "Reino: Desconocido | Filo: Aphotica | Clase: Ministra | Orden: Praecones",
        description: "El que anuncia lo que viene, aunque no lo diga. Sirve al Archidemonio y lleva puesta una parte de su trabajo: donde pasa, la luz deja de informar. Los últimos pisos están llenos de ellos, esperando que llegues para no decirte nada.",
        glyph: 'H',
        color: crate::theme::ROJO_ALTAR,
        base_hp: 34,
        base_damage: (7, 12),
        base_defense: 4,
        behavior: "Agresivo. Te apaga la vista antes de terminar el trabajo.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 5, 28],
        aplica: Some(StatusEffectType::Blindness),
        xp: 70,
    },
];

/// El jefe final. Su caída termina la corrida, así que el nombre lo consultan
/// tanto la generación del piso 48 como `reap_dead`.
pub const ARCHIDEMONIO: &str = "ARCHIDEMONIO DEL SILENCIO";

/// Experiencia que deja una criatura, buscada por el nombre con el que aparece
/// en el mapa. Los jefes no están en el compendio y tienen su propia tabla.
pub fn xp_de(nombre: &str) -> u32 {
    if let Some(e) = BESTIARIO.iter().find(|e| e.short_name == nombre) {
        return e.xp;
    }
    match nombre {
        ARCHIDEMONIO => 500,
        n if n.starts_with("Guardián") => 80,
        _ => 15,
    }
}

/// El efecto que deja una criatura al golpear, si deja alguno.
pub fn efecto_de(nombre: &str) -> Option<StatusEffectType> {
    BESTIARIO
        .iter()
        .find(|e| e.short_name == nombre)
        .and_then(|e| e.aplica.clone())
}

/// La colección completa, para el compendio.
pub fn get_bestiary() -> &'static [BestiaryEntry] {
    &BESTIARIO
}
