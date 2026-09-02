//! Piezas de dibujo compartidas por todas las pantallas.
//!
//! Todo se apoya en `theme`: cada color tiene un significado y uno solo.

use crate::app::{App, LogType, StatusEffectType};
use crate::settings::Glifos;
use crate::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub const MUROS: [char; 12] = ['║', '═', '╚', '╔', '╝', '╗', '╠', '╣', '╩', '╦', '╬', '■'];

/* ───────────────────────────── utilidades ───────────────────────────── */

pub fn marco(foco: bool) -> Style {
    Style::default().fg(if foco { theme::ORO } else { theme::ORO_APAGADO })
}
pub fn titulo(foco: bool) -> Style {
    if foco {
        Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::CENIZA)
    }
}

pub fn pad_der(s: &str, n: usize) -> String {
    let len = s.chars().count();
    if len >= n {
        s.chars().take(n).collect()
    } else {
        format!("{}{}", s, " ".repeat(n - len))
    }
}
pub fn pad_izq(s: &str, n: usize) -> String {
    let len = s.chars().count();
    if len >= n {
        s.chars().take(n).collect()
    } else {
        format!("{}{}", " ".repeat(n - len), s)
    }
}
pub fn recortar(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else if n <= 1 {
        s.chars().take(n).collect()
    } else {
        let mut t: String = s.chars().take(n - 1).collect();
        t.push('…');
        t
    }
}

/// Cómo se pinta una barra: el par de tonos y el juego de glifos.
#[derive(Clone, Copy)]
pub struct EstiloBarra {
    pub lleno: Color,
    pub vacio: Color,
    pub glifos: Glifos,
}

impl EstiloBarra {
    pub fn new(lleno: Color, vacio: Color, glifos: Glifos) -> Self {
        EstiloBarra {
            lleno,
            vacio,
            glifos,
        }
    }
}

/// Barra de bloques. En modo ascii cae a `#` y `-`.
pub fn barra(ancho: usize, prop: f64, estilo: EstiloBarra) -> Vec<Span<'static>> {
    let EstiloBarra {
        lleno,
        vacio,
        glifos,
    } = estilo;
    let (a, b) = match glifos {
        Glifos::Unicode => ('█', '░'),
        Glifos::Ascii => ('#', '-'),
    };
    let n = (((ancho as f64) * prop.clamp(0.0, 1.0)).round() as usize).min(ancho);
    vec![
        Span::styled(
            std::iter::repeat_n(a, n).collect::<String>(),
            Style::default().fg(lleno),
        ),
        Span::styled(
            std::iter::repeat_n(b, ancho - n).collect::<String>(),
            Style::default().fg(vacio),
        ),
    ]
}

pub const ETIQUETA: usize = 8;

/// Una fila de medidor: etiqueta, barra y valor pegado a la derecha.
pub fn fila_medidor(
    interior: usize,
    etiqueta: &str,
    prop: f64,
    valor: &str,
    ancho_valor: usize,
    estilo: EstiloBarra,
) -> Line<'static> {
    let ancho_barra = interior
        .saturating_sub(2 + ETIQUETA + 1 + ancho_valor)
        .max(4);
    let mut spans = vec![
        Span::raw(" "),
        Span::styled(
            pad_der(etiqueta, ETIQUETA),
            Style::default().fg(theme::CENIZA),
        ),
    ];
    spans.extend(barra(ancho_barra, prop, estilo));
    spans.push(Span::raw(" "));
    spans.push(Span::styled(
        pad_izq(valor, ancho_valor),
        Style::default().fg(estilo.lleno),
    ));
    Line::from(spans)
}

pub fn tecla(k: &str, accion: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled(
            k.to_string(),
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(accion.to_string(), Style::default().fg(theme::CENIZA_HONDA)),
        Span::raw("   "),
    ]
}

pub fn barra_teclas(pares: &[(&str, &str)]) -> Line<'static> {
    let mut spans = vec![Span::raw(" ")];
    for (k, a) in pares {
        spans.extend(tecla(k, a));
    }
    Line::from(spans)
}

pub fn ancho_teclas(pares: &[(&str, &str)]) -> usize {
    pares
        .iter()
        .map(|(k, a)| k.chars().count() + 1 + a.chars().count() + 3)
        .sum::<usize>()
        .saturating_sub(3)
}

pub fn separador(ancho: usize, etiqueta: &str) -> Line<'static> {
    if etiqueta.is_empty() {
        return Line::from(Span::styled(
            "─".repeat(ancho),
            Style::default().fg(theme::ORO_APAGADO),
        ));
    }
    let etiqueta = format!(" {} ", etiqueta);
    let resto = ancho.saturating_sub(2 + etiqueta.chars().count());
    Line::from(vec![
        Span::styled("──".to_string(), Style::default().fg(theme::ORO_APAGADO)),
        Span::styled(
            etiqueta,
            Style::default()
                .fg(theme::ORO_APAGADO)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("─".repeat(resto), Style::default().fg(theme::ORO_APAGADO)),
    ])
}

pub fn color_log(t: &LogType) -> Color {
    match t {
        LogType::Info => theme::CENIZA,
        LogType::Combat => theme::ROJO_ALTAR,
        LogType::Item => theme::HUESO,
        LogType::Warning => theme::AMBAR,
        LogType::Whisper => theme::VIOLETA,
    }
}

pub fn glifo_tile(c: char, glifos: Glifos) -> char {
    match glifos {
        Glifos::Unicode => {
            if c == '.' {
                '·'
            } else {
                c
            }
        }
        Glifos::Ascii => {
            if MUROS.contains(&c) {
                '#'
            } else {
                c
            }
        }
    }
}

pub fn color_tile(c: char) -> Color {
    if c == '#' || MUROS.contains(&c) {
        theme::MURO
    } else if c == '>' {
        theme::ORO
    } else if c == '+' {
        theme::AMBAR
    } else {
        theme::SUELO
    }
}

/// Cada efecto de estado con su color: son datos, no cromo.
pub fn color_efecto(t: &StatusEffectType) -> (Color, &'static str) {
    match t {
        StatusEffectType::Poison => (Color::Rgb(78, 154, 78), "VENENO"),
        StatusEffectType::Bleed => (theme::ROJO_ALTAR, "SANGRADO"),
        StatusEffectType::Freeze => (Color::Rgb(92, 127, 209), "HELADO"),
        StatusEffectType::Burn => (theme::AMBAR, "QUEMADURA"),
        StatusEffectType::Confusion => (theme::VIOLETA, "CONFUSIÓN"),
        StatusEffectType::Blindness => (theme::CENIZA_HONDA, "CEGUERA"),
    }
}

/// Rectángulo centrado de tamaño fijo, para los modales.
pub fn rect_centrado(ancho: u16, alto: u16, area: Rect) -> Rect {
    let ancho = ancho.min(area.width);
    let alto = alto.min(area.height);
    Rect {
        x: area.x + (area.width - ancho) / 2,
        y: area.y + (area.height - alto) / 2,
        width: ancho,
        height: alto,
    }
}

/// El rectángulo del mapa. Los modales se centran ahí y no sobre toda la
/// pantalla, para no tapar los medidores ni el historial.
pub fn area_mapa(app: &App, area: Rect) -> Rect {
    let alto_log = (app.settings.lineas_susurro as u16) + 2;
    let filas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(10),
            Constraint::Length(alto_log),
            Constraint::Length(1),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Length(30)])
        .split(filas[1])[0]
}
