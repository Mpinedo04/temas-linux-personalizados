#!/usr/bin/env bash
# Instala la versión 2 de los estilos de COSMIC:
#   - temas de color y de terminal (incluido el nuevo «cyberpunk»)
#   - fondos de pantalla animados (fondo-vivo) con transición al cambiar de estilo
#   - intro animada y prompt de la terminal según el estilo
#   - el estilo «cyberpunk» y su acceso en el menú
#
# No pide contraseña, no toca ~/.config/cosmic ni el estilo «original».
# Uso: bash instalar.sh

set -u

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ESTILOS="$HOME/.local/share/estilos"
BIN="$HOME/.local/bin"
APPS="$HOME/.local/share/applications"

carpeta_usuario() {
    # $1 = DOCUMENTS o PICTURES; $2 = nombre en español por defecto
    local d=""
    command -v xdg-user-dir > /dev/null && d="$(xdg-user-dir "$1" 2> /dev/null)"
    if [[ -z $d || $d == "$HOME" ]]; then d="$HOME/$2"; fi
    printf '%s' "$d"
}
# las mismas rutas que usa el comando «estilo»; si no existen, las del sistema
DOCS="$HOME/Documentos/Estilos"
[[ -d $DOCS ]] || DOCS="$(carpeta_usuario DOCUMENTS Documentos)/Estilos"
FONDOS="$HOME/Imágenes/Fondos-estilos"
[[ -d $FONDOS ]] || FONDOS="$(carpeta_usuario PICTURES Imágenes)/Fondos-estilos"

paso() { printf '\n\033[1;36m» %s\033[0m\n' "$1"; }
bien() { printf '  \033[32m✔\033[0m %s\n' "$1"; }
aviso() { printf '  \033[33m!\033[0m %s\n' "$1"; }

echo "Instalando los estilos de COSMIC (versión 2)…"
[[ ${XDG_CURRENT_DESKTOP:-} == *COSMIC* ]] || aviso "No parece que estés en el escritorio COSMIC; sigo igualmente."

# ── 1. Temas ────────────────────────────────────────────────────────────────
paso "Temas de color y de terminal"
mkdir -p "$DOCS"
if [[ "$(realpath "$REPO")" != "$(realpath "$DOCS")" ]]; then
    for d in "Temas de color" "Temas de terminal" "Fondos" "herramientas" "sistema"; do
        mkdir -p "$DOCS/$d"
        cp -r "$REPO/$d/." "$DOCS/$d/"
    done
    cp "$REPO/README.md" "$REPO/CLAUDE.md" "$REPO/LEEME - Si algo va mal.txt" "$DOCS/"
fi
bien "Copiados en $DOCS"

# ── 2. Fondos animados ──────────────────────────────────────────────────────
paso "Fondos animados"
mkdir -p "$BIN"
install -m 755 "$REPO/bin/fondo-vivo" "$BIN/fondo-vivo"
install -m 755 "$REPO/scripts/estilo-animaciones" "$BIN/estilo-animaciones"
if "$BIN/fondo-vivo" --version > /dev/null 2>&1; then
    bien "Programa fondo-vivo instalado en $BIN"
else
    aviso "fondo-vivo no arranca en este equipo; las animaciones del fondo no funcionarán."
fi
# Las escenas ya van dentro del programa. Esta copia es de referencia: para
# retocar una, cópiala a ~/.local/share/estilos/fondos-vivos/<estilo>.frag
mkdir -p "$ESTILOS/fondos-vivos/originales" "$HOME/.config/estilos"
cp "$REPO/fondos-vivos/shaders/"* "$ESTILOS/fondos-vivos/originales/"
if [[ ! -f $HOME/.config/estilos/fondos-vivos.conf ]]; then
    cat > "$HOME/.config/estilos/fondos-vivos.conf" << 'EOF'
# Ajustes de los fondos animados (se aplican al momento al guardar).
# fps: fotogramas por segundo (5 a 60). Menos = menos batería.
fps=30
# calidad: baja, media o alta.
calidad=media
# destello: efecto por encima de las ventanas al cambiar de estilo (si / no).
destello=si
EOF
fi
bien "Ajustes en ~/.config/estilos/fondos-vivos.conf"

# ── 3. Terminal ─────────────────────────────────────────────────────────────
paso "Terminal (intro animada y prompt)"
mkdir -p "$ESTILOS/terminal"
cp "$REPO/terminal/intro.py" "$REPO/terminal/estilo-terminal.sh" "$ESTILOS/terminal/"
BASHRC="$HOME/.bashrc"
if ! grep -q ">>> estilos-terminal >>>" "$BASHRC" 2> /dev/null; then
    cat >> "$BASHRC" << 'EOF'

# >>> estilos-terminal >>>
[ -f "$HOME/.local/share/estilos/terminal/estilo-terminal.sh" ] && . "$HOME/.local/share/estilos/terminal/estilo-terminal.sh"
# <<< estilos-terminal <<<
EOF
    bien "Añadida una línea a ~/.bashrc"
