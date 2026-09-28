"""Genera los 4 temas .ron de COSMIC a partir de una especificación por tema.

Las escalas de neutros se calculan en Oklch con la misma distribución de
luminosidad que la paleta de serie de COSMIC, así que los widgets conservan
el contraste que COSMIC espera y solo cambian de tono.
"""
import sys
from pathlib import Path

from color import hex_to_oklch, oklch_to_hex

# Luminosidad Oklch de neutral_0..neutral_10 en la paleta oscura de serie
# (#000000, #1B1B1B, #303030 ... #E2E2E2, #FFFFFF).
DARK_L = [0.0, 0.222, 0.309, 0.398, 0.482, 0.569, 0.657, 0.741, 0.827, 0.913, 1.0]

ALPHA_MAP = [
    ("extremely_low", "0.9"), ("extremely_low_2", "0.87692"),
    ("very_low", "0.85385"), ("very_low_2", "0.83076"),
    ("low", "0.80769"), ("low_2", "0.78461"),
    ("medium", "0.76154"), ("medium_2", "0.73846"),
    ("high", "0.71538"), ("high_2", "0.69231"),
    ("very_high", "0.66023"), ("very_high_2", "0.64615"),
    ("extremely_high", "0.62308"), ("extremely_high_2", "0.6"),
]

SPACING_STANDARD = [0, 4, 8, 12, 16, 24, 32, 48, 64, 128]
# Igual que la densidad "Compacta" de Configuración → Apariencia.
SPACING_COMPACT = [0, 4, 4, 8, 8, 16, 24, 32, 48, 64]
SPACING_KEYS = ["space_none", "space_xxxs", "space_xxs", "space_xs", "space_s",
                "space_m", "space_l", "space_xl", "space_xxl", "space_xxxl"]


def ok(L, C, H):
    return oklch_to_hex(L, C, H)


def neutrals(hue, chroma, light):
    """neutral_0..10: extremos puros (negro/blanco), intermedios tintados."""
    out = []
    for i, L in enumerate(DARK_L):
        if i in (0, 10):
            out.append("#000000" if i == 0 else "#FFFFFF")
        else:
            c = chroma(L) if callable(chroma) else chroma
            out.append(ok(L, c, hue))
    return out[::-1] if light else out


def hx(c):
    """Todos los colores se escriben como #RRGGBBAA, igual que exporta COSMIC."""
    c = c.upper()
    return c if len(c) == 9 else c + "FF"


# ───────────────────────────── HACKER ─────────────────────────────
G = 150  # tono verde fósforo (Oklch)
hacker = dict(
    file="hacker.ron",
    header=[
        'Tema "Hacker" para COSMIC — terminal Matrix / fósforo verde',
        "",
        "Negro casi puro con un leve tinte verde, texto verde fósforo,",
        "acento verde Matrix (#00FF41), avisos en ámbar de monitor antiguo",
        "y errores en rojo neón. Esquinas totalmente rectas, interfaz",
        "compacta (más información en menos espacio) y barra/menús de",
        'cristal oscuro semitransparente, como un "HUD".',
    ],
    mode="Dark",
    name="hacker-matrix",
    accent="#00FF41",
    success="#00FF9C",
    warning="#FFB000",
    destructive="#FF3355",
    bg=ok(0.145, 0.020, G),
    primary=ok(0.185, 0.028, G),
    secondary=ok(0.225, 0.034, G),
    neutral_tint=ok(0.50, 0.075, G),
    text_tint=ok(0.90, 0.140, 146),
    window_hint=None,
    active_hint=2,
    gaps=(0, 8),
    radii=(0, 0, 0, 0, 0),
    frosted="Low",
    frosted_flags=dict(applets=True, panel=True, system_interface=True, windows=False),
    spacing=SPACING_COMPACT,
    neutrals=neutrals(G, 0.075, light=False),
    bright=dict(red="#FF5C7A", green="#39FF14", orange="#FFB000"),
    accents=dict(
        blue="#00E5FF", indigo="#6C7BFF", purple="#C04DFF", pink="#FF2E97",
        red="#FF3355", orange="#FF9F1C", yellow="#FFE500", green="#00FF41",
        warm_grey="#8DB89A",
    ),
    ext=dict(
        warm_grey="#5E7A66", orange="#FF9F1C", yellow="#FFE500", blue="#00B3FF",
        purple="#B14DFF", pink="#FF007F", indigo="#4D5BFF",
    ),
)

