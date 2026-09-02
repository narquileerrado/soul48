//! Menú principal.
//!
//! La pantalla es casi toda oscuridad, que es de lo que trata el juego: hueso y
//! ceniza sobre penumbra, sin un solo borde, todo sobre un eje central. El oro
//! aparece una única vez —la opción elegida— porque es lo que `theme` define
//! que significa: foco, objetivo, selección. Al estar centrado, un marcador al
//! costado rompería la simetría y el color ya alcanza para decir cuál es.

use crate::settings::{Glifos, Settings};
use crate::theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{ListState, Paragraph},
    Frame,
};

/// Datos mínimos de la partida guardada: piso, alma, alma máxima y semilla.
pub type Fragmento = Option<(u32, i32, i32, u64)>;

#[derive(Clone, Copy, PartialEq)]
pub enum MainMenuOption {
    StartGame,
    Bestiary,
    LoadGame,
    Options,
    Quit,
}

impl MainMenuOption {
    /// El orden convencional: empezar, continuar, y recién después lo demás.
    /// El índice del cursor entra por acá, así que reordenar esta lista alcanza.
    const ALL: [MainMenuOption; 5] = [
        MainMenuOption::StartGame,
        MainMenuOption::LoadGame,
        MainMenuOption::Bestiary,
        MainMenuOption::Options,
        MainMenuOption::Quit,
    ];

    pub fn all() -> &'static [MainMenuOption] {
        &Self::ALL
    }
    pub fn as_str(&self) -> &str {
        match self {
            MainMenuOption::StartGame => "DESCENDER AL ABISMO",
            MainMenuOption::Bestiary => "COMPENDIO DE SOMBRAS",
            MainMenuOption::LoadGame => "RECOGER FRAGMENTOS",
            MainMenuOption::Options => "SINTONIZAR ALMA",
            MainMenuOption::Quit => "VOLVER AL SILENCIO",
        }
    }
    pub fn description(&self) -> &str {
        match self {
            MainMenuOption::StartGame => "Bajá desde el umbral hasta el piso 48. Recuperá tu voz.",
            MainMenuOption::Bestiary => "Estudia a los moradores de las profundidades.",
            MainMenuOption::LoadGame => "Continúa una partida guardada anteriormente.",
            MainMenuOption::Options => "Sintoniza la penumbra, los glifos y el guardado.",
            MainMenuOption::Quit => "Abandona el juego y regresa al sistema.",
        }
    }
}

/* ─────────────────────────── el título ─────────────────────────── */

/// Lo que dice el título, en el orden en que se dibuja.
const TITULO: &str = "SOUL 48";
/// Ancho de un glifo en pixeles, más el pixel de separación.
const PASO_GLIFO: usize = 6;
/// Lo que avanza un espacio.
const PASO_ESPACIO: usize = 3;

