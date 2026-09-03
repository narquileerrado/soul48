//! El Compendio de las Sombras.

use super::widgets::*;
use crate::arte;
use crate::bestiary::get_bestiary;
use crate::settings::{Glifos, Settings};
use crate::sprite::Paleta;
use crate::theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph, Wrap},
    Frame,
};

/* ───────────────────────────── compendio ───────────────────────────── */

const GLIFOS_MAPA: [(char, Color, &str); 12] = [
    ('@', theme::AZUL_ALMA, "vos"),
    ('#', theme::MURO, "muro"),
    ('·', theme::SUELO, "suelo"),
    ('>', theme::ORO, "escalera"),
    ('+', theme::CENIZA, "puerta"),
    ('W', theme::VIOLETA, "pared que habla"),
    ('A', theme::ROJO_ALTAR, "altar de los ecos"),
    ('?', theme::VIOLETA, "pergamino"),
    ('C', theme::COFRE, "cofre o remedador"),
    ('^', theme::AMBAR, "trampa o almete"),
    ('!', theme::HUESO, "redoma"),
    ('k', theme::HUESO, "llave"),
];

/// Renderiza el Compendio: criaturas, presencias y el alfabeto del mapa.
pub fn bestiary_ui(f: &mut Frame, list_state: &mut ListState, ajustes: &Settings) {
    let size = f.size();
    let bestiary = get_bestiary();

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(marco(true))
            .title(Span::styled(" EL COMPENDIO DE LAS SOMBRAS ", titulo(true)))
            .title_alignment(Alignment::Center),
        size,
    );

    let principal = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(size);

    let contenido = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(32),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(principal[0]);
    let (columna_izq, columna_der) = (contenido[0], contenido[2]);

    // La lista de criaturas pasó de cinco a dieciséis con los jefes: el bloque
    // de entidades es el que tiene que crecer, y los glifos del mapa —que son
    // siempre los mismos doce— van con alto fijo.
    let alto_glifos = GLIFOS_MAPA.len() as u16 + 2;
    let izquierda = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(alto_glifos)])
        .split(columna_izq);

    /* --- entidades --- */
    let bloque_entidades = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" LAS ENTIDADES ", titulo(false)))
        .border_style(marco(false));
    let interior_entidades = bloque_entidades.inner(izquierda[0]);
    f.render_widget(bloque_entidades, izquierda[0]);

    let secciones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // separador CRIATURAS
            Constraint::Min(3),    // la lista, que es la que se lleva el sobrante
            Constraint::Length(1),
            Constraint::Length(1), // separador PRESENCIAS
            Constraint::Length(2),
        ])
        .split(interior_entidades);

    let ancho = interior_entidades.width as usize;
    f.render_widget(Paragraph::new(separador(ancho, "CRIATURAS")), secciones[0]);

    let items: Vec<ListItem> = bestiary
        .iter()
        .map(|e| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{} ", e.glyph),
                    Style::default().fg(e.color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(e.name, Style::default().fg(theme::HUESO)),
            ]))
        })
        .collect();
    let lista = List::new(items)
        .highlight_style(
            Style::default()
                .fg(theme::PENUMBRA)
                .bg(theme::ORO)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(lista, secciones[1], list_state);

    f.render_widget(Paragraph::new(separador(ancho, "PRESENCIAS")), secciones[3]);
    let presencias = vec![
        Line::from(vec![
            Span::styled(
                "  W ",
                Style::default()
                    .fg(theme::VIOLETA)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Pared de los Lamentos", Style::default().fg(theme::HUESO)),
        ]),
        Line::from(vec![
            Span::styled(
                "  A ",
                Style::default()
                    .fg(theme::ROJO_ALTAR)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Altar de los Ecos", Style::default().fg(theme::HUESO)),
        ]),
    ];
    f.render_widget(Paragraph::new(presencias), secciones[4]);

    /* --- glifos del mapa --- */
    let glifos: Vec<Line> = GLIFOS_MAPA
        .iter()
        .map(|(g, c, nombre)| {
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    g.to_string(),
                    Style::default().fg(*c).add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
                Span::styled(*nombre, Style::default().fg(theme::CENIZA)),
            ])
        })
        .collect();
    f.render_widget(
        Paragraph::new(glifos).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" GLIFOS DEL MAPA ", titulo(false)))
                .border_style(marco(false)),
        ),
        izquierda[1],
    );

    /* --- crónica --- */
    let e = &bestiary[list_state.selected().unwrap_or(0).min(bestiary.len() - 1)];
    let bloque_cronica = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .title(Span::styled(" CRÓNICA ", titulo(false)))
        .border_style(marco(false));
    let interior_cronica = bloque_cronica.inner(columna_der);
    let ancho_cronica = interior_cronica.width as usize;
    f.render_widget(bloque_cronica, columna_der);

    // banda de retrato: el sprite a la izquierda, la ficha de la criatura al lado
    let retrato = arte::de_criatura(e.short_name);
    let alto_banda = retrato.map(|s| s.alto_en_celdas()).unwrap_or(0);
    let con_retrato = retrato.is_some() && interior_cronica.height >= 20 + alto_banda;

    let area_texto = match retrato {
        Some(s) if con_retrato => {
            let partes = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(alto_banda), Constraint::Min(0)])
                .split(interior_cronica);
            let banda = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(s.ancho() as u16),
                    Constraint::Length(2),
                    Constraint::Min(0),
                ])
                .split(partes[0]);

            // la rampa de tonos sale del color que la criatura ya tiene en el mapa
            let paleta = Paleta::de(e.color, theme::ROJO_ALTAR);
            f.render_widget(
                Paragraph::new(s.lineas(&paleta, Color::Reset, ajustes.glifos == Glifos::Ascii)),
                banda[0],
            );

            let ficha = vec![
                Line::from(Span::styled(
                    e.name.to_uppercase(),
                    Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    e.scientific_name,
                    Style::default()
                        .fg(theme::CENIZA)
                        .add_modifier(Modifier::ITALIC),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    e.taxonomy,
                    Style::default().fg(theme::CENIZA_HONDA),
                )),
            ];
            f.render_widget(Paragraph::new(ficha).wrap(Wrap { trim: true }), banda[2]);
            partes[1]
        }
        _ => interior_cronica,
    };

    let mut detalle: Vec<Line> = Vec::new();
    if !con_retrato {
        detalle.push(Line::from(Span::styled(
            e.name.to_uppercase(),
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        )));
        detalle.push(Line::from(Span::styled(
            e.taxonomy,
            Style::default().fg(theme::CENIZA_HONDA),
        )));
    }
    detalle.extend([
        Line::from(""),
        separador(ancho_cronica, "RELATO ANTIGUO"),
        Line::from(""),
        Line::from(Span::styled(
            e.description,
            Style::default().fg(theme::HUESO),
        )),
        Line::from(""),
        separador(ancho_cronica, "ATRIBUTOS"),
        Line::from(""),
    ]);

    for (etiqueta, prop, valor, color) in [
        (
            "VIGOR",
            e.base_hp as f64 / 45.0,
            format!("{}", e.base_hp),
            theme::ROJO_ALTAR,
        ),
        (
            "PUJANÇA",
            ((e.base_damage.0 + e.base_damage.1) as f64 / 2.0) / 12.0,
            format!("{}-{}", e.base_damage.0, e.base_damage.1),
            theme::AMBAR,
        ),
        (
            "REPARO",
            e.base_defense as f64 / 5.0,
            format!("{}", e.base_defense),
            theme::AZUL_ALMA,
        ),
    ] {
        let mut spans = vec![Span::styled(
            pad_der(etiqueta, 12),
            Style::default().fg(theme::CENIZA),
        )];
        spans.extend(barra(
            20,
            prop,
            EstiloBarra::new(color, theme::CENIZA_HONDA, Glifos::Unicode),
        ));
        spans.push(Span::raw("  "));
        spans.push(Span::styled(valor, Style::default().fg(color)));
        detalle.push(Line::from(spans));
    }
    detalle.push(Line::from(""));
    detalle.push(Line::from(vec![
        Span::styled(pad_der("CONDICIÓN", 12), Style::default().fg(theme::CENIZA)),
        Span::styled(e.behavior, Style::default().fg(theme::HUESO)),
    ]));

    f.render_widget(
        Paragraph::new(detalle).wrap(Wrap { trim: true }),
        area_texto,
    );

    let pares = [("↑↓", "andar"), ("ESC", "tornar")];
    f.render_widget(
        Paragraph::new(barra_teclas(&pares)).alignment(Alignment::Center),
        principal[1],
    );
}