else
    bien "~/.bashrc ya estaba preparado"
fi

# ── 4. Comando «estilo» (solo si falta, p. ej. en un equipo nuevo) ───────────
if [[ ! -x $BIN/estilo ]]; then
    paso "Comando estilo"
    install -m 755 "$REPO/sistema/estilo" "$BIN/estilo"
    mkdir -p "$APPS"
    for f in "$REPO/sistema/aplicaciones/"*.desktop; do
        sed "s#/home/[^/]*/.local/bin#$BIN#g" "$f" > "$APPS/$(basename "$f")"
    done
    bien "Instalado en $BIN/estilo, con sus accesos en el menú"
fi

# ── 5. Estilo cyberpunk ─────────────────────────────────────────────────────
paso "Estilo cyberpunk"
mkdir -p "$FONDOS"
cp "$REPO/Fondos/cyberpunk.jpg" "$FONDOS/cyberpunk.jpg"
bien "Fondo estático en $FONDOS/cyberpunk.jpg"

if [[ -d $ESTILOS/cyberpunk ]]; then
    bien "El estilo cyberpunk ya existía; no lo toco"
else
    base=""
    for b in hacker windows minimalista macos; do
        [[ -d $ESTILOS/$b ]] && { base=$b; break; }
    done
    if [[ -n $base ]]; then
        cp -a "$ESTILOS/$base" "$ESTILOS/cyberpunk"
        rm -f "$ESTILOS/cyberpunk/.colores-pendientes" "$ESTILOS/cyberpunk/.ron-importado"
        # mismo diseño de barra y dock que «$base», pero con el fondo de Night City
        while IFS= read -r -d '' f; do
            sed -i -E "s#\"[^\"]*/[^\"/]+\.(png|jpg|jpeg|webp|PNG|JPG)\"#\"$FONDOS/cyberpunk.jpg\"#g" "$f"
        done < <(find "$ESTILOS/cyberpunk" -path '*CosmicBackground*' -type f -print0)
        # la primera vez que se ponga, «estilo» importará cyberpunk.ron él solo
        touch "$ESTILOS/cyberpunk/.colores-pendientes"
        bien "Creado a partir del estilo «$base»"
    else
        aviso "No encuentro ningún estilo en $ESTILOS. Para crear el cyberpunk:"
        aviso "pon el fondo cyberpunk.jpg, ejecuta «estilo guardar cyberpunk» y luego «estilo poner cyberpunk»"
    fi
fi

mkdir -p "$APPS"
ESTILO_CMD="$(command -v estilo || echo "$BIN/estilo")"
if [[ -f $APPS/estilo-hacker.desktop && ! -f $APPS/estilo-cyberpunk.desktop ]]; then
    sed -e 's/hacker/cyberpunk/g' -e 's/Hacker/Cyberpunk/g' -e 's/HACKER/CYBERPUNK/g' \
        "$APPS/estilo-hacker.desktop" > "$APPS/estilo-cyberpunk.desktop"
elif [[ ! -f $APPS/estilo-cyberpunk.desktop ]]; then
    cat > "$APPS/estilo-cyberpunk.desktop" << EOF
[Desktop Entry]
Type=Application
Name=Estilo: Cyberpunk
Comment=Pone el estilo Cyberpunk (Night City)
Exec=$ESTILO_CMD poner cyberpunk
Icon=preferences-desktop-theme
Terminal=false
Categories=Settings;
EOF
fi
cat > "$APPS/estilo-animaciones.desktop" << EOF
[Desktop Entry]
Type=Application
Name=Estilo: Animaciones sí/no
Comment=Enciende o apaga los fondos animados de los estilos
Exec=$BIN/estilo-animaciones alternar
Icon=preferences-desktop-wallpaper
Terminal=false
Categories=Settings;
EOF
command -v update-desktop-database > /dev/null && update-desktop-database "$APPS" 2> /dev/null
bien "Accesos en el menú: «Estilo: Cyberpunk» y «Estilo: Animaciones sí/no»"

# ── 6. Encender ─────────────────────────────────────────────────────────────
paso "Encendiendo las animaciones"
"$BIN/estilo-animaciones" reiniciar > /dev/null   # carga la versión nueva si ya estaba en marcha
"$BIN/estilo-animaciones" si > /dev/null
sleep 1
if "$BIN/estilo-animaciones" estado | grep -q "en marcha"; then
    bien "Fondos animados en marcha"
else
    aviso "Los fondos animados no se han podido arrancar ahora; prueba a cerrar sesión y volver a entrar."
fi

cat << EOF

Listo. Qué hacer ahora:
  · Abre una terminal nueva para ver la intro y el nuevo prompt.
  · Prueba el estilo nuevo desde el menú: «Estilo: Cyberpunk».
    La primera vez tarda un segundo más porque importa sus colores solo.
  · Para apagar o encender las animaciones: «Estilo: Animaciones sí/no».
EOF
