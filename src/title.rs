use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, // <-- ¡Aquí está la pieza que faltaba!
};

const LOGO: &str = r#"
 @@@@@@    @@@@@@   @@@  @@@  @@@                       @@@    @@@@@@
@@@@@@@   @@@@@@@@  @@@  @@@  @@@                      @@@@   @@@@@@@@
!@@       @@!  @@@  @@!  @@@  @@!                     @@!@!   @@!  @@@
!@!       !@!  @!@  !@!  @!@  !@!                    !@!!@!   !@!  @!@
!!@@!!    @!@  !@!  @!@  !@!  @!!       @!@!@!@!@   @!! @!!    !@!!@!
 !!@!!!   !@!  !!!  !@!  !!!  !!!       !!!@!@!!!  !!!  !@!    !!@!!!
     !:!  !!:  !!!  !!:  !!!  !!:                  :!!:!:!!:  !!:  !!!
    !:!   :!:  !:!  :!:  !:!   :!:                 !:::!!:::  :!:  !:!
:::: ::   ::::: ::  ::::: ::   :: ::::                  :::   ::::: ::
:: : :     : :  :    : :  :   : :: : :                  :::    : :  :
"#;

pub const SUBTITLE: &str = "--- THE TALKING DEAD ---";

pub const STORY_SUMMARY: &str = "Despiertas en la penumbra del piso 48. No eres más que un eco de quien fuiste, un alma atada a un cuerpo que ya no respira. El demonio que te arrebató la vida te observa desde las profundidades, burlándose de tu silencio. Para recuperar tu voz y tu destino, debes ascender. Pero ten cuidado: en este dominio, hasta las paredes tienen algo que decir, y la muerte es solo el comienzo de una nueva conversación.";

#[derive(Clone, Copy, PartialEq)]
pub enum MainMenuOption {
    StartGame,
    Bestiary,
    LoadGame,
    Options,
    Quit,
}

impl MainMenuOption {
    const ALL: [MainMenuOption; 5] = [
        MainMenuOption::StartGame,
        MainMenuOption::Bestiary,
        MainMenuOption::LoadGame,
        MainMenuOption::Options,
        MainMenuOption::Quit,
    ];

    pub fn all() -> &'static [MainMenuOption] {
        &Self::ALL
    }
    pub fn as_str(&self) -> &str {
        match self {
            MainMenuOption::StartGame => "ASCENDER AL ORIGEN",
            MainMenuOption::Bestiary => "COMPENDIO DE SOMBRAS",
            MainMenuOption::LoadGame => "RECOGER FRAGMENTOS",
            MainMenuOption::Options => "SINTONIZAR ALMA",
            MainMenuOption::Quit => "VOLVER AL SILENCIO",
        }
    }
    pub fn description(&self) -> &str {
        match self {
            MainMenuOption::StartGame => "Inicia tu ascenso desde el piso 1. Recupera tu alma.",
            MainMenuOption::Bestiary => "Estudia a los moradores de las profundidades.",
            MainMenuOption::LoadGame => "Continúa una partida guardada. (Próximamente)",
            MainMenuOption::Options => "Ajusta la configuración de audio y controles.",
            MainMenuOption::Quit => "Abandona el juego y regresa al sistema.",
        }
    }
}

pub fn ui(f: &mut Frame, menu_state: &mut ListState) {
    let size = f.size();
    let soul_blue = Color::Rgb(100, 200, 255);
    let ghost_gray = Color::Rgb(150, 150, 150);
    let ui_style = Style::default().fg(soul_blue);

    // 1. Marco Global sutil
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(50, 50, 50)));
    f.render_widget(outer_block, size);

    // Layout principal: Logo (arriba) y Contenido (abajo)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1) // Margen para no pisar el marco global
        .constraints([
            Constraint::Length(12), // Espacio para el logo
            Constraint::Length(2),  // Subtítulo
            Constraint::Min(0),     // Contenido dinámico
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    // 1. Dibujar Logo
    let logo_paragraph = Paragraph::new(LOGO)
        .alignment(Alignment::Center)
        .style(Style::default().fg(soul_blue).add_modifier(Modifier::BOLD));
    f.render_widget(logo_paragraph, chunks[0]);

    // 2. Dibujar Subtítulo
    let subtitle = Paragraph::new(SUBTITLE).alignment(Alignment::Center).style(
        Style::default()
            .fg(ghost_gray)
            .add_modifier(Modifier::ITALIC),
    );
    f.render_widget(subtitle, chunks[1]);

    // 3. Layout de Contenido (2 columnas)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .margin(2)
        .split(chunks[2]);

    // Columna Izquierda: Menú
    let options = MainMenuOption::all();
    let list_items: Vec<ListItem> = options
        .iter()
        .map(|opt| ListItem::new(format!("  {}", opt.as_str())))
        .collect();

    let menu_list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" CRIPTAS ")
                .border_style(ui_style),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED), // Efecto de iluminación
        )
        .highlight_symbol(" > ");
    f.render_widget(menu_list, content_chunks[0]);

    // Columna Derecha: Lore e Info
    let selected_index = menu_state.selected().unwrap_or(0);
    let selected_option = &options[selected_index];

    let info_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            " DESTINO SELECCIONADO:",
            Style::default().fg(ghost_gray),
        )),
        Line::from(Span::styled(
            format!(" {}", selected_option.description()),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " ───────────────",
            Style::default().fg(soul_blue),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " EL RELATO DEL DIFUNTO:",
            Style::default().fg(ghost_gray),
        )),
        Line::from(Span::styled(
            STORY_SUMMARY,
            Style::default().fg(Color::White),
        )),
    ];

    let info_paragraph = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" CRÓNICA ")
                .border_style(ui_style),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(info_paragraph, content_chunks[1]);

    // 4. Footer
    let footer_text = vec![
        Line::from(Span::styled(
            "Soul 48: The Talking Dead - v0.2.0",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "[ARRIBA/ABAJO] Navegar  [ENTER] Confirmar  [ESC/Q] Salir",
            ui_style,
        )),
    ];
    let footer = Paragraph::new(footer_text).alignment(Alignment::Center);
    f.render_widget(footer, chunks[3]);
}