/// Tipografía de bloques de 5x7 pixeles para el título.
///
/// Reemplaza al logo FIGlet de `@ ! : .`, que dibujaba las letras con un
/// degradado de caracteres y salía embarrado. Devuelve `None` para el espacio.
fn glifo_titulo(c: char) -> Option<[&'static str; 7]> {
    Some(match c {
        'S' => [
            "01110", "10001", "10000", "01110", "00001", "10001", "01110",
        ],
        'O' => [
            "01110", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'U' => [
            "10001", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'L' => [
            "10000", "10000", "10000", "10000", "10000", "10000", "11111",
        ],
        '4' => [
            "00010", "00110", "01010", "10010", "11111", "00010", "00010",
        ],
        '8' => [
            "01110", "10001", "10001", "01110", "10001", "10001", "01110",
        ],
        _ => return None,
    })
}

/// Dibuja el título en cuatro filas de terminal.
///
/// Cada celda pinta dos pixeles verticales con `▀` / `▄` / `█`: la misma técnica
/// de medio bloque que `sprite` usa para los retratos, que deja los pixeles
/// cuadrados en vez de estirados al doble de alto. En modo ascii cae a `#`, `'`
/// y `.`, igual que `Sprite::lineas`.
fn titulo_en_bloques(ascii: bool) -> Vec<Line<'static>> {
    let ancho: usize = TITULO
        .chars()
        .map(|c| {
            if glifo_titulo(c).is_some() {
                PASO_GLIFO
            } else {
                PASO_ESPACIO
            }
        })
        .sum::<usize>()
        .saturating_sub(1);

    // siete filas de letra más una en blanco: cuatro celdas justas
    let mut pixeles = vec![vec![false; ancho]; 8];
    let mut x = 0;
    for c in TITULO.chars() {
        match glifo_titulo(c) {
            None => x += PASO_ESPACIO,
            Some(glifo) => {
                for (py, fila) in glifo.iter().enumerate() {
                    for (px, bit) in fila.chars().enumerate() {
                        if bit == '1' {
                            pixeles[py][x + px] = true;
                        }
                    }
                }
                x += PASO_GLIFO;
            }
        }
    }

    let (lleno, arriba, abajo) = if ascii {
        ('#', '\'', '.')
    } else {
        ('█', '▀', '▄')
    };

    (0..4)
        .map(|celda| {
            let texto: String = (0..ancho)
                .map(
                    |cx| match (pixeles[celda * 2][cx], pixeles[celda * 2 + 1][cx]) {
                        (true, true) => lleno,
                        (true, false) => arriba,
                        (false, true) => abajo,
                        (false, false) => ' ',
                    },
                )
                .collect();
            Line::from(Span::styled(texto, Style::default().fg(theme::HUESO)))
        })
        .collect()
}

/// Letterspacing a la manera de una terminal: separando con espacios.
fn espaciado(texto: &str) -> String {
    texto
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn tenue(texto: String) -> Line<'static> {
    Line::from(Span::styled(
        texto,
        Style::default().fg(theme::CENIZA_HONDA),
    ))
}

fn aire(n: usize) -> Vec<Line<'static>> {
    vec![Line::from(""); n]
}

/* ─────────────────────────── la pantalla ─────────────────────────── */

pub fn ui(f: &mut Frame, menu_state: &mut ListState, fragmento: &Fragmento, ajustes: &Settings) {
    let size = f.size();
    let seleccionado = menu_state.selected().unwrap_or(0);
    let ascii = ajustes.glifos == Glifos::Ascii;

    // La composición vive de su aire. En una terminal baja se compacta antes de
    // recortarse: perder el respiro es mejor que perder una línea.
    let holgado = size.height >= 34;
    let (respiro, entre_opciones) = if holgado { (5, 1) } else { (1, 0) };

    let mut lineas: Vec<Line> = Vec::new();
    lineas.extend(titulo_en_bloques(ascii));
    lineas.extend(aire(1));
    lineas.push(tenue(espaciado("the talking dead")));
    lineas.extend(aire(respiro));

    for (i, opcion) in MainMenuOption::all().iter().enumerate() {
        let estilo = if i == seleccionado {
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::CENIZA)
        };
        lineas.push(Line::from(Span::styled(opcion.as_str(), estilo)));
        if i + 1 < MainMenuOption::all().len() {
            lineas.extend(aire(entre_opciones));
        }
    }

    lineas.extend(aire(if holgado { 3 } else { 1 }));
    let opcion = MainMenuOption::all()[seleccionado.min(MainMenuOption::all().len() - 1)];
    lineas.push(tenue(opcion.description().to_string()));
    lineas.extend(aire(if holgado { 2 } else { 1 }));

    lineas.push(match fragmento {
        Some((piso, hp, max_hp, seed)) => tenue(format!(
            "piso {}   ·   alma {}/{}   ·   semilla {}",
            piso, hp, max_hp, seed
        )),
        None => tenue("sin partida guardada".into()),
    });

    lineas.extend(aire(if holgado { 2 } else { 1 }));
    let (flechas, enter) = if ascii {
        ("^v", "ENTER")
    } else {
        ("↑↓", "⏎")
    };
    lineas.push(tenue(format!("{}        {}        esc", flechas, enter)));

    // centrado vertical: lo que sobra se reparte arriba y abajo
    let alto = lineas.len() as u16;
    let arriba = size.height.saturating_sub(alto) / 2;
    let zona = Rect {
        x: size.x,
        y: size.y + arriba,
        width: size.width,
        height: size.height.saturating_sub(arriba),
    };

    f.render_widget(Paragraph::new(lineas).alignment(Alignment::Center), zona);
}
