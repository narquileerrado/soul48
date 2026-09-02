use ratatui::style::Color;

/// Define la estructura de información para una entrada en el compendio de criaturas.
pub struct BestiaryEntry {
    pub name: &'static str,
    pub scientific_name: &'static str,
    pub taxonomy: &'static str,
    pub description: &'static str,
    pub glyph: char,
    pub color: Color,
    pub base_hp: i32,
    pub base_damage: (i32, i32),
    pub base_defense: i32,
    pub behavior: &'static str,
}

/// Retorna la colección completa de criaturas documentadas en el juego.
pub fn get_bestiary() -> Vec<BestiaryEntry> {
    vec![
        BestiaryEntry {
            name: "Murciélago de Cripta",
            scientific_name: "Vespertilio Umbra",
            taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Mammalia | Orden: Chiroptera",
            description: "Morador de los techos olvidados, esta criatura no bebe sangre, sino el eco de los susurros de los muertos. Sus alas, finas como pergamino antiguo, baten sin sonido en la oscuridad absoluta. Se dice que son los ojos de aquellos que no pudieron ascender.",
            glyph: 'b',
            color: Color::DarkGray,
            base_hp: 6,
            base_damage: (1, 2),
            base_defense: 0,
            behavior: "Errático. Ataca solo cuando se siente acorralado o percibe debilidad.",
        },
        BestiaryEntry {
            name: "Serpiente de Médula",
            scientific_name: "Serpens Venenosa",
            taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Reptilia | Orden: Squamata",
            description: "Fría como el mármol de una tumba, la serpiente se desliza entre las grietas de la realidad. Su veneno no pudre la carne, sino la voluntad, sumiendo a su víctima en un letargo del que pocos despiertan. Se alimentan del calor residual de las almas que aún palpitan.",
            glyph: 's',
            color: Color::Green,
            base_hp: 12,
            base_damage: (2, 4),
            base_defense: 1,
            behavior: "Agresiva. Persigue a su presa incansablemente una vez detectado el rastro.",
        },
        BestiaryEntry {
            name: "Ladrón de Ecos",
            scientific_name: "Homo Furunculus",
            taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Mammalia | Familia: Hominidae (Degenerado)",
            description: "Antaño hombres que buscaron fortuna en los niveles prohibidos, ahora son sombras famélicas que solo conocen la codicia. Han perdido su voz y su nombre, conservando únicamente la habilidad de ocultarse y herir por la espalda. Temen a la luz del espíritu.",
            glyph: 'L',
            color: Color::Blue,
            base_hp: 18,
            base_damage: (2, 5),
            base_defense: 2,
            behavior: "Cauto. Prefiere atacar y huir, evitando el enfrentamiento directo.",
        },
        BestiaryEntry {
            name: "Gnoll Aullador",
            scientific_name: "Hyaenanthropus Ferox",
            taxonomy: "Reino: Animalia | Filo: Chordata | Clase: Mammalia | Orden: Carnivora (Abisal)",
            description: "Guerreros brutales con rostros de hiena, son los carceleros del piso 48. Su risa es un sonido que desgarra la cordura. No conocen el miedo ni la piedad, solo el hambre insaciable que los impulsa a devorar hasta el último fragmento de esencia vital.",
            glyph: 'g',
            color: Color::Rgb(150, 75, 0),
            base_hp: 28,
            base_damage: (4, 7),
            base_defense: 3,
            behavior: "Implacable. Busca el combate directo y no retrocede ante el peligro.",
        },
        BestiaryEntry {
            name: "Mímico de Caoba",
            scientific_name: "Mimicus Ligneus",
            taxonomy: "Reino: Desconocido | Filo: Amorphobionta | Clase: Pseudopoda | Orden: Insidiosa",
            description: "No es un cofre, sino una lengua que espera. Una forma de vida parásita que imita objetos de valor para atraer a los incautos. Su 'madera' es en realidad piel endurecida, y sus 'bisagras' son mandíbulas capaces de triturar el acero más templado.",
            glyph: 'C',
            color: Color::Yellow,
            base_hp: 45,
            base_damage: (6, 12),
            base_defense: 5,
            behavior: "Estático. Espera pacientemente a que la curiosidad selle el destino del viajero.",
        },
    ]
}
