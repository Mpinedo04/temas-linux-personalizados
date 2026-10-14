"""Esquemas de color para la Terminal de COSMIC, a juego con cada estilo."""
import sys
from pathlib import Path

from color import hex_to_oklch, oklch_to_hex

ANSI = ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"]


def dim_dark(c):
    L, C, H = hex_to_oklch(c)
    return oklch_to_hex(L * 0.72, C * 0.8, H)


def dim_light(c):
    L, C, H = hex_to_oklch(c)
    return oklch_to_hex(L + (1 - L) * 0.35, C * 0.75, H)


SCHEMES = [
    dict(
        file="hacker.ron",
        name="Hacker Matrix",
        light=False,
        foreground="#7DF59A",
        background="#050C06",
        cursor="#00FF41",
        bright_foreground="#C6FFD2",
        dim_foreground="#4E9E60",
        normal=["#0A140C", "#FF3355", "#00E03A", "#FFB000", "#00A2FF", "#C04DFF", "#00D5E8", "#A5E8B2"],
        bright=["#4F7F5A", "#FF6E88", "#00FF41", "#FFD24D", "#4DC3FF", "#D98CFF", "#5CF2FF", "#E6FFEA"],
    ),
    dict(
        file="macos.ron",
        name="macOS Claro",
        light=True,
        foreground="#1D1D1F",
        background="#FFFFFF",
        cursor="#007AFF",
        bright_foreground="#000000",
        dim_foreground="#6E6E73",
        normal=["#1D1D1F", "#D70015", "#1E7E34", "#A05A00", "#0040DD", "#8944AB", "#0071A4", "#C7C7CC"],
        bright=["#6E6E73", "#FF3B30", "#248A3D", "#C93400", "#007AFF", "#AF52DE", "#0088C7", "#F2F2F7"],
    ),
    dict(
        file="minimalista.ron",
        name="Nord",
        light=False,
        foreground="#D8DEE9",
        background="#2E3440",
        cursor="#D8DEE9",
        bright_foreground="#ECEFF4",
        dim_foreground="#A5ABB6",
        normal=["#3B4252", "#BF616A", "#A3BE8C", "#EBCB8B", "#81A1C1", "#B48EAD", "#88C0D0", "#E5E9F0"],
        bright=["#616E88", "#BF616A", "#A3BE8C", "#EBCB8B", "#81A1C1", "#B48EAD", "#8FBCBB", "#ECEFF4"],
        # Colores atenuados oficiales de Nord.
        dim=["#373E4D", "#94545D", "#809575", "#B29E75", "#68809A", "#8C738C", "#6D96A5", "#AEB3BB"],
    ),
    dict(
        file="windows.ron",
        name="Windows Campbell",
        light=False,
        # Esquema "Campbell", el de serie de Windows Terminal.
        foreground="#CCCCCC",
        background="#0C0C0C",
        cursor="#FFFFFF",
        bright_foreground="#F2F2F2",
        dim_foreground="#999999",
        normal=["#0C0C0C", "#C50F1F", "#13A10E", "#C19C00", "#0037DA", "#881798", "#3A96DD", "#CCCCCC"],
        bright=["#767676", "#E74856", "#16C60C", "#F9F1A5", "#3B78FF", "#B4009E", "#61D6D6", "#F2F2F2"],
    ),
    dict(
        file="cyberpunk.ron",
        name="Cyberpunk Night City",
        light=False,
        foreground="#E6D6FF",
        background="#0A0614",
        cursor="#FF2BD6",
        bright_foreground="#FFFFFF",
        dim_foreground="#9C88C4",
        normal=["#1A1030", "#FF2A6D", "#39FF88", "#FCEE0A", "#3D7BFF", "#FF2BD6", "#00F0FF", "#CFC2EE"],
        bright=["#6B5A99", "#FF6B9A", "#7DFFB2", "#FFF58A", "#7FA6FF", "#FF7BEA", "#7FF8FF", "#FFFFFF"],
    ),
]


def render(s):
    dim = s.get("dim") or [(dim_light if s["light"] else dim_dark)(c) for c in s["normal"]]
    out = ["("]
    out.append(f'    name: "{s["name"]}",')
    for k in ("foreground", "background", "cursor", "bright_foreground", "dim_foreground"):
        out.append(f'    {k}: "{s[k].upper()}",')
    for group, colors in (("normal", s["normal"]), ("bright", s["bright"]), ("dim", dim)):
        out.append(f"    {group}: (")
        for k, c in zip(ANSI, colors):
            out.append(f'        {k}: "{c.upper()}",')
        out.append("    ),")
    out.append(")")
    return "\n".join(out) + "\n"


if __name__ == "__main__":
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    for s in SCHEMES:
        (out / s["file"]).write_text(render(s), encoding="utf-8")
        print("escrito", out / s["file"])
