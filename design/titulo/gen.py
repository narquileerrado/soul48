#!/usr/bin/env python3
"""Genera las pantallas de título como .dc.html sobre una grilla de terminal real.

Cada carácter es una celda: 92 columnas por 36 filas, la misma grilla que ya usa
el canvas de `design/`. Nada de lo que sale de acá es algo que ratatui no pueda
dibujar — sin degradados, sin sombras, sin scanlines: sólo caracteres, un color
de frente y uno de fondo por celda, y negrita.
"""

import html
import os

COLS, ROWS = 92, 36

# --- paleta: los valores exactos de src/theme.rs ---
ORO = "#D4AF37"
ORO_APAGADO = "#6B5A22"
VIOLETA = "#B48CC8"
ROJO = "#FF6464"
AZUL = "#64C8FF"
AZUL_APAGADO = "#2F5F78"
AMBAR = "#E08A3C"
HUESO = "#E8E0D0"
CENIZA = "#8A8175"
CENIZA_HONDA = "#4A453E"
PENUMBRA = "#0E0C0B"
# tono estructural más tenue, ya establecido en el canvas anterior
LINEA = "#2A2521"

# --- tipografía de bloques: 5x7 px, un pixel = medio carácter ---
FUENTE = {
    "S": ["01110", "10001", "10000", "01110", "00001", "10001", "01110"],
    "O": ["01110", "10001", "10001", "10001", "10001", "10001", "01110"],
    "U": ["10001", "10001", "10001", "10001", "10001", "10001", "01110"],
    "L": ["10000", "10000", "10000", "10000", "10000", "10000", "11111"],
    "4": ["00010", "00110", "01010", "10010", "11111", "00010", "00010"],
    "8": ["01110", "10001", "10001", "01110", "10001", "10001", "01110"],
}


class Pantalla:
    """Una grilla de celdas, cada una con carácter, color y negrita."""

    def __init__(self, fondo=PENUMBRA):
        self.fondo = fondo
        self.celdas = [[(" ", CENIZA, False, None) for _ in range(COLS)] for _ in range(ROWS)]

    def poner(self, fila, col, texto, color=CENIZA, bold=False, bg=None):
        for i, ch in enumerate(texto):
            if 0 <= col + i < COLS and 0 <= fila < ROWS:
                self.celdas[fila][col + i] = (ch, color, bold, bg)

    def celda(self, fila, col, ch, color, bold=False, bg=None):
        if 0 <= col < COLS and 0 <= fila < ROWS:
            self.celdas[fila][col] = (ch, color, bold, bg)

    def logo(self, fila, col, tramos, ascii_mode=False):
        """Dibuja texto en bloques. `tramos` es [(texto, color), ...].

        Cada celda pinta dos pixeles verticales con ▀ / ▄ / █, la misma técnica
        de medio bloque que `sprite.rs` usa para los retratos: así los pixeles
        quedan cuadrados y las letras no salen estiradas.

        Con `ascii_mode` cae a # / \' / . exactamente como `Sprite::lineas`
        cuando el ajuste GLIFOS está en ascii: si el logo no sobrevive ese
        cambio, hay que verlo acá y no en la terminal de alguien.
        """
        lleno, alto, bajo = ("#", "'", ".") if ascii_mode else ("█", "▀", "▄")
        # armar el bitmap completo
        ancho = 0
        for texto, _ in tramos:
            for ch in texto:
                ancho += 3 if ch == " " else 6
        ancho = max(ancho - 1, 0)

        bitmap = [[None] * ancho for _ in range(8)]  # 7 de letra + 1 en blanco
        x = 0
        for texto, color in tramos:
            for ch in texto:
                if ch == " ":
                    x += 3
                    continue
                glifo = FUENTE[ch]
                for py, linea in enumerate(glifo):
                    for px, bit in enumerate(linea):
                        if bit == "1":
                            bitmap[py][x + px] = color
                x += 6

        for cf in range(4):
            for cx in range(ancho):
                arriba = bitmap[cf * 2][cx]
                abajo = bitmap[cf * 2 + 1][cx]
                if arriba and abajo:
                    self.celda(fila + cf, col + cx, lleno, arriba)
                elif arriba:
                    self.celda(fila + cf, col + cx, alto, arriba)
                elif abajo:
                    self.celda(fila + cf, col + cx, bajo, abajo)
        return ancho, bitmap

    def html(self):
        filas = []
        for fila in self.celdas:
            partes = []
            run, run_estilo = "", None
            for ch, color, bold, bg in fila:
                estilo = (color, bold, bg)
                if estilo != run_estilo:
                    if run:
                        partes.append(self._span(run, run_estilo))
                    run, run_estilo = "", estilo
                run += ch
            if run.strip() or (run_estilo and run_estilo[2]):
                partes.append(self._span(run.rstrip() if not run_estilo[2] else run, run_estilo))
            filas.append("<div>" + "".join(partes) + "</div>")
        return "\n".join(filas)

    @staticmethod
    def _span(texto, estilo):
        color, bold, bg = estilo
        css = f"color: {color}"
        if bold:
            css += "; font-weight: 600"
        if bg:
            css += f"; background: {bg}"
        return f'<span style="{css}">{html.escape(texto)}</span>'


