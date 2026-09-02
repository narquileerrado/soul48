//! Dibujo de todas las pantallas del juego.
//!
//! Todo se apoya en `theme`: cada color tiene un significado y uno solo.

use crate::app::{App, LogType, Point};
use crate::arte;
use crate::sprite::Paleta;
use crate::bestiary::get_bestiary;
use crate::settings::{Glifos, Settings, AJUSTES};
use crate::theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap,
    },
    Frame,
};

const MUROS: [char; 12] = ['║', '═', '╚', '╔', '╝', '╗', '╠', '╣', '╩', '╦', '╬', '■'];

/* ───────────────────────────── utilidades ───────────────────────────── */

fn marco(foco: bool) -> Style {
    Style::default().fg(if foco { theme::ORO } else { theme::ORO_APAGADO })
}
fn titulo(foco: bool) -> Style {
    if foco {
        Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::CENIZA)
    }
}

fn pad_der(s: &str, n: usize) -> String {
    let len = s.chars().count();
    if len >= n {
        s.chars().take(n).collect()
    } else {
        format!("{}{}", s, " ".repeat(n - len))
    }
}
fn pad_izq(s: &str, n: usize) -> String {
    let len = s.chars().count();
    if len >= n {
        s.chars().take(n).collect()
    } else {
        format!("{}{}", " ".repeat(n - len), s)
    }
}
fn recortar(s: &str, n: usize) -> String {
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

/// Barra de bloques. En modo ascii cae a `#` y `-`.
fn barra(ancho: usize, prop: f64, lleno: Color, vacio: Color, glifos: Glifos) -> Vec<Span<'static>> {
    let (a, b) = match glifos {
        Glifos::Unicode => ('█', '░'),
        Glifos::Ascii => ('#', '-'),
    };
    let n = ((ancho as f64) * prop.clamp(0.0, 1.0)).round() as usize;
    let n = n.min(ancho);
    vec![
        Span::styled(
            std::iter::repeat(a).take(n).collect::<String>(),
            Style::default().fg(lleno),
        ),
        Span::styled(
            std::iter::repeat(b).take(ancho - n).collect::<String>(),
            Style::default().fg(vacio),
        ),
    ]
}

/// Una fila de medidor: etiqueta, barra y valor pegado a la derecha.
fn fila_medidor(
    interior: usize,
    etiqueta: &str,
    prop: f64,
    valor: &str,
    ancho_valor: usize,
    color: Color,
    tenue: Color,
    glifos: Glifos,
) -> Line<'static> {
    let ancho_etiqueta = 6;
    let ancho_barra = interior
        .saturating_sub(2 + ancho_etiqueta + 1 + ancho_valor)
        .max(4);
    let mut spans = vec![
        Span::raw(" "),
        Span::styled(
            pad_der(etiqueta, ancho_etiqueta),
            Style::default().fg(theme::CENIZA),
        ),
    ];
    spans.extend(barra(ancho_barra, prop, color, tenue, glifos));
    spans.push(Span::raw(" "));
    spans.push(Span::styled(
        pad_izq(valor, ancho_valor),
        Style::default().fg(color),
    ));
    Line::from(spans)
}

fn tecla(k: &str, accion: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled(
            k.to_string(),
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            accion.to_string(),
            Style::default().fg(theme::CENIZA_HONDA),
        ),
        Span::raw("   "),
    ]
}

fn barra_teclas(pares: &[(&str, &str)]) -> Line<'static> {
    let mut spans = vec![Span::raw(" ")];
    for (k, a) in pares {
        spans.extend(tecla(k, a));
    }
    Line::from(spans)
}

fn separador(ancho: usize, etiqueta: &str) -> Line<'static> {
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
        Span::styled(
            "─".repeat(resto),
            Style::default().fg(theme::ORO_APAGADO),
        ),
    ])
}

