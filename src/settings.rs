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
    "EL GUARDADO",
    "TORNAR AL PRINCIPIO",
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
                    "de suyo".into()
                } else {
                    "a mano".into()
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
            0 => "Quán tenue queda lo que ya anduvistes. En lo mínimo, el mapa recordado casi desaparece y camináis a ciegas; en lo máximo, el sótano entero queda legible y piérdese la sensación de andar a escuras.",
            1 => "Quántas voces quedan a la vista. Más renglones dexan seguir una plática larga con una pared; menos renglones dan más aire a lo demás.",
            2 => "Puédense dibuxar los muros con caracteres de caxa o en ASCII llano. Si vuestra terminal o vuestra letra careciere de los glifos de caxa, escoged ascii y tórnase todo a # y punto.",
            3 => "Si está de suyo, guárdase la jornada sola al salir. A mano salís sin dexar rastro, y el fragmento anterior queda entero.",
            _ => "Torna todo al principio: penumbra en treinta, cinco renglones de susurro, glifos unicode y guardado de suyo.",
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