PLANTILLA = """<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <script src="./support.js"></script>
</head>
<body>
<x-dc>
<helmet>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital,wght@0,400;0,600;1,400;1,600&display=swap">
  <style>
    body {{ margin: 0; background: {fondo}; }}
    a {{ color: #D4AF37; }}
    a:hover {{ color: #E8C766; }}
    .term {{
      font-family: "IBM Plex Mono", "DejaVu Sans Mono", "Liberation Mono", Menlo, Consolas, monospace;
      font-size: 15px;
      line-height: 21px;
      letter-spacing: 0;
      white-space: pre;
      font-variant-ligatures: none;
      font-feature-settings: "liga" 0, "calt" 0;
      color: {hueso};
      -webkit-font-smoothing: antialiased;
    }}
  </style>
</helmet>
<div style="background: {fondo}; padding: 14px 18px; display: flex; flex-direction: column; align-items: flex-start">
  <div class="term">
{contenido}
  </div>
</div>
</x-dc>
</body>
</html>
"""


def escribir(nombre, pantalla):
    ruta = os.path.join(os.path.dirname(os.path.abspath(__file__)), nombre)
    with open(ruta, "w", encoding="utf-8") as f:
        f.write(
            PLANTILLA.format(
                fondo=pantalla.fondo, hueso=HUESO, contenido=pantalla.html()
            )
        )
    print("escrito", nombre)


def espaciado(texto):
    """Letterspacing a la manera de una terminal: separando con espacios."""
    return " ".join(texto)


# ───────────────────────── El Pozo ─────────────────────────

TRAMOS = [
    (1, "LAS CRIPTAS"),
    (13, "LAS CATACUMBAS"),
    (25, "EL ABISMO"),
    (37, "EL SILENCIO"),
    (48, "EL ARCHIDEMONIO"),
]

MENU = [
    "DESCENDER AL ABISMO",
    "RECOGER FRAGMENTOS",
    "COMPENDIO DE SOMBRAS",
    "SINTONIZAR ALMA",
    "VOLVER AL SILENCIO",
]


