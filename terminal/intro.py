#!/usr/bin/env python3
"""Intro animada de la terminal según el estilo de COSMIC.

Se lanza desde estilo-terminal.sh al abrir una terminal. Dura poco más de un
segundo y se salta al pulsar cualquier tecla (la tecla no se pierde).
"""
import datetime
import os
import random
import select
import shutil
import socket
import sys
import time

ESC = "\x1b["


def rgb(r, g, b):
    return f"{ESC}38;2;{r};{g};{b}m"


RESET = ESC + "0m"
BOLD = ESC + "1m"
out = sys.stdout


def pressed():
    """¿Hay una tecla esperando? No la lee: así le llega luego a la terminal."""
    try:
        return bool(select.select([sys.stdin], [], [], 0)[0])
    except (OSError, ValueError):
        return False


def sleep(s):
    """Espera, pero corta en cuanto se pulsa una tecla. Devuelve True si se cortó."""
    end = time.monotonic() + s
    while time.monotonic() < end:
        if pressed():
            return True
        time.sleep(min(0.01, max(0.0, end - time.monotonic())))
    return False


def user():
    return os.environ.get("USER") or "usuario"


def host():
    return socket.gethostname().split(".")[0]


def type_out(text, color, delay=0.012):
    skipped = False
    out.write(color)
    for ch in text:
        out.write(ch)
        out.flush()
        if not skipped and ch != " ":
            skipped = sleep(delay)
    out.write(RESET + "\n")
    out.flush()
    return skipped


def battery():
    for b in ("BAT0", "BAT1"):
        try:
            with open(f"/sys/class/power_supply/{b}/capacity") as f:
                return f.read().strip() + "%"
        except OSError:
            pass
    return None


# ─────────────────────────── HACKER ───────────────────────────
GLYPHS = "ΞΔΛΣΦΨΩЖЯБДИЛПЦЧШЮλπ0123456789Z:=*+-<>¦|"


def hacker(cols, rows):
    rows = max(6, min(rows - 1, 30))
    drops = [random.randint(-rows, 0) for _ in range(cols)]
    speed = [random.choice((1, 1, 2)) for _ in range(cols)]
    out.write(ESC + "?1049h" + ESC + "?25l" + ESC + "2J")
    skipped = False
    try:
        for _ in range(24):
            frame = []
            for x in range(0, cols, 2):
                y = drops[x]
                for k, (yy, col) in enumerate(((y, rgb(210, 255, 220)), (y - 1, rgb(0, 255, 65)),
                                               (y - 3, rgb(0, 190, 50)), (y - 6, rgb(0, 110, 30)),
                                               (y - 10, rgb(0, 50, 15)))):
                    if 0 <= yy < rows:
                        frame.append(f"{ESC}{yy + 1};{x + 1}H{col}{random.choice(GLYPHS)}")
                drops[x] = y + speed[x] if y < rows + 12 else random.randint(-8, 0)
            out.write("".join(frame))
            out.flush()
            if sleep(0.045):
                skipped = True
                break
    finally:
        out.write(RESET + ESC + "?1049l" + ESC + "?25h")
        out.flush()
    green, dim = rgb(0, 255, 65), rgb(0, 150, 45)
    if not skipped:
        skipped = type_out(f" Despierta, {user()}...", green, 0.02)
    else:
        out.write(f"{green} Despierta, {user()}...{RESET}\n")
    if not skipped:
        type_out(" Matrix te tiene.", green, 0.02)
    else:
        out.write(f"{green} Matrix te tiene.{RESET}\n")
    now = datetime.datetime.now().strftime("%H:%M")
    out.write(f"{dim} [{host()}] · {now} · sigue al conejo blanco{RESET}\n\n")


# ────────────────────────── CYBERPUNK ─────────────────────────
FONT = {
    "N": ["███╗   ██╗", "████╗  ██║", "██╔██╗ ██║", "██║╚██╗██║", "██║ ╚████║", "╚═╝  ╚═══╝"],
    "I": ["██╗", "██║", "██║", "██║", "██║", "╚═╝"],
    "G": [" ██████╗ ", "██╔════╝ ", "██║  ███╗", "██║   ██║", "╚██████╔╝", " ╚═════╝ "],
    "H": ["██╗  ██╗", "██║  ██║", "███████║", "██╔══██║", "██║  ██║", "╚═╝  ╚═╝"],
    "T": ["████████╗", "╚══██╔══╝", "   ██║   ", "   ██║   ", "   ██║   ", "   ╚═╝   "],
    "C": [" ██████╗", "██╔════╝", "██║     ", "██║     ", "╚██████╗", " ╚═════╝"],
    "Y": ["██╗   ██╗", "╚██╗ ██╔╝", " ╚████╔╝ ", "  ╚██╔╝  ", "   ██║   ", "   ╚═╝   "],
    " ": ["  ", "  ", "  ", "  ", "  ", "  "],
}
NOISE = "▓▒░█▄▀■□▪◢◣◤◥#%&@$*+=<>/\\|"


