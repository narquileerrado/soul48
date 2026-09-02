//! La pantalla de exploración: mapa, medidores, inventario e historial.

use super::widgets::*;
use crate::app::{App, Point};
use crate::settings::Glifos;
use crate::theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear, Paragraph,
    },
    Frame,
};

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
            Constraint::Length(7), // TU ALMA
            Constraint::Length(4), // LO QUE LLEVÁS PUESTO
            Constraint::Length(9), // LO QUE CARGÁS
            Constraint::Min(0),    // LO QUE TE RODEA
        ])
        .split(cuerpo[1]);

    cinta(f, app, filas[0]);
    mapa(f, app, cuerpo[0], glifos);
    alma(f, app, columna[0], glifos);
    equipo(f, app, columna[1]);
    cargas(f, app, columna[2]);
    rodea(f, app, columna[3]);
    historial(f, app, filas[2]);

    let teclas = if app.drop_mode {
        barra_teclas(&[("1-9", "soltar"), ("D", "cancelar")])
    } else {
        barra_teclas(&[
            ("←↑→↓", "moverte"),
            ("1-9", "usar"),
            ("E", "embestida"),
            ("B", "bloqueo"),
            ("D", "soltar"),
            ("S", "bajar"),
            ("Q", "silencio"),
        ])
    };
    f.render_widget(Paragraph::new(teclas), filas[3]);

    if app.show_descend_prompt {
        modal_descenso(f, app, cuerpo[0]);
    }
}

/// Cinta superior: quién sos, dónde estás, qué te está pasando y con qué semilla.
fn cinta(f: &mut Frame, app: &App, area: Rect) {
    let partes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(32),
            Constraint::Min(0),
            Constraint::Length(30),
        ])
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
        Span::styled("  ·  ", Style::default().fg(theme::CENIZA_HONDA)),
        Span::styled(
            format!("NIVEL {}", app.player.level),
            Style::default().fg(theme::HUESO),
        ),
    ]);
    f.render_widget(Paragraph::new(izq), partes[0]);

    // lo transitorio va acá: pide atención y no come layout cuando no pasa nada
    let mut estados: Vec<Span> = vec![Span::raw(" ")];
    for ef in &app.player.status_effects {
        let (color, nombre) = color_efecto(&ef.effect_type);
        estados.push(Span::styled(
            format!("{} {}  ", nombre, ef.duration),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
    }
    if app.player.parry_active {
        estados.push(Span::styled(
            "PARADA  ",
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        ));
    }
    if app.player.invisible_turns > 0 {
        estados.push(Span::styled(
            format!("INVISIBLE {}  ", app.player.invisible_turns),
            Style::default()
                .fg(theme::VIOLETA)
                .add_modifier(Modifier::BOLD),
        ));
    }
    f.render_widget(Paragraph::new(Line::from(estados)), partes[1]);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("SEMILLA {} ", app.seed),
            Style::default().fg(theme::CENIZA_HONDA),
        )))
        .alignment(Alignment::Right),
        partes[2],
    );
}

