//! El catálogo de criaturas: una sola fuente para el juego y el compendio.
//!
//! Antes las estadísticas estaban escritas dos veces —una en `EnemyTemplate`
//! dentro de `world/map_builder.rs` para generar los mobs, otra en `BestiaryEntry`
//! acá para mostrarlos— con los colores RGB copiados a mano en ambos lados,
//! aunque ya existían en `theme`. Cambiar la vida de una serpiente obligaba a
//! acordarse del otro archivo, y el compendio podía mentir sin que nada fallara.

use crate::app::{EnemyAI, StatusEffectType};
use ratatui::style::Color;

/// El jefe final. Su caída termina la corrida, así que el nombre lo consultan
/// tanto la generación del piso 48 como `reap_dead`.
pub const ARCHIDEMONIO: &str = "ARCHIDEMONIO DEL SILENCIO";

/// Objetos que la lógica busca por nombre. Estaban escritos a mano en cada
/// sitio que los comparaba, y bastaba una tilde de más para romperlos.
pub const POCION: &str = "Redoma de Salud";
pub const LLAVE: &str = "Llave de Fierro";

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
    /// Si se la puede calmar pagando cordura en vez de pelearla.
    ///
    /// Era un `name.contains("Ladrón")` suelto en `interaction`: cualquier
    /// cambio de nombre lo rompía sin que nada fallara.
    pub negociable: bool,
    /// Experiencia que deja al caer.
    pub xp: u32,
}