fn color_log(t: &LogType) -> Color {
    match t {
        LogType::Info => theme::CENIZA,
        LogType::Combat => theme::ROJO_ALTAR,
        LogType::Item => theme::HUESO,
        LogType::Warning => theme::AMBAR,
        LogType::Whisper => theme::VIOLETA,
    }
}

fn glifo_tile(c: char, glifos: Glifos) -> char {
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

fn color_tile(c: char) -> Color {
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

/// Rectángulo centrado de tamaño fijo, para los modales.
fn rect_centrado(ancho: u16, alto: u16, area: Rect) -> Rect {
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
fn area_mapa(app: &App, area: Rect) -> Rect {
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

/* ───────────────────────────── pantalla de juego ───────────────────────────── */

/// Renderiza la interfaz principal durante la exploración (GameState::Playing).
pub fn ui(f: &mut Frame, app: &App) {
    let glifos = app.settings.glifos;
    let alto_log = (app.settings.lineas_susurro as u16) + 2;

    let filas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // cinta
            Constraint::Min(10),   // cuerpo
            Constraint::Length(alto_log),
            Constraint::Length(1), // teclas
        ])
        .split(f.size());

    let cuerpo = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Length(30)])
        .split(filas[1]);

    let columna = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // TU ALMA
            Constraint::Length(12), // LO QUE CARGAS
            Constraint::Min(0),     // LO QUE TE RODEA
        ])
        .split(cuerpo[1]);

    cinta(f, app, filas[0]);
    mapa(f, app, cuerpo[0], glifos);
    alma(f, app, columna[0], glifos);
    cargas(f, app, columna[1]);
    rodea(f, app, columna[2]);
    historial(f, app, filas[2]);

    let teclas = if app.drop_mode {
        barra_teclas(&[("1-9", "soltar"), ("D", "cancelar")])
    } else {
        barra_teclas(&[
            ("←↑→↓", "moverte"),
            ("1-9", "usar"),
            ("D", "soltar"),
            ("V", "hablar"),
            ("S", "bajar"),
            ("Q", "volver al silencio"),
        ])
    };
    f.render_widget(Paragraph::new(teclas), filas[3]);

    if app.show_descend_prompt {
        modal_descenso(f, app, cuerpo[0]);
    }
}

/// Cinta superior: quién sos, dónde estás y con qué semilla.
fn cinta(f: &mut Frame, app: &App, area: Rect) {
    let mitades = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let izq = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            "SOUL 48",
            Style::default()
                .fg(theme::AZUL_ALMA)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ·  ", Style::default().fg(theme::CENIZA_HONDA)),
        Span::styled(
            format!("PISO {}", app.depth),
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        ),
    ]);
    f.render_widget(Paragraph::new(izq), mitades[0]);

    let der = Line::from(Span::styled(
        format!("SEMILLA {} ", app.seed),
        Style::default().fg(theme::CENIZA_HONDA),
    ));
    f.render_widget(
        Paragraph::new(der).alignment(Alignment::Right),
        mitades[1],
    );
}

