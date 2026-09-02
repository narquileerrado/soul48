//! Dibujo de sprites 8-bit en la terminal, por medio bloque.
//!
//! Cada celda pinta **dos píxeles verticales** con `▀` (U+2580): el de arriba
//! va en color de frente y el de abajo en color de fondo. Como la celda de
//! terminal es aproximadamente 1:2, cada píxel queda cuadrado; y como son
//! exactamente dos píxeles con dos colores, no se pierde ningún color.
//!
//! Un sprite de 16x16 ocupa 16 columnas por 8 filas.

use crate::theme;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// Un sprite. El arte se escribe como texto: una línea por fila de píxeles y
/// un carácter por píxel. El punto es transparente.
pub struct Sprite {
    pub arte: &'static [&'static str],
}

/// Colores de un sprite: una rampa de cuatro tonos derivada del color de la
/// criatura, más un acento y algunos colores fijos del tema.
pub struct Paleta {
    tonos: [Color; 4],
    acento: Color,
}

impl Paleta {
    /// Deriva la rampa del color que la entidad ya tiene en el mapa, así el
    /// retrato y el glifo son siempre el mismo color.
    pub fn de(base: Color, acento: Color) -> Paleta {
        Paleta {
            tonos: [
                theme::recordado(base, 35),
                theme::recordado(base, 60),
                theme::recordado(base, 85),
                base,
            ],
            acento,
        }
    }

    fn color(&self, c: char) -> Option<Color> {
        match c {
            '1' => Some(self.tonos[0]),
            '2' => Some(self.tonos[1]),
            '3' => Some(self.tonos[2]),
            '4' => Some(self.tonos[3]),
            'o' => Some(self.acento),
            'h' => Some(theme::HUESO),
            'c' => Some(theme::CENIZA),
            'd' => Some(theme::CENIZA_HONDA),
            'm' => Some(theme::MURO),
            'v' => Some(theme::VIOLETA),
            'r' => Some(theme::ROJO_ALTAR),
            'a' => Some(theme::AZUL_ALMA),
            'g' => Some(theme::ORO),
            _ => None,
        }
    }
}

impl Sprite {
    pub fn ancho(&self) -> usize {
        self.arte.iter().map(|f| f.chars().count()).max().unwrap_or(0)
    }

    /// Alto en filas de terminal: dos píxeles por celda, redondeando hacia arriba.
    pub fn alto_en_celdas(&self) -> u16 {
        ((self.arte.len() + 1) / 2) as u16
    }

    /// Dibuja el sprite. `ascii` cae a caracteres planos para cuando la fuente
    /// no tiene bloques, conservando exactamente el mismo tamaño en celdas.
    pub fn lineas(&self, pal: &Paleta, fondo: Color, ascii: bool) -> Vec<Line<'static>> {
        let filas: Vec<Vec<char>> = self.arte.iter().map(|f| f.chars().collect()).collect();
        let ancho = self.ancho();
        let px = |x: usize, y: usize| -> Option<Color> {
            filas.get(y).and_then(|f| f.get(x)).and_then(|c| pal.color(*c))
        };

        let mut salida = Vec::with_capacity(self.alto_en_celdas() as usize);
        let mut y = 0;
        while y < filas.len() {
            let mut spans = Vec::with_capacity(ancho);
            for x in 0..ancho {
                let arriba = px(x, y);
                let abajo = px(x, y + 1);
                if ascii {
                    let ch = match (arriba.is_some(), abajo.is_some()) {
                        (true, true) => "#",
                        (true, false) => "'",
                        (false, true) => ".",
                        (false, false) => " ",
                    };
                    let color = arriba.or(abajo).unwrap_or(fondo);
                    spans.push(Span::styled(
                        ch.to_string(),
                        Style::default().fg(color).bg(fondo),
                    ));
                } else {
                    // un píxel transparente no puede pintarse: '▀' con color por
                    // defecto sería un bloque sólido. Según cuál de los dos falte
                    // se usa medio bloque de arriba, de abajo, o nada.
                    let span = match (arriba, abajo) {
                        (Some(a), Some(b)) => Span::styled(
                            "▀".to_string(),
                            Style::default().fg(a).bg(b),
                        ),
                        (Some(a), None) => {
                            Span::styled("▀".to_string(), Style::default().fg(a).bg(fondo))
                        }
                        (None, Some(b)) => {
                            Span::styled("▄".to_string(), Style::default().fg(b).bg(fondo))
                        }
                        (None, None) => {
                            Span::styled(" ".to_string(), Style::default().bg(fondo))
                        }
                    };
                    spans.push(span);
                }
            }
            salida.push(Line::from(spans));
            y += 2;
        }
        salida
    }
}