/// El catálogo. Es `static` y no una función que arma un `Vec`: lo consultan
/// el render y cada pulsación de flecha del compendio.
pub static BESTIARIO: [BestiaryEntry; 16] = [
    BestiaryEntry {
        name: "Murciélago de Cripta",
        short_name: "Murciélago",
        scientific_name: "Vespertilio Umbra",
        taxonomy: "Reyno: Animalia | Philo: Chordata | Clase: Mammalia | Orden: Chiroptera",
        description: "Morador de los techos olvidados, no bebe aquesta criatura sangre, sino el eco de los susurros de los muertos. Sus alas, delgadas como pergamino viejo, baten sin ruido en la escuridad. Dízese que son los ojos de aquellos que no pudieron subir.",
        glyph: 'b',
        color: crate::theme::MURCIELAGO,
        base_hp: 6,
        base_damage: (1, 2),
        base_defense: 0,
        behavior: "Errátil. No acomete sino viéndose acorralado, o oliendo flaqueza.",
        ai: EnemyAI::Wandering,
        spawn_weight: [30, 10, 0, 0],
        aplica: None,
        negociable: false,
        xp: 10,
    },
    BestiaryEntry {
        name: "Sierpe de Médula",
        short_name: "Serpiente",
        scientific_name: "Serpens Venenosa",
        taxonomy: "Reyno: Animalia | Philo: Chordata | Clase: Reptilia | Orden: Squamata",
        description: "Fría como el mármol de una sepultura, deslízase la sierpe por las hendiduras de lo real. Su ponçoña no pudre la carne, sino la voluntad, y sume a quien pica en un letargo del que pocos despiertan. Susténtase del calor que les queda a las ánimas que aún laten.",
        glyph: 's',
        color: crate::theme::VERDE_VENENO,
        base_hp: 12,
        base_damage: (2, 4),
        base_defense: 1,
        behavior: "Brava. Tomado el rastro, sigue a su presa sin cansarse jamás.",
        ai: EnemyAI::Melee,
        spawn_weight: [25, 20, 5, 0],
        aplica: Some(StatusEffectType::Poison),
        negociable: false,
        xp: 15,
    },
    BestiaryEntry {
        name: "Ladrón de Ecos",
        short_name: "Ladrón",
        scientific_name: "Homo Furunculus",
        taxonomy: "Reyno: Animalia | Philo: Chordata | Clase: Mammalia | Familia: Hominidae (Degenerado)",
        description: "Hombres fueron antaño, que buscaron hazienda en los sótanos vedados; agora son sombras hambrientas que no conocen sino la codicia. Perdieron su voz y su nombre, y sólo les quedó la maña de esconderse y herir por las espaldas. Temen la luz del espíritu.",
        glyph: 'L',
        color: crate::theme::AZUL_LADRON,
        base_hp: 18,
        base_damage: (2, 5),
        base_defense: 2,
        behavior: "Cauto. Precia más acometer y huir que sustentar la pelea.",
        ai: EnemyAI::Coward,
        spawn_weight: [20, 20, 10, 0],
        aplica: None,
        negociable: true,
        xp: 20,
    },
    BestiaryEntry {
        name: "Gnoll Aullador",
        short_name: "Gnoll",
        scientific_name: "Hyaenanthropus Ferox",
        taxonomy: "Reyno: Animalia | Philo: Chordata | Clase: Mammalia | Orden: Carnivora (Abisal)",
        description: "Guerreros brutos con rostros de hiena, son los carceleros del sótano quarenta y ocho. Su risa es sonido que desgarra el seso. No conocen miedo ni piedad, sino el hambre que nunca se harta y los lleva a devorar hasta el postrer pedaço de sustancia vital.",
        glyph: 'g',
        color: crate::theme::PARDO_GNOLL,
        base_hp: 28,
        base_damage: (4, 7),
        base_defense: 3,
        behavior: "Implacable. Busca la pelea de cara y no vuelve el pie ante el peligro.",
        ai: EnemyAI::Melee,
        spawn_weight: [10, 25, 20, 10],
        aplica: Some(StatusEffectType::Bleed),
        negociable: false,
        xp: 30,
    },
    BestiaryEntry {
        name: "Remedador de Caoba",
        short_name: "Cofre Sospechoso",
        scientific_name: "Mimicus Ligneus",
        taxonomy: "Reyno: Desconocido | Philo: Amorphobionta | Clase: Pseudopoda | Orden: Insidiosa",
        description: "No es cofre, sino lengua que aguarda. Forma de vida parásita que remeda cosas de valor para atraer a los incautos. Su «madera» es cuero endurecido, y sus «goznes», quixadas que muelen el azero mejor templado.",
        glyph: 'C',
        color: crate::theme::COFRE,
        base_hp: 45,
        base_damage: (6, 12),
        base_defense: 5,
        behavior: "Quedo. Aguarda con paciencia a que la curiosidad selle la suerte del caminante.",
        ai: EnemyAI::Stationary,
        spawn_weight: [8, 15, 15, 10],
        aplica: None,
        negociable: false,
        xp: 45,
    },
    BestiaryEntry {
        name: "Rata de Osario",
        short_name: "Rata",
        scientific_name: "Rattus Ossuarius",
        taxonomy: "Reyno: Animalia | Philo: Chordata | Clase: Mammalia | Orden: Rodentia",
        description: "Nunca viene sola ni viene la primera. Vive de lo que dexan los muertos y de lo que dexan los vivos que están por serlo. Sufríanlas los enterradores: mientras hubiese ratas, había qué comer, y aquesto quería dezir que el poço no estaba vazío del todo.",
        glyph: 'r',
        color: crate::theme::PARDO_GNOLL,
        base_hp: 4,
        base_damage: (1, 3),
        base_defense: 0,
        behavior: "En manada. Flaca de una en una; de quatro en quatro, insufrible.",
        ai: EnemyAI::Melee,
        spawn_weight: [25, 8, 0, 0],
        aplica: None,
        negociable: false,
        xp: 6,
    },
    BestiaryEntry {
        name: "Osario Errante",
        short_name: "Osario",
        scientific_name: "Ossuarium Ambulans",
        taxonomy: "Reyno: Desconocido | Philo: Osteomorpha | Clase: Congregata | Orden: Perambulans",
        description: "No es un esqueleto: son muchos, mal repartidos. Quando una cripta se llena, aprenden los huessos a acomodarse solos, y lo que se levanta camina con más piernas de las que le tocan. Poco aprovecha golpearlo: menester es desarmarlo.",
        glyph: 'O',
        color: crate::theme::HUESO,
        base_hp: 40,
        base_damage: (3, 6),
        base_defense: 6,
        behavior: "Tardo e implacable. No le hurtaréis el cuerpo dos vezes en un mesmo corredor.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 22, 12, 5],
        aplica: None,
        negociable: false,
        xp: 40,
    },
    BestiaryEntry {
        name: "Sombra Muda",
        short_name: "Sombra",
        scientific_name: "Umbra Tacita",
        taxonomy: "Reyno: Desconocido | Philo: Aphotica | Clase: Incorporea | Orden: Silentes",
        description: "Sombra sin nadie que la haga. Muévese más apriessa de lo que el ojo alcança, y no hace ruido al hazerlo, porque no tiene con qué. Dízese que fueron caminantes que llegaron hasta aquí y determinaron quedarse quedos para siempre.",
        glyph: 'v',
        color: crate::theme::VIOLETA_APAGADO,
        base_hp: 14,
        base_damage: (5, 9),
        base_defense: 1,
        behavior: "Presta y quebradiza. Os alcança antes que la veáis venir.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 5, 25, 20],
        aplica: Some(StatusEffectType::Bleed),
        negociable: false,
        xp: 35,
    },
    BestiaryEntry {
        name: "Devorador de Ecos",
        short_name: "Devorador",
        scientific_name: "Vorax Resonantiae",
        taxonomy: "Reyno: Desconocido | Philo: Amorphobionta | Clase: Absorbens | Orden: Vorax",
        description: "No os muerde: os escucha. Cada vez que se allega, algo de lo que ibais a dezir dexa de estar. Los que escaparon de uno cuentan que lo peor no fue el encuentro, sino hallar después las palabras que ya no tenían.",
        glyph: 'e',
        color: crate::theme::VIOLETA,
        base_hp: 30,
        base_damage: (2, 4),
        base_defense: 3,
        behavior: "Sigue en silencio. Lo que os quita no se cura con redoma ninguna.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 22, 18],
        aplica: Some(StatusEffectType::Confusion),
        negociable: false,
        xp: 50,
    },
    BestiaryEntry {
        name: "Coro de Lamentos",
        short_name: "Coro",
        scientific_name: "Chorus Lamentorum",
        taxonomy: "Reyno: Desconocido | Philo: Aphotica | Clase: Resonantia | Orden: Plurivox",
        description: "Muchas gargantas y boca ninguna. Canta de lexos, y lo que canta duele adonde estéis, sin haber menester tocaros. Es lo más parecido a una plática que hallaréis en aqueste tramo, y es de un solo lado.",
        glyph: 'c',
        color: crate::theme::AZUL_APAGADO,
        base_hp: 26,
        base_damage: (6, 10),
        base_defense: 2,
        behavior: "Quedo. No se allega: no lo ha menester.",
        ai: EnemyAI::Stationary,
        spawn_weight: [0, 0, 8, 25],
        aplica: None,
        negociable: false,
        xp: 55,
    },
    BestiaryEntry {
        name: "Pregonero del Silencio",
        short_name: "Pregonero",
        scientific_name: "Praeco Silentii",
        taxonomy: "Reyno: Desconocido | Philo: Aphotica | Clase: Ministra | Orden: Praecones",
        description: "El que pregona lo que viene, aunque no lo diga. Sirve al Archidemonio y trae puesta una parte de su oficio: por donde passa, dexa la luz de avisar. Llenos están dellos los postreros sótanos, aguardando que lleguéis para no deziros nada.",
        glyph: 'H',
        color: crate::theme::ROJO_ALTAR,
        base_hp: 34,
        base_damage: (7, 12),
        base_defense: 4,
        behavior: "Bravo. Os apaga la vista antes de acabar la obra.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 5, 28],
        aplica: Some(StatusEffectType::Blindness),
        negociable: false,
        xp: 70,
    },
    /* --- los cinco jefes. Peso 0 en todos los tramos: no los pone el spawn
       aleatorio sino `MapBuilder`, en el piso que les toca. Están acá para
       tener ficha en el Compendio y para que su experiencia salga de la misma
       tabla que la del resto. --- */
    BestiaryEntry {
        name: "Osario Mayor",
        short_name: "Osario Mayor",
        scientific_name: "Ossuarium Rex",
        taxonomy: "Reyno: Desconocido | Philo: Osteomorpha | Clase: Congregata | Orden: Regens",
        description: "Lo que queda quando una cripta entera determina levantarse junta. Guarda la salida de Las Criptas, y no por mandado de nadie: es sencillamente lo mayor que hay en doze sótanos, y quedóse donde había sitio.",
        glyph: 'B',
        color: crate::theme::ROJO_ALTAR,
        base_hp: 86,
        base_damage: (11, 16),
        base_defense: 6,
        behavior: "Guardián del primer tramo. Cierra el sótano doze.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 0, 0],
        aplica: None,
        negociable: false,
        xp: 120,
    },
    BestiaryEntry {
        name: "Guardador de los Nombres",
        short_name: "Guardador de los Nombres",
        scientific_name: "Custos Nominum",
        taxonomy: "Reyno: Desconocido | Philo: Aphotica | Clase: Archivaria | Orden: Custodes",
        description: "Fue el que borró los nombres de las paredes, uno a uno, y quedóse guardando la obra. El vuestro le es sabido. Que no pueda dezirlo es lo solo que os está salvando.",
        glyph: 'B',
        color: crate::theme::ROJO_ALTAR,
        base_hp: 122,
        base_damage: (17, 22),
        base_defense: 8,
        behavior: "Guardián del segundo tramo. Cierra el sótano veinte y quatro.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 0, 0],
        aplica: Some(StatusEffectType::Confusion),
        negociable: false,
        xp: 200,
    },
    BestiaryEntry {
        name: "Boca del Abismo",
        short_name: "Boca del Abismo",
        scientific_name: "Os Profundi",
        taxonomy: "Reyno: Desconocido | Philo: Amorphobionta | Clase: Vorax | Orden: Fauces",
        description: "No es que more en el Abismo: es la parte del Abismo que se tomó el trabajo de tener forma. Cierra el tramo porque más abaxo ya no es menester que nada la tenga.",
        glyph: 'B',
        color: crate::theme::ROJO_ALTAR,
        base_hp: 158,
        base_damage: (23, 28),
        base_defense: 11,
        behavior: "Guardián del tercer tramo. Cierra el sótano treinta y seis.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 0, 0],
        aplica: Some(StatusEffectType::Bleed),
        negociable: false,
        xp: 320,
    },
    BestiaryEntry {
        name: "Pregonero Mayor",
        short_name: "Pregonero Mayor",
        scientific_name: "Praeco Maximus",
        taxonomy: "Reyno: Desconocido | Philo: Aphotica | Clase: Ministra | Orden: Praecones",
        description: "El postrero que hallaréis antes del fin, y el solo que sabe puntualmente lo que estáis por oír. Pregona al Archidemonio callándose, que es el modo más honrado que tiene de pregonarlo.",
        glyph: 'B',
        color: crate::theme::ROJO_ALTAR,
        base_hp: 194,
        base_damage: (29, 34),
        base_defense: 13,
        behavior: "Guardián del postrer tramo. Cierra el sótano quarenta y ocho... casi.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 0, 0],
        aplica: Some(StatusEffectType::Blindness),
        negociable: false,
        xp: 450,
    },
    BestiaryEntry {
        name: "Archidemonio del Silencio",
        short_name: ARCHIDEMONIO,
        scientific_name: "Daemon Silentii",
        taxonomy: "Reyno: Desconocido | Philo: Innominata | Clase: Unica | Orden: Unica",
        description: "Quitóos la voz y quedóse con ella quarenta y ocho sótanos más abaxo, adonde nadie había de ir a buscarla. Erró en una sola cosa, y es la que estáis por mostrarle. No tiene ficha cumplida porque ninguno que lo viese de cerca tornó a escrebir.",
        glyph: 'D',
        color: crate::theme::VIOLETA,
        base_hp: 150,
        base_damage: (8, 16),
        base_defense: 6,
        behavior: "El fin del descenso. Sótano quarenta y ocho.",
        ai: EnemyAI::Melee,
        spawn_weight: [0, 0, 0, 0],
        aplica: None,
        negociable: false,
        xp: 500,
    },
];

/// Experiencia que deja una criatura, buscada por el nombre con el que aparece
/// en el mapa. Los jefes no están en el compendio y tienen su propia tabla.
pub fn xp_de(nombre: &str) -> u32 {
    if let Some(e) = BESTIARIO.iter().find(|e| e.short_name == nombre) {
        return e.xp;
    }
    // los jefes ya están en el catálogo: no hay más tabla aparte
    match nombre {
        n if n.starts_with("Eco del ") => 80,
        _ => 15,
    }
}

/// Si a esta criatura se la puede calmar en vez de pelearla.
pub fn es_negociable(nombre: &str) -> bool {
    BESTIARIO
        .iter()
        .any(|e| e.short_name == nombre && e.negociable)
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