/// El mapa. Lo visible en color pleno, lo recordado apagado pero con su propio tono.
fn mapa(f: &mut Frame, app: &App, area: Rect, glifos: Glifos) {
    let brillo = app.settings.penumbra;
    let mut lineas: Vec<Line> = Vec::with_capacity(app.map.len());

    for (y, fila) in app.map.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::with_capacity(fila.len());
        for (x, &tile) in fila.iter().enumerate() {
            let pos = Point::new(x, y);

            if pos == app.hero_pos {
                let color = if app.hero_hp * 4 <= app.hero_max_hp {
                    theme::ROJO_ALTAR
                } else {
                    theme::AZUL_ALMA
                };
                spans.push(Span::styled(
                    "@",
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            } else if app.visible[y][x] {
                if let Some(e) = app.entities.iter().find(|e| e.pos == pos) {
                    spans.push(Span::styled(
                        e.glyph.to_string(),
                        Style::default().fg(e.color).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::styled(
                        glifo_tile(tile, glifos).to_string(),
                        Style::default().fg(color_tile(tile)),
                    ));
                }
            } else if app.explored[y][x] {
                // lo que no se mueve se recuerda en una versión apagada de su color
                let estatico = app
                    .entities
                    .iter()
                    .find(|e| e.pos == pos && App::es_estatico(&e.e_type));
                match estatico {
                    Some(e) => spans.push(Span::styled(
                        e.glyph.to_string(),
                        Style::default().fg(theme::recordado(e.color, brillo)),
                    )),
                    None => spans.push(Span::styled(
                        glifo_tile(tile, glifos).to_string(),
                        Style::default().fg(theme::recordado(color_tile(tile), brillo)),
                    )),
                }
            } else {
                spans.push(Span::raw(" "));
            }
        }
        lineas.push(Line::from(spans));
    }

    let mut leyenda: Vec<Span> = vec![Span::raw(" ")];
    for (g, nombre, color) in [
        ('@', "vos", theme::AZUL_ALMA),
        ('W', "pared", theme::VIOLETA),
        ('A', "altar", theme::ROJO_ALTAR),
        ('>', "escalera", theme::ORO),
    ] {
        leyenda.push(Span::styled(
            g.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
        leyenda.push(Span::styled(
            format!(" {}  ", nombre),
            Style::default().fg(theme::CENIZA),
        ));
    }

    f.render_widget(
        Paragraph::new(lineas).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" LA PENUMBRA ", titulo(true)))
                .title(
                    Title::from(Line::from(leyenda))
                        .position(Position::Bottom)
                        .alignment(Alignment::Left),
                )
                .border_style(marco(true)),
        ),
        area,
    );
}

/// Vitales: sólo lo que cambia turno a turno.
fn alma(f: &mut Frame, app: &App, area: Rect, glifos: Glifos) {
    let interior = area.width.saturating_sub(2) as usize;
    let v_alma = format!("{}/{}", app.hero_hp, app.hero_max_hp);
    let v_voz = format!("{}/{}", app.hero_voice, app.hero_max_voice);
    let ancho_valor = v_alma.chars().count().max(v_voz.chars().count());

    let arma = app
        .equipped_weapon
        .as_ref()
        .map(|w| (w.0.clone(), format!("{}-{}", w.1, w.2)))
        .unwrap_or_else(|| ("Puños".to_string(), "1-3".to_string()));

    let nombre_arma = recortar(&arma.0, interior.saturating_sub(9 + arma.1.chars().count()));
    let ancho_arma = nombre_arma.chars().count();

    let lineas = vec![
        fila_medidor(
            interior,
            "ALMA",
            app.hero_hp as f64 / app.hero_max_hp.max(1) as f64,
            &v_alma,
            ancho_valor,
            theme::AZUL_ALMA,
            theme::AZUL_APAGADO,
            glifos,
        ),
        fila_medidor(
            interior,
            "VOZ",
            app.hero_voice as f64 / app.hero_max_voice.max(1) as f64,
            &v_voz,
            ancho_valor,
            theme::VIOLETA,
            theme::VIOLETA_APAGADO,
            glifos,
        ),
        Line::from(""),
        Line::from(vec![
            Span::raw(" "),
            Span::styled(pad_der("ARMA", 6), Style::default().fg(theme::CENIZA)),
            Span::styled(nombre_arma.clone(), Style::default().fg(theme::HUESO)),
            Span::styled(
                pad_izq(&arma.1, interior.saturating_sub(8 + ancho_arma)),
                Style::default().fg(theme::CENIZA),
            ),
            Span::raw(" "),
        ]),
    ];

    f.render_widget(
        Paragraph::new(lineas).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" TU ALMA ", titulo(false)))
                .border_style(marco(false)),
        ),
        area,
    );
}