def pozo(ascii_mode=False):
    p = Pantalla()
    v = "|" if ascii_mode else "│"
    tick = "+-" if ascii_mode else "├─"
    fin = "`-" if ascii_mode else "└─"
    regla = "-" if ascii_mode else "─"
    caret = ">" if ascii_mode else "▸"
    marca = "<" if ascii_mode else "◂"

    # el título: SOUL en azul —vos y tu alma—, 48 en oro —el objetivo
    p.logo(2, 5, [("SOUL", AZUL), (" ", AZUL), ("48", ORO)], ascii_mode)
    p.poner(7, 5, espaciado("THE TALKING DEAD"), CENIZA)
    p.poner(9, 5, regla * 82, LINEA)

    # menú: una fila sí, una no. El único acento es el caret.
    for i, opcion in enumerate(MENU):
        fila = 11 + i * 2
        activo = i == 0
        if activo:
            p.poner(fila, 3, caret, ORO, bold=True)
            p.poner(fila, 5, opcion, HUESO, bold=True)
        else:
            p.poner(fila, 5, opcion, CENIZA)

    # el descenso: los cuatro tramos como una regla vertical
    p.poner(11, 43, "EL DESCENSO", CENIZA_HONDA)
    for i, (piso, nombre) in enumerate(TRAMOS):
        fila = 13 + i * 2
        ultimo = i == len(TRAMOS) - 1
        p.poner(fila, 43, f"{piso:>2}", CENIZA if not ultimo else ORO)
        p.poner(fila, 46, fin if ultimo else tick, ORO_APAGADO)
        p.poner(fila, 49, nombre, HUESO if i == 0 else (ORO if ultimo else CENIZA))
        if not ultimo:
            p.poner(fila + 1, 47, v, ORO_APAGADO)
    p.poner(13, 66, f"{marca} piso 7", AZUL)

    # lo que hace la opción elegida
    p.poner(24, 5, "Bajá desde el umbral hasta el piso 48.", CENIZA)
    p.poner(25, 5, "Recuperá tu voz.", CENIZA)

    p.poner(28, 5, regla * 82, LINEA)
    p.poner(30, 5, "ÚLTIMO FRAGMENTO", CENIZA_HONDA)
    p.poner(30, 24, "PISO", CENIZA_HONDA)
    p.poner(30, 29, "7", HUESO)
    p.poner(30, 31, "·", CENIZA_HONDA)
    p.poner(30, 33, "LAS CRIPTAS", CENIZA)
    p.poner(30, 47, "ALMA", CENIZA_HONDA)
    p.poner(30, 52, "12/40", AZUL)
    p.poner(30, 60, "SEMILLA", CENIZA_HONDA)
    p.poner(30, 68, "4242", CENIZA_HONDA)

    p.poner(33, 5, "↑↓" if not ascii_mode else "^v", ORO, bold=True)
    p.poner(33, 8, "elegir", CENIZA_HONDA)
    p.poner(33, 18, "ENTER", ORO, bold=True)
    p.poner(33, 24, "confirmar", CENIZA_HONDA)
    p.poner(33, 37, "ESC", ORO, bold=True)
    p.poner(33, 41, "salir", CENIZA_HONDA)
    p.poner(33, 70, "Soul 48 · v0.3.0", CENIZA_HONDA)
    return p


# ───────────────────────── Fósforo ─────────────────────────