# ───────────────────────────── WINDOWS ────────────────────────────
windows = dict(
    file="windows.ron",
    header=[
        'Tema "Windows" para COSMIC — Windows 11 modo oscuro (Fluent)',
        "",
        "Colores oficiales de Windows 11 oscuro: fondo Mica #202020,",
        "tarjetas #2B2B2B y acento azul #4CC2FF. Botones con esquinas",
        "de 4 px y ventanas y menús de 8 px, como en Windows 11 (antes",
        "los botones salían en forma de píldora). Barra de tareas y menús",
        "con efecto acrílico (transparencia con desenfoque).",
    ],
    mode="Dark",
    name="windows-11",
    accent="#4CC2FF",
    success="#6CCB5F",
    warning="#FCE100",
    destructive="#FF99A4",
    bg="#202020",
    primary="#2B2B2B",
    secondary="#323232",
    neutral_tint="#808080",
    text_tint=None,
    window_hint=None,
    active_hint=1,
    gaps=(0, 4),
    # radius_s 4 → ventanas de 8 px (COSMIC suma 4); radius_xl 4 → botones de 4 px.
    radii=(4, 4, 8, 8, 4),
    frosted="Medium",
    frosted_flags=dict(applets=True, panel=True, system_interface=True, windows=False),
    spacing=SPACING_STANDARD,
    neutrals=neutrals(0, 0.0, light=False),
    bright=dict(red="#FF99A4", green="#6CCB5F", orange="#FFA56B"),
    # Colores de énfasis de Windows en su tono claro (el que usa el modo oscuro).
    accents=dict(
        blue="#4CC2FF", indigo="#A5A8FF", purple="#C9A6F2", pink="#FF8CCF",
        red="#FF99A4", orange="#FFA56B", yellow="#FFD34D", green="#6CCB5F",
        warm_grey="#C7BCA8",
    ),
    # Colores de énfasis clásicos de Windows (tono base).
    ext=dict(
        warm_grey="#7E735F", orange="#F7630C", yellow="#FFB900", blue="#0078D4",
        purple="#8764B8", pink="#E3008C", indigo="#6B69D6",
    ),
)

# ────────────────────────────── MACOS ─────────────────────────────
macos = dict(
    file="macos.ron",
    header=[
        'Tema "macOS" para COSMIC — macOS modo claro',
        "",
        "Fondo gris claro de Apple (#F2F2F7) con tarjetas blancas, azul",
        "del sistema (#007AFF) y los colores de énfasis de macOS (azul,",
        "morado, rosa, rojo, naranja, amarillo, verde y grafito).",
        "Botones en forma de cápsula, ventanas muy redondeadas y barra,",
        "dock y menús translúcidos. El borde de la ventana activa es una",
        "línea gris fina, como en un Mac, en vez de un borde de color.",
    ],
    mode="Light",
    name="macos",
    accent="#007AFF",
    # Variantes "alto contraste" de Apple: se leen bien sobre blanco.
    success="#1E7E34",
    warning="#C93400",
    destructive="#D70015",
    bg="#F2F2F7",
    primary="#FFFFFF",
    secondary="#EBEBF0",
    neutral_tint="#8E8E93",
    text_tint=None,
    window_hint="#B4B4BB",
    active_hint=1,
    gaps=(0, 10),
    # radius_s 12 → ventanas de 16 px; radius_xl 160 → botones cápsula.
    radii=(8, 12, 14, 20, 160),
    frosted="Medium",
    frosted_flags=dict(applets=True, panel=True, system_interface=True, windows=False),
    spacing=SPACING_STANDARD,
    neutrals=neutrals(286, 0.007, light=True),
    bright=dict(red="#D70015", green="#1E7E34", orange="#C93400"),
    # Los colores de énfasis que ofrece macOS en Ajustes del Sistema.
    accents=dict(
        blue="#007AFF", indigo="#5856D6", purple="#AF52DE", pink="#FF2D55",
        red="#FF3B30", orange="#FF9500", yellow="#FFCC00", green="#34C759",
        warm_grey="#8E8E93",
    ),
    ext=dict(
        warm_grey="#A2845E", orange="#FF9500", yellow="#FFCC00", blue="#5AC8FA",
        purple="#AF52DE", pink="#FF2D55", indigo="#5856D6",
    ),
)

# ─────────────────────────── MINIMALISTA ──────────────────────────
N = 262  # tono azul pizarra de Nord
nord = dict(
    file="minimalista.ron",
    header=[
        'Tema "Minimalista" para COSMIC — paleta Nord completa',
        "",
        "Los 16 colores de Nord: fondos Polar Night (#2E3440, #3B4252,",
        "#434C5E), texto Snow Storm, acento Frost (#88C0D0) y estados",
        "Aurora. Ahora también los grises internos (campos de texto,",
        "interruptores, barras de desplazamiento) son azul pizarra de Nord",
        "en vez de gris neutro. Diseño plano, sin transparencias, con más",
        "separación entre ventanas.",
    ],
    mode="Dark",
    name="nord",
    accent="#88C0D0",
    success="#A3BE8C",
    warning="#EBCB8B",
    destructive="#D87780",
    bg="#2E3440",
    primary="#3B4252",
    secondary="#434C5E",
    neutral_tint="#4C566A",
    text_tint="#D8DEE9",
    window_hint=None,
    active_hint=1,
    gaps=(0, 12),
    radii=(4, 8, 12, 16, 160),
    frosted="Low",
    frosted_flags=dict(applets=False, panel=False, system_interface=False, windows=False),
    spacing=SPACING_STANDARD,
    # Menos tinte en los tonos claros, como el blanco nieve de Nord.
    neutrals=neutrals(N, lambda L: 0.030 if L < 0.5 else 0.030 - (L - 0.5) * 0.035, light=False),
    bright=dict(red="#BF616A", green="#A3BE8C", orange="#D08770"),
    accents=dict(
        blue="#88C0D0", indigo="#81A1C1", purple="#B48EAD", pink=ok(*hex_to_oklch("#B48EAD")[:2], 350),
        red="#BF616A", orange="#D08770", yellow="#EBCB8B", green="#A3BE8C",
        warm_grey="#D8DEE9",
    ),
    ext=dict(
        warm_grey="#616E88", orange="#D08770", yellow="#EBCB8B", blue="#8FBCBB",
        purple="#B48EAD", pink=ok(*hex_to_oklch("#B48EAD")[:2], 350), indigo="#5E81AC",
    ),
)