def banner(text):
    return ["".join(FONT[c][r] for c in text) for r in range(6)]


def cyberpunk(cols, rows):
    magenta, cyan, yellow, lav = rgb(255, 43, 214), rgb(0, 240, 255), rgb(252, 238, 10), rgb(200, 180, 240)
    lines = banner("NIGHT CITY")
    width = len(lines[0])
    if cols < width + 2:
        out.write(f"{magenta}▓▒░ {cyan}{BOLD}NIGHT CITY{RESET}{magenta} ░▒▓{RESET}\n")
    else:
        pad = " "
        grad = [rgb(255, 43, 214), rgb(225, 70, 230), rgb(180, 100, 245), rgb(120, 150, 255), rgb(50, 200, 255), rgb(0, 240, 255)]
        out.write(ESC + "?25l")
        skipped = False
        steps = 14
        for s in range(steps + 1):
            reveal = s / steps
            if s:
                out.write(f"{ESC}6A")
            for r, line in enumerate(lines):
                buf = []
                for ch in line:
                    if ch != " " and random.random() > reveal:
                        buf.append(random.choice((magenta, cyan, yellow)) + random.choice(NOISE))
                    else:
                        buf.append(grad[r] + ch)
                # desplazamiento de glitch en algunas líneas
                shift = " " * random.randint(0, 3) if (s < steps and random.random() < 0.25) else ""
                out.write("\r" + ESC + "2K" + pad + shift + "".join(buf) + RESET + "\n")
            out.flush()
            if s < steps and sleep(0.05):
                skipped = True
                break
        if skipped:
            out.write(f"{ESC}6A")
            for r, line in enumerate(lines):
                out.write("\r" + ESC + "2K" + pad + grad[r] + line + RESET + "\n")
        out.write(ESC + "?25h")
    now = datetime.datetime.now().strftime("%H:%M")
    bat = battery()
    parts = [f"{cyan}NETRUNNER {lav}{user()}", f"{cyan}NODO {lav}{host()}", f"{cyan}HORA {lav}{now}"]
    if bat:
        parts.append(f"{cyan}BATERÍA {lav}{bat}")
    out.write(f" {magenta}▌{RESET} " + f"  {magenta}//{RESET} ".join(parts) + f" {magenta}▐{RESET}\n")
    out.write(f" {yellow}» Bienvenido a Night City. No te fíes de nadie.{RESET}\n\n")


# ──────────────────────────── MACOS ───────────────────────────
DIAS = ["lun", "mar", "mié", "jue", "vie", "sáb", "dom"]
MESES = ["ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic"]


def macos(cols, rows):
    state = os.path.expanduser("~/.cache/estilos/ultimo-inicio")
    last = None
    try:
        with open(state) as f:
            last = datetime.datetime.fromisoformat(f.read().strip())
    except (OSError, ValueError):
        pass
    try:
        os.makedirs(os.path.dirname(state), exist_ok=True)
        with open(state, "w") as f:
            f.write(datetime.datetime.now().isoformat(timespec="seconds"))
    except OSError:
        pass
    if last:
        tty = os.path.basename(os.ttyname(0)) if os.isatty(0) else "ttys000"
        when = f"{DIAS[last.weekday()]} {last.day} {MESES[last.month - 1]} {last:%H:%M:%S}"
        out.write(f"Último inicio de sesión: {when} en {tty}\n")


# ─────────────────────────── WINDOWS ──────────────────────────
def windows(cols, rows):
    blue, gray = rgb(76, 194, 255), rgb(204, 204, 204)
    out.write(f"{gray}PowerShell 7 {blue}·{gray} estilo Windows{RESET}\n")
    out.write(f"{gray}Escribe «help» para obtener ayuda.{RESET}\n\n")


# ───────────────────────── MINIMALISTA ────────────────────────
def minimalista(cols, rows):
    h = datetime.datetime.now().hour
    saludo = "buenos días" if 6 <= h < 13 else ("buenas tardes" if 13 <= h < 21 else "buenas noches")
    text = f"  {saludo}, {user()}."
    out.write(ESC + "?25l")
    for c in ((76, 86, 106), (97, 110, 136), (129, 161, 193), (136, 192, 208)):
        out.write("\r" + rgb(*c) + text + RESET)
        out.flush()
        if sleep(0.06):
            break
    out.write("\r" + rgb(136, 192, 208) + text + RESET + "\n\n" + ESC + "?25h")


INTROS = {"hacker": hacker, "cyberpunk": cyberpunk, "macos": macos, "windows": windows, "minimalista": minimalista}


def main():
    style = sys.argv[1] if len(sys.argv) > 1 else ""
    fn = INTROS.get(style)
    if not fn or not out.isatty():
        return
    size = shutil.get_terminal_size((80, 24))
    try:
        fn(size.columns, size.lines)
    except (KeyboardInterrupt, BrokenPipeError):
        pass
    finally:
        out.write(RESET + ESC + "?25h")
        out.flush()


if __name__ == "__main__":
    main()