/// Inventario. En modo soltar cambia de color, de título y de borde.
fn cargas(f: &mut Frame, app: &App, area: Rect) {
    let interior = area.width.saturating_sub(2) as usize;
    let color_numero = if app.drop_mode {
        theme::AMBAR
    } else {
        theme::ORO
    };

    let mut lineas: Vec<Line> = vec![Line::from("")];
    for (i, (item, cuenta)) in app.inventory.iter().enumerate() {
        let meta = if *cuenta > 1 {
            format!("x{}", cuenta)
        } else if app
            .equipped_weapon
            .as_ref()
            .map(|w| w.0 == item.name)
            .unwrap_or(false)
        {
            "equipada".to_string()
        } else {
            String::new()
        };
        let ancho_nombre = interior.saturating_sub(5 + meta.chars().count());
        lineas.push(Line::from(vec![
            Span::raw(" "),
            Span::styled(
                format!("{}", i + 1),
                Style::default().fg(color_numero).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                pad_der(&recortar(&item.name, ancho_nombre), ancho_nombre),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled(
                format!("{} ", meta),
                Style::default().fg(theme::CENIZA),
            ),
        ]));
    }

    let libres = 9usize.saturating_sub(app.inventory.len());
    if libres > 0 {
        lineas.push(Line::from(""));
        let texto = format!("{} ranuras libres", libres);
        let margen = interior.saturating_sub(texto.chars().count()) / 2;
        lineas.push(Line::from(Span::styled(
            format!("{}{}", " ".repeat(margen), texto),
            Style::default().fg(theme::CENIZA_HONDA),
        )));
    }
    if app.drop_mode {
        lineas.push(Line::from(""));
        lineas.push(Line::from(Span::styled(
            "elegí un número y lo dejás a tus pies",
            Style::default()
                .fg(theme::AMBAR)
                .add_modifier(Modifier::ITALIC),
        )));
    }

    let (nombre, estilo_borde, estilo_titulo) = if app.drop_mode {
        (
            " LO QUE SUELTAS ",
            Style::default().fg(theme::AMBAR),
            Style::default().fg(theme::AMBAR).add_modifier(Modifier::BOLD),
        )
    } else {
        (" LO QUE CARGAS ", marco(false), titulo(false))
    };

    f.render_widget(
        Paragraph::new(lineas)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(nombre, estilo_titulo))
                    .border_style(estilo_borde),
            ),
        area,
    );
}

/// Lo que el héroe tiene a la vista, ordenado por cercanía.
fn rodea(f: &mut Frame, app: &App, area: Rect) {
    let interior = area.width.saturating_sub(2) as usize;
    let capacidad = area.height.saturating_sub(3) as usize;
    let cerca = app.entidades_cercanas(capacidad);

    let mut lineas: Vec<Line> = vec![Line::from(Span::styled(
        pad_izq("pasos ", interior),
        Style::default().fg(theme::CENIZA_HONDA),
    ))];

    if cerca.is_empty() {
        lineas.push(Line::from(Span::styled(
            " nada a la vista",
            Style::default().fg(theme::CENIZA_HONDA),
        )));
    }

    for (glifo, color, nombre, dist) in cerca {
        let d = format!("{}", dist);
        let ancho_nombre = interior.saturating_sub(4 + d.chars().count());
        lineas.push(Line::from(vec![
            Span::raw(" "),
            Span::styled(
                glifo.to_string(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                pad_der(&recortar(&nombre, ancho_nombre), ancho_nombre),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled(format!("{} ", d), Style::default().fg(theme::CENIZA)),
        ]));
    }

    f.render_widget(
        Paragraph::new(lineas).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" LO QUE TE RODEA ", titulo(false)))
                .border_style(marco(false)),
        ),
        area,
    );
}

