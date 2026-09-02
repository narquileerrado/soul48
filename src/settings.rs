//! Ajustes del jugador: la pantalla SINTONIZAR ALMA.
//!
//! Sólo se guardan ajustes que hacen algo hoy. El volumen del eco y de los
//! susurros esperan a que exista el sistema de audio (ver TODO.md).

use serde::{Deserialize, Serialize};

pub const RUTA_AJUSTES: &str = "settings.json";

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Glifos {
    Unicode,
    Ascii,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    /// Cuánto brilla lo ya caminado, de 0 a 100.
    pub penumbra: u8,
    /// Cuántas líneas del historial quedan a la vista.
    pub lineas_susurro: usize,
    /// Muros y suelo con dibujo de caja o en ASCII plano.
    pub glifos: Glifos,
    /// Guardar la partida al salir.
    pub guardado_automatico: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            penumbra: 30,
            lineas_susurro: 5,
            glifos: Glifos::Unicode,
            guardado_automatico: true,
        }
    }
}

/// Filas de la pantalla de ajustes, en orden.
pub const AJUSTES: [&str; 5] = [
    "LA PENUMBRA",
    "LÍNEAS DEL SUSURRO",
    "GLIFOS",
    "GUARDADO",
    "RESTABLECER",
];

impl Settings {
    pub fn load(ruta: &str) -> Settings {
        std::fs::read_to_string(ruta)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, ruta: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(ruta, json);
        }
    }

    /// Valor mostrado a la derecha de cada fila.
    pub fn valor(&self, idx: usize) -> String {
        match idx {
            0 => format!("{}", self.penumbra),
            1 => format!("{}", self.lineas_susurro),
            2 => match self.glifos {
                Glifos::Unicode => "unicode".into(),
                Glifos::Ascii => "ascii".into(),
            },
            3 => {
                if self.guardado_automatico {
                    "automático".into()
                } else {
                    "manual".into()
                }
            }
            _ => String::new(),
        }
    }

    /// Proporción para las filas que se dibujan como barra (None si no lleva barra).
    pub fn proporcion(&self, idx: usize) -> Option<f64> {
        match idx {
            0 => Some(self.penumbra as f64 / 100.0),
            1 => Some((self.lineas_susurro.saturating_sub(3)) as f64 / 5.0),
            _ => None,
        }
    }

    pub fn descripcion(idx: usize) -> &'static str {
        match idx {
            0 => "Qué tan tenue queda lo que ya caminaste. Al mínimo, el mapa recordado casi desaparece y avanzás a ciegas; al máximo, el piso entero queda legible y se pierde la sensación de estar a oscuras.",
            1 => "Cuántas voces quedan a la vista en el historial. Más líneas dejan seguir una conversación larga con una pared; menos líneas dejan más aire al resto de la pantalla.",
            2 => "Los muros pueden dibujarse con caracteres de caja o en ASCII plano. Si tu terminal o tu fuente no tienen los glifos de caja, elegí ascii y todo vuelve a ser # y punto.",
            3 => "Si está en automático, la partida se guarda sola al salir. En manual salís sin dejar rastro y el fragmento anterior queda intacto.",
            _ => "Vuelve todo al principio: penumbra al 30, cinco líneas de susurro, glifos unicode y guardado automático.",
        }
    }

    /// Mueve el valor de una fila. `delta` es -1 o 1.
    pub fn ajustar(&mut self, idx: usize, delta: i32) {
        match idx {
            0 => {
                let v = self.penumbra as i32 + delta * 10;
                self.penumbra = v.clamp(0, 100) as u8;
            }
            1 => {
                let opciones = [3usize, 5, 8];
                let actual = opciones
                    .iter()
                    .position(|o| *o == self.lineas_susurro)
                    .unwrap_or(1);
                let siguiente =
                    (actual as i32 + delta).clamp(0, opciones.len() as i32 - 1) as usize;
                self.lineas_susurro = opciones[siguiente];
            }
            2 => {
                self.glifos = match self.glifos {
                    Glifos::Unicode => Glifos::Ascii,
                    Glifos::Ascii => Glifos::Unicode,
                };
            }
            3 => self.guardado_automatico = !self.guardado_automatico,
            _ => {}
        }
    }

    pub fn restablecer(&mut self) {
        *self = Settings::default();
    }
}
