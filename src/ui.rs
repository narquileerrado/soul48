use crate::app::{App, LogType, Point};
use crate::bestiary::get_bestiary;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(6)])
        .split(f.size());
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(chunks[0]);
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(top_chunks[1]);

    let ui_style = Style::default().fg(Color::Cyan);

    let map_lines: Vec<Line> = app
        .map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let spans: Vec<Span> = row
                .iter()
                .enumerate()
                .map(|(x, &tile)| {
                    let current_pos = Point::new(x, y);
                    if current_pos == app.hero_pos {
                        Span::styled(
                            "@",
                            Style::default().fg(if app.hero_hp < 10 {
                                Color::Red
                            } else {
                                Color::Green
                            }),
                        )
                    } else if app.visible[y][x] {
                        if let Some(e) = app.entities.iter().find(|e| e.pos == current_pos) {
                            Span::styled(e.glyph.to_string(), Style::default().fg(e.color))
                        } else {
                            let color = match tile {
                                '#' | '║' | '═' | '╚' | '╔' | '╝' | '╗' | '╠' | '╣' | '╩' | '╦'
                                | '╬' | '■' => Color::Gray,
                                '+' => Color::Yellow,
                                '>' => Color::LightMagenta,
                                _ => Color::DarkGray,
                            };
                            Span::styled(tile.to_string(), Style::default().fg(color))
                        }
                    } else if app.explored[y][x] {
                        Span::styled(
                            tile.to_string(),
                            Style::default().fg(Color::Rgb(40, 40, 40)),
                        )
                    } else {
                        Span::raw(" ")
                    }
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    f.render_widget(
        Paragraph::new(map_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" MAZMORRA ")
                .border_style(ui_style),
        ),
        top_chunks[0],
    );

    let weapon_text = if let Some(w) = &app.equipped_weapon {
        format!("{} ({}-{})", w.0, w.1, w.2)
    } else {
        "Puños (1-3)".to_string()
    };
    let stats = format!(
        "PISO: {}\nHP: {}/{}\nARMA: {}\nSEED: {}",
        app.depth, app.hero_hp, app.hero_max_hp, weapon_text, app.seed
    );
    f.render_widget(
        Paragraph::new(stats).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" PERSONAJE ")
                .border_style(ui_style),
        ),
        right_chunks[0],
    );

    let inv_lines: Vec<String> = app
        .inventory
        .iter()
        .enumerate()
        .map(|(i, (item, count))| {
            if *count > 1 {
                format!("{}. {} (x{})", i + 1, item.name, count)
            } else {
                format!("{}. {}", i + 1, item.name)
            }
        })
        .collect();

    let inv_border_style = if app.drop_mode {
        Style::default().fg(Color::LightGreen)
    } else {
        ui_style
    };
    let inv_title = if app.drop_mode {
        " INV [SOLTAR] "
    } else {
        " INVENTARIO "
    };
    f.render_widget(
        Paragraph::new(inv_lines.join("\n")).block(
            Block::default()
                .borders(Borders::ALL)
                .title(inv_title)
                .border_style(inv_border_style),
        ),
        right_chunks[1],
    );

    let log_lines: Vec<Line> = app
        .logs
        .iter()
        .map(|msg| {
            let color = match msg.l_type {
                LogType::Combat => Color::Red,
                LogType::Item => Color::Magenta,
                LogType::Warning => Color::Yellow,
                LogType::Info => Color::Cyan,
            };
            Line::from(Span::styled(&msg.text, Style::default().fg(color)))
        })
        .collect();
    f.render_widget(
        Paragraph::new(log_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" HISTORIAL ")
                .border_style(ui_style),
        ),
        chunks[1],
    );
}

pub fn game_over_ui(f: &mut Frame, app: &App) {
    ui(f, app); // Dibuja la UI normal de fondo

    let area = centered_rect(50, 20, f.size());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" FIN DE LA PARTIDA ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            " HAS CAÍDO",
            Style::default()
                .fg(Color::Red)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " El archidemonio ha consumido lo poco que",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            " quedaba de tu alma en este piso.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!(" PISO ALCANZADO: {}", app.depth),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " [R] Reiniciar    [Q] Salir al sistema",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}

pub fn bestiary_ui(f: &mut Frame, list_state: &mut ListState) {
    let size = f.size();
    let bestiary = get_bestiary();
    let antique_gold = Color::Rgb(212, 175, 55);
    let _parchment = Color::Rgb(252, 245, 229);
    let _ink_blue = Color::Rgb(20, 30, 70);

    // Fondo "pergamino"
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(antique_gold))
        .title(" EL COMPENDIO DE LAS SOMBRAS ")
        .title_alignment(Alignment::Center);
    f.render_widget(outer_block, size);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(size);

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_layout[0]);

    // Lista de enemigos
    let items: Vec<ListItem> = bestiary
        .iter()
        .map(|e| {
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", e.glyph), Style::default().fg(e.color)),
                Span::raw(e.name),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" ENTIDADES "))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(antique_gold)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" > ");

    f.render_stateful_widget(list, content_layout[0], list_state);

    // Detalle del enemigo
    let selected_idx = list_state.selected().unwrap_or(0);
    let e = &bestiary[selected_idx];

    let mut details = vec![
        Line::from(vec![
            Span::styled(
                e.name.to_uppercase(),
                Style::default()
                    .fg(antique_gold)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - "),
            Span::styled(
                e.scientific_name,
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(Span::styled(
            e.taxonomy,
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "──────── RELATO ANTIGUO ────────",
            Style::default().fg(antique_gold),
        )),
        Line::from(""),
    ];

    // Wrap description lines
    let _desc_para = Paragraph::new(e.description).wrap(Wrap { trim: true });

    details.push(Line::from(e.description));
    details.push(Line::from(""));
    details.push(Line::from(Span::styled(
        "──────── ATRIBUTOS ────────",
        Style::default().fg(antique_gold),
    )));
    details.push(Line::from(format!(" VITALIDAD: {}", e.base_hp)));
    details.push(Line::from(format!(
        " POTENCIA:  {}-{}",
        e.base_damage.0, e.base_damage.1
    )));
    details.push(Line::from(format!(" DEFENSA:   {}", e.base_defense)));
    details.push(Line::from(format!(" CONDUCTA:  {}", e.behavior)));

    let detail_paragraph = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title(" CRÓNICA "))
        .wrap(Wrap { trim: true });

    f.render_widget(detail_paragraph, content_layout[1]);

    // Footer
    let footer = Paragraph::new("[ARRIBA/ABAJO] Navegar  [ESC/Q] Volver")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, main_layout[1]);
}

fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