/// Historial: cada voz con su color.
fn historial(f: &mut Frame, app: &App, area: Rect) {
    let lineas: Vec<Line> = app
        .logs
        .iter()
        .map(|msg| {
            let color = color_log(&msg.l_type);
            let texto = msg.text.trim_start_matches("> ").to_string();
            Line::from(vec![
                Span::raw(" "),
                Span::styled("»", Style::default().fg(color)),
                Span::raw(" "),
                Span::styled(texto, Style::default().fg(color)),
            ])
        })
        .collect();

    f.render_widget(
        Paragraph::new(lineas).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" LO QUE SE DICE ", titulo(false)))
                .border_style(marco(false)),
        ),
        area,
    );
}

/// Confirmación de descenso. Antes esta pregunta no se dibujaba en ningún lado.
fn modal_descenso(f: &mut Frame, app: &App, area: Rect) {
    let zona = rect_centrado(44, 9, area);
    f.render_widget(Clear, zona);

    let texto = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Descienden hacia el piso {}.", app.depth + 1),
            Style::default().fg(theme::HUESO),
        )),
        Line::from(Span::styled(
            "Lo que dejes aquí, aquí se queda.",
            Style::default()
                .fg(theme::CENIZA)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "S",
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" bajar        ", Style::default().fg(theme::CENIZA_HONDA)),
            Span::styled(
                "N",
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" quedarte", Style::default().fg(theme::CENIZA_HONDA)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(texto)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(" LAS ESCALERAS ", titulo(true)))
                    .title_alignment(Alignment::Center)
                    .border_style(marco(true)),
            ),
        zona,
    );
}

/* ───────────────────────────── fin de la partida ───────────────────────────── */

