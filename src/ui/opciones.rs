//! La pantalla SINTONIZAR ALMA.

use super::widgets::*;
use crate::settings::{Settings, AJUSTES};
use crate::theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, ListState, Padding, Paragraph, Wrap},
    Frame,
};

/* ───────────────────────────── sintonizar alma ───────────────────────────── */

/// Pantalla de ajustes. Sólo muestra lo que hoy hace algo.
pub fn options_ui(f: &mut Frame, ajustes: &Settings, estado: &mut ListState) {
    let size = f.size();
    let seleccionado = estado.selected().unwrap_or(0);

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(marco(true))
            .title(Span::styled(" SINTONIZAR ALMA ", titulo(true)))
            .title_alignment(Alignment::Center),
        size,
    );

    let principal = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(size);

    let columnas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(38),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(principal[0]);

    let bloque = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" AJUSTES ", titulo(false)))
        .border_style(marco(false));
    let interior = bloque.inner(columnas[0]);
    f.render_widget(bloque, columnas[0]);

    let ancho = interior.width as usize;
    let mut filas: Vec<Line> = vec![Line::from("")];
    for (i, etiqueta) in AJUSTES.iter().enumerate() {
        let activo = i == seleccionado;
        let base = if activo {
            Style::default().fg(theme::PENUMBRA).bg(theme::ORO)
        } else {
            Style::default().fg(theme::CENIZA)
        };

        if i == AJUSTES.len() - 1 {
            filas.push(Line::from(""));
            filas.push(separador(ancho, ""));
        }

        let valor = ajustes.valor(i);
        let mut spans = vec![
            Span::styled(" ", base),
            Span::styled(pad_der(etiqueta, 19), base),
        ];
        match ajustes.proporcion(i) {
            Some(prop) => {
                let lleno = if activo { theme::PENUMBRA } else { theme::ORO };
                let vacio = if activo {
                    theme::ORO_APAGADO
                } else {
                    theme::CENIZA_HONDA
                };
                let mut b = barra(10, prop, EstiloBarra::new(lleno, vacio, ajustes.glifos));
                if activo {
                    for s in b.iter_mut() {
                        s.style = s.style.bg(theme::ORO);
                    }
                }
                spans.extend(b);
                spans.push(Span::styled(
                    pad_izq(&valor, ancho.saturating_sub(31)),
                    if activo {
                        base.add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme::HUESO)
                    },
                ));
            }
            None => spans.push(Span::styled(
                pad_izq(&valor, ancho.saturating_sub(21)),
                if activo {
                    base.add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme::HUESO)
                },
            )),
        }
        filas.push(Line::from(spans));
        filas.push(Line::from(""));
    }
    f.render_widget(Paragraph::new(filas), interior);

    let descripcion = vec![
        Line::from(""),
        Line::from(Span::styled(
            AJUSTES[seleccionado],
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            Settings::descripcion(seleccionado),
            Style::default().fg(theme::HUESO),
        )),
    ];
    f.render_widget(
        Paragraph::new(descripcion).wrap(Wrap { trim: true }).block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1))
                .title(Span::styled(" QUÉ HACE ", titulo(false)))
                .border_style(marco(false)),
        ),
        columnas[2],
    );

    let pares = [
        ("↑↓", "elegir"),
        ("←→", "ajustar"),
        ("ENTER", "confirmar"),
        ("ESC", "volver"),
    ];
    let x = (size.width as usize).saturating_sub(ancho_teclas(&pares)) / 2;
    let mut linea = barra_teclas(&pares);
    linea.spans.insert(0, Span::raw(" ".repeat(x)));
    f.render_widget(Paragraph::new(linea), principal[1]);
}
