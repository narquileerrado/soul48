//! El catálogo de criaturas: una sola fuente para el juego y el compendio.
//!
//! Antes las estadísticas estaban escritas dos veces —una en `EnemyTemplate`
//! dentro de `map_builder.rs` para generar los mobs, otra en `BestiaryEntry`
//! acá para mostrarlos— con los colores RGB copiados a mano en ambos lados,
//! aunque ya existían en `theme`. Cambiar la vida de una serpiente obligaba a
//! acordarse del otro archivo, y el compendio podía mentir sin que nada fallara.

use crate::app::EnemyAI;
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
    /// Peso relativo de aparición.
    pub spawn_weight: i32,
    /// Experiencia que deja al caer.
    pub xp: u32,
}

/// El catálogo. Es `static` y no una función que arma un `Vec`: lo consultan
/// el render y cada pulsación de flecha del compendio.
pub static BESTIARIO: [BestiaryEntry; 5] = [
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
        spawn_weight: 30,
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
        spawn_weight: 25,
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
        spawn_weight: 20,
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
        spawn_weight: 15,
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
        spawn_weight: 10,
        xp: 45,
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

/// La colección completa, para el compendio.
pub fn get_bestiary() -> &'static [BestiaryEntry] {
    &BESTIARIO
}