/// El mapa. Lo visible en color pleno, lo recordado apagado pero con su propio tono.
fn mapa(f: &mut Frame, app: &App, area: Rect, glifos: Glifos) {
    let brillo = app.settings.penumbra;
    // una tabla por cuadro en vez de recorrer las entidades por cada celda
    let indice = app.indice_entidades();
    let mut lineas: Vec<Line> = Vec::with_capacity(app.map.len());

    for (y, fila) in app.map.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::with_capacity(fila.len());
        for (x, &tile) in fila.iter().enumerate() {
            let pos = Point::new(x, y);

            if pos == app.player.pos {
                let color = if app.player.hp * 4 <= app.player.max_hp {
                    theme::ROJO_ALTAR
                } else {
                    theme::AZUL_ALMA
                };
                spans.push(Span::styled(
                    "@",
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            } else if app.visible[y][x] {
                if let Some(e) = indice.get(&pos).map(|i| &app.entities[*i]) {
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
                let estatico = indice
                    .get(&pos)
                    .map(|i| &app.entities[*i])
                    .filter(|e| App::es_estatico(&e.e_type));
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

    // el destello rojo al recibir daño se conserva: pasa a teñir el marco con foco
    let herido = app.player.damage_flash_turns > 0;
    let borde = if herido {
        Style::default().fg(theme::ROJO_ALTAR)
    } else {
        marco(true)
    };

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
                .border_style(borde),
        ),
        area,
    );
}

/// Vitales: alma, cordura, experiencia y atributos.
fn alma(f: &mut Frame, app: &App, area: Rect, glifos: Glifos) {
    let interior = area.width.saturating_sub(2) as usize;
    let v_alma = format!("{}/{}", app.player.hp, app.player.max_hp);
    let v_cor = format!("{}/{}", app.player.sanity, app.player.max_sanity_total());
    let v_xp = format!("{}/{}", app.player.xp, app.player.next_level_xp);
    let ancho_valor = v_alma
        .chars()
        .count()
        .max(v_cor.chars().count())
        .max(v_xp.chars().count());

    let lineas = vec![
        fila_medidor(
            interior,
            "ALMA",
            app.player.hp as f64 / app.player.max_hp.max(1) as f64,
            &v_alma,
            ancho_valor,
            EstiloBarra::new(theme::AZUL_ALMA, theme::AZUL_APAGADO, glifos),
        ),
        fila_medidor(
            interior,
            "CORDURA",
            app.player.sanity as f64 / app.player.max_sanity_total().max(1) as f64,
            &v_cor,
            ancho_valor,
            EstiloBarra::new(theme::VIOLETA, theme::VIOLETA_APAGADO, glifos),
        ),
        fila_medidor(
            interior,
            "XP",
            app.player.xp as f64 / app.player.next_level_xp.max(1) as f64,
            &v_xp,
            ancho_valor,
            EstiloBarra::new(theme::ORO, theme::ORO_APAGADO, glifos),
        ),
        Line::from(""),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("FUE ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                pad_der(&app.player.stats.strength.to_string(), 4),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled("AGI ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                pad_der(&app.player.stats.agility.to_string(), 4),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled("VOL ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                app.player.stats.willpower.to_string(),
                Style::default().fg(theme::HUESO),
            ),
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

/// Las cinco ranuras de equipo, sin que se corte ninguna.
fn equipo(f: &mut Frame, app: &App, area: Rect) {
    let interior = area.width.saturating_sub(2) as usize;

    let (arma, dano) = app
        .player
        .equipment
        .weapon
        .as_ref()
        .map(|w| (w.0.clone(), format!("{}-{}", w.1, w.2)))
        .unwrap_or_else(|| ("Puños".to_string(), "1-3".to_string()));
    let ancho_nombre = interior.saturating_sub(8 + dano.chars().count());

    let ranura = |v: &Option<(String, i32)>| -> String {
        v.as_ref().map(|x| x.1.to_string()).unwrap_or("-".into())
    };

    let lineas = vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled(pad_der("ARMA", 6), Style::default().fg(theme::CENIZA)),
            Span::styled(
                pad_der(&recortar(&arma, ancho_nombre), ancho_nombre),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled(format!("{} ", dano), Style::default().fg(theme::CENIZA)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("ARM ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                pad_der(&ranura(&app.player.equipment.armor), 3),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled("CAS ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                pad_der(&ranura(&app.player.equipment.helmet), 3),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled("ANI ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                pad_der(&ranura(&app.player.equipment.ring), 3),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled("AMU ", Style::default().fg(theme::CENIZA)),
            Span::styled(
                ranura(&app.player.equipment.amulet),
                Style::default().fg(theme::HUESO),
            ),
        ]),
    ];

    f.render_widget(
        Paragraph::new(lineas).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" LO QUE LLEVÁS PUESTO ", titulo(false)))
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

    let mut lineas: Vec<Line> = Vec::new();
    for (i, (item, cuenta)) in app.inventory.iter().enumerate() {
        let meta = if *cuenta > 1 {
            format!("x{}", cuenta)
        } else {
            String::new()
        };
        let ancho_nombre = interior.saturating_sub(5 + meta.chars().count());
        lineas.push(Line::from(vec![
            Span::raw(" "),
            Span::styled(
                format!("{}", i + 1),
                Style::default()
                    .fg(color_numero)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                pad_der(&recortar(&item.name, ancho_nombre), ancho_nombre),
                Style::default().fg(theme::HUESO),
            ),
            Span::styled(format!("{} ", meta), Style::default().fg(theme::CENIZA)),
        ]));
    }

    let libres = crate::balance::objetos::SLOTS_INVENTARIO.saturating_sub(app.inventory.len());
    if libres > 0 && app.inventory.len() < 6 {
        let texto = format!("{} ranuras libres", libres);
        let margen = interior.saturating_sub(texto.chars().count()) / 2;
        lineas.push(Line::from(""));
        lineas.push(Line::from(Span::styled(
            format!("{}{}", " ".repeat(margen), texto),
            Style::default().fg(theme::CENIZA_HONDA),
        )));
    }

    let (nombre, estilo_borde, estilo_titulo) = if app.drop_mode {
        (
            " LO QUE SUELTAS ",
            Style::default().fg(theme::AMBAR),
            Style::default()
                .fg(theme::AMBAR)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (" LO QUE CARGÁS ", marco(false), titulo(false))
    };

    f.render_widget(
        Paragraph::new(lineas).block(
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

    for a in cerca {
        let d = format!("{}", a.distancia);
        // las criaturas heridas se notan: tres bloques de vida junto al nombre
        let medidor = a.vida.map(|v| ((v * 3.0).ceil() as usize).clamp(0, 3));
        let ancho_medidor = if medidor.is_some() { 4 } else { 0 };
        let ancho_nombre = interior.saturating_sub(4 + ancho_medidor + d.chars().count());

        let mut spans = vec![
            Span::raw(" "),
            Span::styled(
                a.glifo.to_string(),
                Style::default().fg(a.color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                pad_der(&recortar(a.nombre, ancho_nombre), ancho_nombre),
                Style::default().fg(theme::HUESO),
            ),
        ];
        if let Some(n) = medidor {
            let color = if n <= 1 {
                theme::ROJO_ALTAR
            } else {
                theme::CENIZA
            };
            spans.push(Span::styled(
                format!("{}{} ", "▮".repeat(n), "▯".repeat(3 - n)),
                Style::default().fg(color),
            ));
        }
        spans.push(Span::styled(
            format!("{} ", d),
            Style::default().fg(theme::CENIZA),
        ));
        lineas.push(Line::from(spans));
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
    // el historial guarda mucho más de lo que entra en el panel: se muestran
    // las últimas voces, que son las que importan
    let visibles = app.settings.lineas_susurro.max(1);
    let desde = app.logs.len().saturating_sub(visibles);
    let lineas: Vec<Line> = app
        .logs
        .iter()
        .skip(desde)
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
        Paragraph::new(texto).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" LAS ESCALERAS ", titulo(true)))
                .title_alignment(Alignment::Center)
                .border_style(marco(true)),
        ),
        zona,
    );
}
