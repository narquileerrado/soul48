//! Menú principal: las Criptas.

use crate::arte;
use crate::settings::{Glifos, Settings};
use crate::sprite::Paleta;
use crate::theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListState, Padding, Paragraph, Wrap},
    Frame,
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
            MainMenuOption::LoadGame => "Continúa una partida guardada anteriormente.",
            MainMenuOption::Options => "Sintoniza la penumbra, los glifos y el guardado.",
            MainMenuOption::Quit => "Abandona el juego y regresa al sistema.",
        }
    }
}

pub fn ui(f: &mut Frame, menu_state: &mut ListState, fragmento: &Fragmento, ajustes: &Settings) {
    let size = f.size();
    let seleccionado = menu_state.selected().unwrap_or(0);

    // marco global, apenas insinuado
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::CENIZA_HONDA)),
        size,
    );

    let filas = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(11), // logo
            Constraint::Length(2),  // subtítulo
            Constraint::Min(0),     // contenido
            Constraint::Length(3),  // pie
        ])
        .split(size);

    // el logo sigue siendo azul: el título del juego es tu alma
    f.render_widget(
        Paragraph::new(LOGO).alignment(Alignment::Center).style(
            Style::default()
                .fg(theme::AZUL_ALMA)
                .add_modifier(Modifier::BOLD),
        ),
        filas[0],
    );
    f.render_widget(
        Paragraph::new(SUBTITLE).alignment(Alignment::Center).style(
            Style::default()
                .fg(theme::CENIZA)
                .add_modifier(Modifier::ITALIC),
        ),
        filas[1],
    );

    let contenido = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(34), Constraint::Min(0)])
        .horizontal_margin(2)
        .split(filas[2]);

    let izquierda = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(4)])
        .split(contenido[0]);

    /* --- criptas --- */
    let bloque = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " CRIPTAS ",
            Style::default().fg(theme::CENIZA),
        ))
        .border_style(Style::default().fg(theme::ORO_APAGADO));
    let interior = bloque.inner(izquierda[0]);
    f.render_widget(bloque, izquierda[0]);

    let ancho = interior.width as usize;
    let mut opciones: Vec<Line> = vec![Line::from("")];
    for (i, opt) in MainMenuOption::all().iter().enumerate() {
        if i == seleccionado {
            let texto = format!(" > {}", opt.as_str());
            let relleno = ancho.saturating_sub(texto.chars().count());
            opciones.push(Line::from(Span::styled(
                format!("{}{}", texto, " ".repeat(relleno)),
                Style::default()
                    .fg(theme::PENUMBRA)
                    .bg(theme::ORO)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            opciones.push(Line::from(Span::styled(
                format!("   {}", opt.as_str()),
                Style::default().fg(theme::HUESO),
            )));
        }
        opciones.push(Line::from(""));
    }
    f.render_widget(Paragraph::new(opciones), interior);

    /* --- último fragmento: qué te espera si elegís RECOGER FRAGMENTOS --- */
    let cuerpo_fragmento = match fragmento {
        Some((piso, hp, max_hp, seed)) => {
            let color_alma = if hp * 4 <= *max_hp {
                theme::ROJO_ALTAR
            } else {
                theme::AZUL_ALMA
            };
            vec![
                Line::from(vec![
                    Span::styled(" PISO ", Style::default().fg(theme::CENIZA)),
                    Span::styled(format!("{}", piso), Style::default().fg(theme::HUESO)),
                    Span::styled("    ALMA ", Style::default().fg(theme::CENIZA)),
                    Span::styled(
                        format!("{}/{}", hp, max_hp),
                        Style::default().fg(color_alma),
                    ),
                ]),
                Line::from(vec![
                    Span::styled(" SEMILLA ", Style::default().fg(theme::CENIZA)),
                    Span::styled(
                        format!("{}", seed),
                        Style::default().fg(theme::CENIZA_HONDA),
                    ),
                ]),
            ]
        }
        None => vec![Line::from(Span::styled(
            " sin partida guardada",
            Style::default().fg(theme::CENIZA_HONDA),
        ))],
    };
    f.render_widget(
        Paragraph::new(cuerpo_fragmento).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " ÚLTIMO FRAGMENTO ",
                    Style::default().fg(theme::CENIZA),
                ))
                .border_style(Style::default().fg(theme::ORO_APAGADO)),
        ),
        izquierda[1],
    );

    /* --- crónica --- */
    let opcion = &MainMenuOption::all()[seleccionado];
    let cronica = vec![
        Line::from(Span::styled(
            opcion.as_str(),
            Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            opcion.description(),
            Style::default().fg(theme::HUESO),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "───────────────",
            Style::default().fg(theme::ORO_APAGADO),
        )),
        Line::from(Span::styled(
            "EL RELATO DEL DIFUNTO",
            Style::default().fg(theme::CENIZA),
        )),
        Line::from(""),
        Line::from(Span::styled(
            STORY_SUMMARY,
            Style::default().fg(theme::HUESO),
        )),
    ];

    let bloque_cronica = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .title(Span::styled(
            " CRÓNICA ",
            Style::default().fg(theme::CENIZA),
        ))
        .border_style(Style::default().fg(theme::ORO_APAGADO));
    let interior_cronica = bloque_cronica.inner(contenido[1]);
    f.render_widget(bloque_cronica, contenido[1]);

    // la ilustración entra sólo si sobra lugar: recortar el relato para meter
    // arte sería la decisión equivocada
    let alto_arte = arte::PORTAL.alto_en_celdas() + 1;
    let area_texto = if interior_cronica.height >= 16 + alto_arte {
        let partes = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(alto_arte), Constraint::Min(0)])
            .split(interior_cronica);
        let paleta = Paleta::de(theme::MURO, theme::ORO);
        let lineas = arte::PORTAL.lineas(
            &paleta,
            Color::Reset,
            ajustes.glifos == Glifos::Ascii,
        );
        f.render_widget(
            Paragraph::new(lineas).alignment(Alignment::Center),
            partes[0],
        );
        partes[1]
    } else {
        interior_cronica
    };

    f.render_widget(
        Paragraph::new(cronica).wrap(Wrap { trim: true }),
        area_texto,
    );

    /* --- pie --- */
    let pie = vec![
        Line::from(Span::styled(
            "Soul 48: The Talking Dead — v0.2.0",
            Style::default().fg(theme::CENIZA_HONDA),
        )),
        Line::from(vec![
            Span::styled(
                "↑↓",
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" navegar   ", Style::default().fg(theme::CENIZA_HONDA)),
            Span::styled(
                "ENTER",
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" confirmar   ", Style::default().fg(theme::CENIZA_HONDA)),
            Span::styled(
                "ESC",
                Style::default().fg(theme::ORO).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" salir", Style::default().fg(theme::CENIZA_HONDA)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(pie).alignment(Alignment::Center),
        filas[3],
    );
}
