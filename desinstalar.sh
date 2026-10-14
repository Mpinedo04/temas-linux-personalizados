#!/usr/bin/env bash
# Quita los fondos animados y la intro de la terminal.
# Deja como están los temas, el estilo cyberpunk y el resto de estilos.

set -u
BIN="$HOME/.local/bin"

if [[ -x $BIN/estilo-animaciones ]]; then
    "$BIN/estilo-animaciones" no > /dev/null
else
    rm -f "$HOME/.config/autostart/fondo-vivo.desktop"
    pkill -x -u "$(id -u)" fondo-vivo 2> /dev/null || true
fi
rm -f "$BIN/fondo-vivo" "$BIN/estilo-animaciones" "$HOME/.local/share/applications/estilo-animaciones.desktop"
rm -rf "$HOME/.local/share/estilos/terminal" "$HOME/.local/share/estilos/fondos-vivos"
if [[ -f $HOME/.bashrc ]]; then
    sed -i '/# >>> estilos-terminal >>>/,/# <<< estilos-terminal <<</d' "$HOME/.bashrc"
fi
echo "Quitados los fondos animados y la intro de la terminal."
echo "Los temas, los estilos (también cyberpunk) y tus ajustes siguen igual."