THEMES = [hacker, macos, nord, windows]


def opt(c):
    return f'Some("{hx(c)}")' if c else "None"


def render(t):
    n = t["neutrals"]
    radii = t["radii"]
    ff = t["frosted_flags"]
    L = []
    w = L.append
    w("// " + "=" * 66)
    for line in t["header"]:
        w(("// " + line).rstrip())
    w("//")
    w("// Cómo ponerlo: Configuración → Escritorio → Apariencia → Importar")
    w("// y elegir este archivo. Después: Menú → \"Estilo: Guardar cambios\".")
    w("// " + "=" * 66)
    w("(")
    w("    // Color principal: botones destacados, selección, interruptores.")
    w(f"    accent: {opt(t['accent'])},")
    w("    // Grosor en píxeles del borde de la ventana activa.")
    w(f"    active_hint: {t['active_hint']},")
    w("    alpha_map: (")
    for k, v in ALPHA_MAP:
        w(f"        {k}: {v},")
    w("    ),")
    w("    // Fondo de las ventanas.")
    w(f"    bg_color: {opt(t['bg'])},")
    w("    // Redondeo de esquinas. xs: tarjetas · s: campos y ventanas (+4 px)")
    w("    // · m: menús desplegables · l: menús del panel · xl: botones.")
    w("    corner_radii: (")
    w("        radius_0: (0.0, 0.0, 0.0, 0.0),")
    for key, r in zip(["xs", "s", "m", "l", "xl"], radii):
        v = f"{float(r):.1f}"
        w(f"        radius_{key}: ({v}, {v}, {v}, {v}),")
    w("    ),")
    w(f"    destructive: {opt(t['destructive'])},")
    w("    // Transparencia con desenfoque (Low = sutil, Medium = media).")
    w(f"    frosted: {t['frosted']},")
    w(f"    frosted_applets: {str(ff['applets']).lower()},")
    w(f"    frosted_panel: {str(ff['panel']).lower()},")
    w(f"    frosted_system_interface: {str(ff['system_interface']).lower()},")
    w(f"    frosted_windows: {str(ff['windows']).lower()},")
    w("    // Separación entre ventanas en mosaico (exterior, interior).")
    w(f"    gaps: ({t['gaps'][0]}, {t['gaps'][1]}),")
    w("    // Tono de los botones normales y controles.")
    w(f"    neutral_tint: {opt(t['neutral_tint'])},")
    w("    // Paleta interna: la usan campos de texto, interruptores, barras de")
    w("    // desplazamiento, las apps GTK y los colores de énfasis que ofrece")
    w("    // Apariencia (accent_*).")
    w(f"    palette: {t['mode']}((")
    w(f'        name: "{t["name"]}",')
    for k in ("red", "green", "orange"):
        w(f'        bright_{k}: "{hx(t["bright"][k])}",')
    w(f'        gray_1: "{hx(t["bg"])}",')
    w(f'        gray_2: "{hx(t["primary"])}",')
    for i, c in enumerate(n):
        w(f'        neutral_{i}: "{hx(c)}",')
    for k in ("blue", "indigo", "purple", "pink", "red", "orange", "yellow", "green", "warm_grey"):
        w(f'        accent_{k}: "{hx(t["accents"][k])}",')
    for k in ("warm_grey", "orange", "yellow", "blue", "purple", "pink", "indigo"):
        w(f'        ext_{k}: "{hx(t["ext"][k])}",')
    w("    )),")
    w("    // Fondo de barras laterales y tarjetas (primario) y de lo que va")
    w("    // dentro de ellas (secundario).")
    w(f"    primary_container_bg: {opt(t['primary'])},")
    w(f"    secondary_container_bg: {opt(t['secondary'])},")
    w("    spacing: (")
    for k, v in zip(SPACING_KEYS, t["spacing"]):
        w(f"        {k}: {v},")
    w("    ),")
    w(f"    success: {opt(t['success'])},")
    w("    // Tono del texto (None = gris neutro).")
    w(f"    text_tint: {opt(t['text_tint'])},")
    w(f"    warning: {opt(t['warning'])},")
    w("    // Color del borde de la ventana activa (None = color de acento).")
    w(f"    window_hint: {opt(t['window_hint'])},")
    w(")")
    return "\n".join(L) + "\n"


if __name__ == "__main__":
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    for t in THEMES:
        (out / t["file"]).write_text(render(t), encoding="utf-8")
        print("escrito", out / t["file"])