/// Muestra la pantalla de derrota superpuesta al estado final del mapa.
pub fn game_over_ui(f: &mut Frame, app: &App) {
    ui(f, app);

    let mapa = area_mapa(app, f.size());
    // la ilustración sólo si el mapa da: si no, el modal chico de siempre
    let con_arte = mapa.height >= 22 && mapa.width >= 56;
    let zona = if con_arte {
        rect_centrado(56, 22, mapa)
    } else {
        rect_centrado(52, 13, mapa)
    };
    f.render_widget(Clear, zona);

    let rojo = Style::default().fg(theme::ROJO_ALTAR);
    let texto = vec![
        Line::from(""),
        Line::from(Span::styled(
            "HAS CAÍDO",
            rojo.add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "El archidemonio ha consumido lo poco que",
            Style::default().fg(theme::HUESO),
        )),
        Line::from(Span::styled(
            "quedaba de tu alma en este piso.",
            Style::default().fg(theme::HUESO),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "«...y sin embargo, seguís hablando.»",
            Style::default()
                .fg(theme::VIOLETA)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("PISO ALCANZADO  ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                format!("{}", app.depth),
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "R",
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " volver a empezar      ",
                Style::default().fg(theme::CENIZA_HONDA),
            ),
            Span::styled(
                "Q",
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" silencio", Style::default().fg(theme::CENIZA_HONDA)),
        ]),
    ];

    let bloque = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " FIN DE LA PARTIDA ",
            rojo.add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center)
        .border_style(rojo);
    let interior = bloque.inner(zona);
    f.render_widget(bloque, zona);

    let area_texto = if con_arte {
        let partes = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(arte::CALAVERA.alto_en_celdas()),
                Constraint::Min(0),
            ])
            .split(interior);
        let paleta = Paleta::de(theme::HUESO, theme::ROJO_ALTAR);
        f.render_widget(
            Paragraph::new(arte::CALAVERA.lineas(
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

/* ───────────────────────────── compendio ───────────────────────────── */

const GLIFOS_MAPA: [(char, Color, &str); 10] = [
    ('@', theme::AZUL_ALMA, "vos"),
    ('#', theme::MURO, "muro"),
    ('·', theme::SUELO, "suelo"),
    ('>', theme::ORO, "escaleras"),
    ('W', theme::VIOLETA, "pared parlante"),
    ('A', theme::ROJO_ALTAR, "altar de ecos"),
    ('C', Color::Rgb(196, 160, 0), "cofre o mímico"),
    ('k', theme::HUESO, "llave"),
    ('!', theme::HUESO, "poción"),
    ('/', theme::AZUL_ALMA, "arma"),
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

    let izquierda = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(16), Constraint::Min(0)])
        .split(columna_izq);

    /* --- entidades --- */
    let bloque_entidades = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" ENTIDADES ", titulo(false)))
        .border_style(marco(false));
    let interior_entidades = bloque_entidades.inner(izquierda[0]);
    f.render_widget(bloque_entidades, izquierda[0]);

    let secciones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // separador CRIATURAS
            Constraint::Length(5), // lista
            Constraint::Length(1), // aire
            Constraint::Length(1), // separador PRESENCIAS
            Constraint::Min(0),    // presencias
        ])
        .split(interior_entidades);

    let ancho = interior_entidades.width as usize;
    f.render_widget(
        Paragraph::new(separador(ancho, "CRIATURAS")),
        secciones[0],
    );

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

    f.render_widget(
        Paragraph::new(separador(ancho, "PRESENCIAS")),
        secciones[3],
    );
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
            Span::styled("Altar de Ecos", Style::default().fg(theme::HUESO)),
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
    let retrato = arte::de_criatura(e.glyph);
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
                Paragraph::new(s.lineas(
                    &paleta,
                    Color::Reset,
                    ajustes.glifos == Glifos::Ascii,
                )),
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
        detalle.push(Line::from(vec![
            Span::styled(
                e.name.to_uppercase(),
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  —  ", Style::default().fg(theme::CENIZA_HONDA)),
            Span::styled(
                e.scientific_name,
                Style::default()
                    .fg(theme::CENIZA)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]));
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

    let atributos: [(&str, f64, String, Color); 3] = [
        ("VITALIDAD", e.base_hp as f64 / 45.0, format!("{}", e.base_hp), theme::ROJO_ALTAR),
        (
            "POTENCIA",
            ((e.base_damage.0 + e.base_damage.1) as f64 / 2.0) / 12.0,
            format!("{}-{}", e.base_damage.0, e.base_damage.1),
            theme::AMBAR,
        ),
        ("DEFENSA", e.base_defense as f64 / 5.0, format!("{}", e.base_defense), theme::AZUL_ALMA),
    ];
    for (etiqueta, prop, valor, color) in atributos {
        let mut spans = vec![Span::styled(
            pad_der(etiqueta, 12),
            Style::default().fg(theme::CENIZA),
        )];
        spans.extend(barra(20, prop, color, theme::CENIZA_HONDA, Glifos::Unicode));
        spans.push(Span::raw("  "));
        spans.push(Span::styled(valor, Style::default().fg(color)));
        detalle.push(Line::from(spans));
    }
    detalle.push(Line::from(""));
    detalle.push(Line::from(vec![
        Span::styled(pad_der("CONDUCTA", 12), Style::default().fg(theme::CENIZA)),
        Span::styled(e.behavior, Style::default().fg(theme::HUESO)),
    ]));

    f.render_widget(
        Paragraph::new(detalle).wrap(Wrap { trim: true }),
        area_texto,
    );

    f.render_widget(
        Paragraph::new(barra_teclas(&[("↑↓", "navegar"), ("ESC", "volver")]))
            .alignment(Alignment::Center),
        principal[1],
    );
}

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
                let mut b = barra(10, prop, lleno, vacio, ajustes.glifos);
                for s in b.iter_mut() {
                    if activo {
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
        Paragraph::new(descripcion)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1))
                    .title(Span::styled(" QUÉ HACE ", titulo(false)))
                    .border_style(marco(false)),
            ),
        columnas[2],
    );

    f.render_widget(
        Paragraph::new(barra_teclas(&[
            ("↑↓", "elegir"),
            ("←→", "ajustar"),
            ("ENTER", "confirmar"),
            ("ESC", "volver"),
        ]))
        .alignment(Alignment::Center),
        principal[1],
    );
}