def fosforo():
    p = Pantalla()
    # marco completo, como un monitor
    p.poner(0, 0, "╔" + "═" * (COLS - 2) + "╗", ORO_APAGADO)
    p.poner(ROWS - 1, 0, "╚" + "═" * (COLS - 2) + "╝", ORO_APAGADO)
    for f in range(1, ROWS - 1):
        p.celda(f, 0, "║", ORO_APAGADO)
        p.celda(f, COLS - 1, "║", ORO_APAGADO)

    base = (COLS - 37) // 2
    ancho, bitmap = p.logo(4, base, [("SOUL", AZUL), (" ", AZUL), ("48", AZUL)])
    # El reflejo de fósforo: una sola fila espejada bajo las letras, apagada.
    # Espeja de verdad —la última fila de pixeles arriba, la anteúltima abajo—;
    # repetir la fila tal cual se leía como un error de dibujo.
    for cx in range(ancho):
        arriba, abajo = bitmap[6][cx], bitmap[5][cx]
        if arriba and abajo:
            p.celda(8, base + cx, "█", AZUL_APAGADO)
        elif arriba:
            p.celda(8, base + cx, "▀", AZUL_APAGADO)
        elif abajo:
            p.celda(8, base + cx, "▄", AZUL_APAGADO)

    sub = espaciado("THE TALKING DEAD")
    p.poner(10, (COLS - len(sub)) // 2, sub, CENIZA)
    p.poner(12, (COLS - 40) // 2, "─" * 40, LINEA)

    ancho_menu = 30
    izq = (COLS - ancho_menu) // 2
    for i, opcion in enumerate(MENU):
        fila = 14 + i
        if i == 0:
            # la selección invertida, como ya la dibuja el juego
            p.poner(fila, izq, " " * ancho_menu, PENUMBRA, bg=ORO)
            p.poner(fila, izq + 2, opcion, PENUMBRA, bold=True, bg=ORO)
        else:
            p.poner(fila, izq + 2, opcion, CENIZA)

    p.poner(20, (COLS - 40) // 2, "─" * 40, LINEA)
    desc = "Bajá desde el umbral hasta el piso 48."
    p.poner(22, (COLS - len(desc)) // 2, desc, CENIZA)

    tira = "PISO 7   ·   LAS CRIPTAS   ·   ALMA 12/40   ·   SEMILLA 4242"
    p.poner(25, (COLS - len(tira)) // 2, tira, CENIZA_HONDA)

    pie = "↑↓ elegir     ENTER confirmar     ESC salir"
    p.poner(31, (COLS - len(pie)) // 2, pie, CENIZA_HONDA)
    p.poner(33, (COLS - 16) // 2, "Soul 48 · v0.3.0", CENIZA_HONDA)
    return p


# ───────────────────────── Penumbra ─────────────────────────


def penumbra():
    p = Pantalla()
    p.logo(4, 10, [("SOUL", HUESO), (" ", HUESO), ("48", HUESO)])
    p.poner(9, 11, espaciado("the talking dead"), CENIZA_HONDA)

    for i, opcion in enumerate(MENU):
        fila = 15 + i * 2
        if i == 0:
            p.poner(fila, 10, "▏", ORO, bold=True)
            p.poner(fila, 12, opcion, HUESO, bold=True)
        else:
            p.poner(fila, 12, opcion, CENIZA)

    p.poner(27, 12, "bajá desde el umbral hasta el piso 48", CENIZA_HONDA)
    p.poner(30, 12, "piso 7   ·   alma 12/40   ·   semilla 4242", CENIZA_HONDA)
    p.poner(33, 12, "↑↓        ⏎        esc", CENIZA_HONDA)
    return p


# ───────────────────── Penumbra, centrada ─────────────────────


def centrar(texto):
    return (COLS - len(texto)) // 2


def penumbra_centrada(ascii_mode=False):
    """La dirección elegida, sobre un eje central.

    Mismo sistema que la versión al margen: casi monocromática, hueso y ceniza,
    sin un solo borde. El oro aparece una vez y en un solo lugar —la opción
    elegida— porque es lo que dice theme.rs: oro es foco, objetivo, selección.

    Centrada, el marcador de selección sobra: una marca al costado rompería la
    simetría, y el color ya dice cuál es. Menos tinta, misma claridad.
    """
    p = Pantalla()

    ancho_logo, _ = p.logo(3, centrar_ancho(37), [("SOUL 48", HUESO)], ascii_mode)

    sub = espaciado("the talking dead")
    p.poner(8, centrar(sub), sub, CENIZA_HONDA)

    # el relato del difunto, pasando rodando: acá se ve un momento del recorrido
    cinta = RELATO[:56]
    p.poner(9, centrar(cinta), cinta, CENIZA_HONDA)

    for i, opcion in enumerate(MENU_CENTRADO):
        fila = 14 + i * 2
        if i == 0:
            p.poner(fila, centrar(opcion), opcion, ORO, bold=True)
        else:
            p.poner(fila, centrar(opcion), opcion, CENIZA)

    desc = "bajá desde el umbral hasta el piso 48"
    p.poner(26, centrar(desc), desc, CENIZA_HONDA)

    tira = "piso 7   ·   alma 12/40   ·   semilla 4242"
    p.poner(29, centrar(tira), tira, CENIZA_HONDA)

    flechas = "^v" if ascii_mode else "↑↓"
    enter = "ENTER" if ascii_mode else "⏎"
    pie = f"{flechas}        {enter}        esc"
    p.poner(32, centrar(pie), pie, CENIZA_HONDA)
    return p


def centrar_ancho(ancho):
    return (COLS - ancho) // 2


RELATO = (
    "Despiertas en el umbral, sin voz. No eres más que un eco de quien fuiste, "
    "un alma atada a un cuerpo que ya no respira."
)

# El orden convencional: empezar, continuar, y recién después lo demás.
# En title.rs hoy el Compendio va segundo.
MENU_CENTRADO = MENU


if __name__ == "__main__":
    # la dirección elegida
    escribir("Main.dc.html", penumbra_centrada())
    escribir("PenumbraAscii.dc.html", penumbra_centrada(ascii_mode=True))
    # lo que quedó en el camino
    escribir("Pozo.dc.html", pozo())
    escribir("Fosforo.dc.html", fosforo())
    escribir("PenumbraIzquierda.dc.html", penumbra())
