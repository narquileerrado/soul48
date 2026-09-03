//! Las dos pantallas de cierre: caíste, o llegaste.
//!
//! Comparten toda la estructura —el mapa final de fondo, un panel centrado
//! sobre él y una ilustración si la terminal da—, así que la arman las dos con
//! `cierre` y sólo aportan su texto y su color.

use super::juego::ui;
use super::widgets::*;
use crate::app::App;
use crate::arte;
use crate::settings::Glifos;
use crate::sprite::{Paleta, Sprite};
use crate::theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// El panel de cierre, sobre el mapa de la partida que acaba de terminar.
///
/// `sprite` sólo se dibuja si el área del mapa da para la versión grande; en
/// una terminal chica cae al panel de siempre, sin ilustración.
fn cierre(
    f: &mut Frame,
    app: &App,
    titulo_panel: &str,
    color: Color,
    sprite: &Sprite,
    acento: Color,
    texto: Vec<Line>,
) {
    ui(f, app);

    let mapa = area_mapa(app, f.size());
    let con_arte = mapa.height >= 22 && mapa.width >= 56;
    let zona = if con_arte {
        rect_centrado(56, 22, mapa)
    } else {
        rect_centrado(52, 13, mapa)
    };
    f.render_widget(Clear, zona);

    let estilo = Style::default().fg(color);
    let bloque = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            titulo_panel,
            estilo.add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center)
        .border_style(estilo);
    let interior = bloque.inner(zona);
    f.render_widget(bloque, zona);

    let area_texto = if con_arte {
        let partes = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(sprite.alto_en_celdas()),
                Constraint::Min(0),
            ])
            .split(interior);
        let paleta = Paleta::de(theme::HUESO, acento);
        f.render_widget(
            Paragraph::new(sprite.lineas(
                &paleta,
                Color::Reset,
                app.settings.glifos == Glifos::Ascii,
            ))
            .alignment(Alignment::Center),
            partes[0],
        );
        partes[1]
    } else {
        interior
    };

    f.render_widget(
        Paragraph::new(texto).alignment(Alignment::Center),
        area_texto,
    );
}

/// Piso alcanzado y nivel, la línea de balance que llevan las dos pantallas.
fn marcador(app: &App) -> Line<'static> {
    let dato = Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD);
    Line::from(vec![
        Span::styled("SÓTANO ALCANÇADO  ", Style::default().fg(theme::CENIZA)),
        Span::styled(format!("{}", app.depth), dato),
        Span::styled("     GRADO  ", Style::default().fg(theme::CENIZA)),
        Span::styled(format!("{}", app.player.level), dato),
        Span::styled("     SIMIENTE  ", Style::default().fg(theme::CENIZA)),
        Span::styled(
            format!("{}", app.seed),
            Style::default().fg(theme::CENIZA_HONDA),
        ),
    ])
}

/// Las teclas que ofrecen las dos pantallas.
fn salidas(reinicio: &'static str) -> Line<'static> {
    let tecla = Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD);
    let glosa = Style::default().fg(theme::CENIZA_HONDA);
    Line::from(vec![
        Span::styled("R", tecla),
        Span::styled(format!(" {}      ", reinicio), glosa),
        Span::styled("Q", tecla),
        Span::styled(" silencio", glosa),
    ])
}

/// Muestra la pantalla de derrota superpuesta al estado final del mapa.
pub fn game_over_ui(f: &mut Frame, app: &App) {
    let hueso = Style::default().fg(theme::HUESO);
    let texto = vec![
        Line::from(""),
        Line::from(Span::styled(
            "HABÉIS CAÍDO",
            Style::default()
                .fg(theme::ROJO_ALTAR)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Consumió el abismo lo poco que", hueso)),
        Line::from(Span::styled(
            "de vuestra ánima quedaba en aqueste sótano.",
            hueso,
        )),
        Line::from(""),
        Line::from(Span::styled(
            "«...y con todo, seguís hablando.»",
            Style::default()
                .fg(theme::VIOLETA)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        marcador(app),
        Line::from(""),
        Line::from(Span::styled(
            "Deshácese el fragmento: no hay tornar a aqueste sótano.",
            Style::default().fg(theme::CENIZA_HONDA),
        )),
        Line::from(""),
        salidas("tornar a empeçar"),
    ];

    cierre(
        f,
        app,
        " FIN DE LA JORNADA ",
        theme::ROJO_ALTAR,
        &arte::CALAVERA,
        theme::ROJO_ALTAR,
        texto,
    );
}

/// La pantalla de victoria: el Archidemonio cayó.
pub fn victory_ui(f: &mut Frame, app: &App) {
    let hueso = Style::default().fg(theme::HUESO);
    let texto = vec![
        Line::from(""),
        Line::from(Span::styled(
            "HABÉIS COBRADO VUESTRA VOZ",
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Deshácese el Archidemonio del Silencio,",
            hueso,
        )),
        Line::from(Span::styled(
            "y con él los quarenta y ocho sótanos que os callaron.",
            hueso,
        )),
        Line::from(""),
        Line::from(Span::styled(
            "«Hablad. Nadie os lo ha de quitar otra vez.»",
            Style::default()
                .fg(theme::VIOLETA)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        marcador(app),
        Line::from(""),
        salidas("otra baxada"),
    ];

    cierre(
        f,
        app,
        " EL ORIGEN ",
        theme::ORO,
        &arte::PORTAL,
        theme::AZUL_ALMA,
        texto,
    );
}
