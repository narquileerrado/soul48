//! Cómo encuentran el camino los que te persiguen.
//!
//! Antes cada mob se movía con `(hx - ex).signum()`: el paso que más lo acerca
//! en línea recta. Contra una pared en diagonal eso da siempre la misma casilla
//! bloqueada y el enemigo queda vibrando en el lugar, sin doblar nunca la
//! esquina. Un jugador podía pararse detrás de un recodo y quedarse a salvo.
//!
//! El reemplazo es un *flow field*: un BFS desde el héroe que anota la
//! distancia real —caminando— hasta cada casilla del piso. Con eso, cada mob
//! sólo mira a sus ocho vecinos y se va al de distancia menor. Se calcula una
//! vez por turno para todos: con 60x25 casillas es más barato que lo que hacía
//! antes en cuanto hay unos pocos enemigos.

use super::{App, Point};
use std::collections::VecDeque;

/// Distancias en pasos desde el héroe hasta cada casilla transitable.
///
/// `None` en una casilla quiere decir que no hay forma de llegar caminando:
/// del otro lado de un muro, o de una puerta que todavía está cerrada.
pub struct FlowField {
    distancias: Vec<Option<u32>>,
    ancho: usize,
}

impl FlowField {
    /// Los pasos que faltan desde una casilla hasta el héroe.
    pub fn distancia(&self, pos: Point) -> Option<u32> {
        self.distancias.get(pos.y * self.ancho + pos.x).copied()?
    }
}

/// Las ocho direcciones, incluidas las diagonales.
const VECINOS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

impl App {
    /// Calcula las distancias de todo el piso hasta el héroe.
    ///
    /// Las puertas cerradas cortan el paso, igual que un muro: un enemigo no
    /// debería atravesar lo que al héroe le costó una llave.
    pub fn flow_field(&self) -> FlowField {
        let (alto, ancho) = self.dimensiones();
        let mut distancias = vec![None; alto * ancho];
        let mut cola = VecDeque::new();

        let inicio = self.player.pos;
        if inicio.y < alto && inicio.x < ancho {
            distancias[inicio.y * ancho + inicio.x] = Some(0);
            cola.push_back(inicio);
        }

        while let Some(actual) = cola.pop_front() {
            let d = distancias[actual.y * ancho + actual.x].unwrap_or(0);
            for (dx, dy) in VECINOS {
                let (nx, ny) = (actual.x as isize + dx, actual.y as isize + dy);
                if nx < 0 || ny < 0 || nx >= ancho as isize || ny >= alto as isize {
                    continue;
                }
                let vecino = Point::new(nx as usize, ny as usize);
                let idx = vecino.y * ancho + vecino.x;
                if distancias[idx].is_some() || !self.es_transitable(vecino) {
                    continue;
                }
                if self.bloquea_vision(vecino) {
                    continue; // puerta cerrada: tampoco se pasa caminando
                }
                distancias[idx] = Some(d + 1);
                cola.push_back(vecino);
            }
        }

        FlowField { distancias, ancho }
    }

    /// El paso que acerca a un mob al héroe, o `None` si no hay camino.
    ///
    /// Sigue la pendiente del campo: de los ocho vecinos, el que está más
    /// cerca. Si ninguno mejora, el mob se queda donde está en vez de golpear
    /// la pared.
    pub fn paso_hacia_el_heroe(&self, desde: Point, campo: &FlowField) -> Option<(isize, isize)> {
        let actual = campo.distancia(desde)?;
        let mut mejor: Option<((isize, isize), u32)> = None;

        for (dx, dy) in VECINOS {
            let (nx, ny) = (desde.x as isize + dx, desde.y as isize + dy);
            if nx < 0 || ny < 0 {
                continue;
            }
            let vecino = Point::new(nx as usize, ny as usize);
            let Some(d) = campo.distancia(vecino) else {
                continue;
            };
            if d < actual && mejor.is_none_or(|(_, mejor_d)| d < mejor_d) {
                mejor = Some(((dx, dy), d));
            }
        }
        mejor.map(|(paso, _)| paso)
    }

    /// El paso que más aleja a un mob del héroe: lo que necesita un cobarde.
    ///
    /// Con el mismo campo, pero cuesta arriba. Si está acorralado y ningún
    /// vecino lo aleja, devuelve `None` y el mob aguanta donde está.
    pub fn paso_lejos_del_heroe(&self, desde: Point, campo: &FlowField) -> Option<(isize, isize)> {
        let actual = campo.distancia(desde)?;
        let mut mejor: Option<((isize, isize), u32)> = None;

        for (dx, dy) in VECINOS {
            let (nx, ny) = (desde.x as isize + dx, desde.y as isize + dy);
            if nx < 0 || ny < 0 {
                continue;
            }
            let vecino = Point::new(nx as usize, ny as usize);
            let Some(d) = campo.distancia(vecino) else {
                continue;
            };
            if d > actual && mejor.is_none_or(|(_, mejor_d)| d > mejor_d) {
                mejor = Some(((dx, dy), d));
            }
        }
        mejor.map(|(paso, _)| paso)
    }
}
