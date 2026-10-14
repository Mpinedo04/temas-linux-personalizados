# Contexto: sistema de "estilos" para el escritorio COSMIC (Pop!_OS / Linux)

Este documento explica a una IA (o a cualquiera) cómo está montado el sistema de
estilos de este escritorio. Para el usuario hay una ayuda más corta en
`LEEME - Si algo va mal.txt`, en esta misma carpeta, y un resumen de las
mejoras en `README.md`.

## Qué es

Un sistema para cambiar el aspecto completo del escritorio COSMIC con un solo
comando. Cada estilo cambia a la vez los colores, la barra y el dock, el fondo de
pantalla, los bordes redondeados y la separación entre ventanas. Desde la
versión 2, cada estilo tiene además un fondo animado, un efecto de transición
al ponerlo y una terminal con intro animada y prompt propios.

Estilos disponibles:

- `original`: copia de seguridad del aspecto inicial. Está protegida y no se puede borrar.
- `macos`
- `windows`
- `minimalista`
- `hacker`
- `cyberpunk` (versión 2; `instalar.sh` lo crea copiando la carpeta de `hacker`)

El estilo puesto en cada momento está en `~/.local/share/estilos/.actual`.

## Piezas del sistema

| Qué | Dónde |
|---|---|
| El comando `estilo` (script bash) | `~/.local/bin/estilo` |
| Estilos guardados (una carpeta por estilo) | `~/.local/share/estilos/<nombre>/` |
| Estilo puesto ahora | `~/.local/share/estilos/.actual` |
| Huella del `.ron` ya importado en cada estilo | `~/.local/share/estilos/<nombre>/.ron-importado` |
| Esquemas y esquema elegido de la Terminal | `~/.config/cosmic/com.system76.CosmicTerm/v1/` (`color_schemes_dark`, `color_schemes_light`, `syntax_theme_dark`, `syntax_theme_light`) |
| Copias de seguridad (antes de las mejoras) | `~/.local/share/estilos-backup/` |
| Copia automática del aspecto anterior | `~/.local/share/estilos/.anterior/` |
| Temas de color importables (`.ron`) | `~/Documentos/Estilos/Temas de color/` |
| Esquemas para la Terminal de COSMIC (`.ron`) | `~/Documentos/Estilos/Temas de terminal/` |
| Generadores y validador de los temas | `~/Documentos/Estilos/herramientas/` |
| Fondos de pantalla | `~/Imágenes/Fondos-estilos/` (`macos.png`, `windows.png`, `minimalista.png`, `hacker.png`, `cyberpunk.jpg`) |
| Fondos estáticos sacados de las animaciones (2560×1440) | `~/Documentos/Estilos/Fondos/` |
| Programa de fondos animados | `~/.local/bin/fondo-vivo` (código en `fondos-vivos/`, compilado en `bin/`) |
| Interruptor de animaciones | `~/.local/bin/estilo-animaciones` (código en `scripts/`) |
| Arranque automático de los fondos animados | `~/.config/autostart/fondo-vivo.desktop` |
| Ajustes de los fondos animados | `~/.config/estilos/fondos-vivos.conf` |
| Intro y prompt de la terminal | `~/.local/share/estilos/terminal/` (cargado desde `~/.bashrc`, bloque `estilos-terminal`) |
| Copia de referencia de las animaciones | `~/.local/share/estilos/fondos-vivos/originales/` |
| Accesos en el menú ("Estilo: …") | `~/.local/share/applications/estilo-*.desktop` (uno por estilo, más `estilo-guardar.desktop` y `estilo-animaciones.desktop`) |
| Apps ancladas al dock | `~/.config/cosmic/com.system76.CosmicAppList/v1/favorites` |

Este repositorio es una copia de `~/Documentos/Estilos/`. Además guarda, en
`sistema/`, una copia del comando `estilo` y de los accesos del menú
(`sistema/aplicaciones/`), por si hay que reinstalarlos.

## Cómo funciona el comando `estilo`

- `estilo lista`: muestra los estilos y marca con `*` el que está puesto.
- `estilo poner NOMBRE`:
  1. Guarda el aspecto actual en `.anterior`.
  2. Copia el estilo en `~/.config/cosmic/`, pero solo los archivos que cambian.
  3. Cada archivo se prepara primero fuera de la carpeta, en `~/.local/share/estilos/.escribiendo`, y se mueve de golpe con `mv`. Así COSMIC, que vigila esa carpeta, no lee archivos a medio escribir.

  4. Pone en la Terminal el esquema del estilo (ver más abajo).
  5. Si el `.ron` del estilo es nuevo o ha cambiado (su sha256 no coincide con `.ron-importado`), o el estilo tiene `.colores-pendientes`, lo importa solo con `cosmic-settings appearance import`, espera a que COSMIC calcule el tema y guarda el resultado en el estilo (como `estilo guardar`). Si falla, marca `.colores-pendientes`, avisa y abre Apariencia para hacerlo a mano.

  COSMIC aplica el cambio al momento, sin reiniciar nada. No se borran carpetas; solo se sobrescriben archivos.
- `estilo guardar [NOMBRE]`: guarda la configuración actual de COSMIC en el estilo indicado o, sin nombre, en el que está puesto. Conserva `.ron-importado`, así los retoques que haga el usuario en Apariencia no se pisan al volver a poner el estilo. Después comprueba si los colores del `.ron` ya se importaron: compara el acento del Builder con el del tema calculado. Si faltan, crea el archivo de marca `.colores-pendientes` dentro del estilo.
- `estilo borrar NOMBRE`: borra un estilo, excepto `original`.
- Los nombres de estilo solo admiten letras, números, `-` y `_`.
- Avisa al usuario con notificaciones de escritorio (gdbus → org.freedesktop.Notifications).

Partes de `~/.config/cosmic` que guarda cada estilo:

```
com.system76.CosmicTheme.Dark
com.system76.CosmicTheme.Dark.Builder
com.system76.CosmicTheme.Light
com.system76.CosmicTheme.Light.Builder
com.system76.CosmicTheme.Mode
com.system76.CosmicTk
com.system76.CosmicPanel
com.system76.CosmicPanel.Panel
com.system76.CosmicPanel.Dock
com.system76.CosmicBackground
com.system76.CosmicComp
```

## Los colores se importan solos

COSMIC calcula el tema final a partir del "Builder", y copiando archivos los
colores no se aplican bien. Antes había que importar cada `.ron` a mano en
Apariencia. Ahora `estilo poner` lo hace solo con
`cosmic-settings appearance import <archivo.ron>` (disponible en la versión
instalada; no abre ninguna ventana y también cambia a modo claro u oscuro).

- Solo importa cuando hace falta: la primera vez, o cuando el `.ron` cambia
  (se compara su sha256 con `.ron-importado` del estilo).
- Para forzar que se vuelva a importar un estilo: borrar su
  `.ron-importado` y ponerlo otra vez.
- Si cambias un `.ron`, basta con volver a poner ese estilo.

## Los temas de color (`.ron`)

| Tema | Modo | Acento | Fondo / tarjetas | Bordes (botones · ventanas) | Transparencias | Separación |
|---|---|---|---|---|---|---|
| macos | claro | `#007AFF` | `#F2F2F7` / `#FFFFFF` | cápsula · 16 px | media (barra, dock, menús) | 10 px |
| windows | oscuro | `#4CC2FF` | `#202020` / `#2B2B2B` | 4 px · 8 px | media (barra, dock, menús) | 4 px |
| minimalista | oscuro (Nord) | `#88C0D0` | `#2E3440` / `#3B4252` | cápsula · 12 px | no | 12 px |
| hacker | oscuro | verde Matrix `#00FF41` | `#050C06` / `#09160C` | rectos (0 px) | baja (barra, dock, menús) | 8 px |
| cyberpunk | oscuro | cian neón `#00F0FF` | `#0D0618` / `#190C28` | 2 px · rectas | media (barra, dock, menús) | 10 px |

Detalles de cada uno:

- **macos**: colores de énfasis de macOS (azul, índigo, morado, rosa, rojo,
  naranja, amarillo, verde, grafito). Éxito/aviso/peligro con las variantes de
  alto contraste de Apple (`#1E7E34`, `#C93400`, `#D70015`). Borde de ventana
  activa gris fino (`window_hint #B4B4BB`, 1 px) en vez de azul.
- **windows**: tokens de Windows 11 oscuro (Mica `#202020`, tarjetas
  `#2B2B2B`, éxito `#6CCB5F`, aviso `#FCE100`, peligro `#FF99A4`). Los
  colores de énfasis del selector son los de Windows en su tono claro.
- **minimalista**: los 16 colores de Nord. Texto tintado `#D8DEE9`. Peligro
  aclarado a `#D87780` porque el rojo Nord `#BF616A` casi no se leía
  (sigue en la paleta como `accent_red`).
- **hacker**: texto verde fósforo (`text_tint`), éxito `#00FF9C` (el acento
  antiguo), aviso ámbar `#FFB000`, peligro `#FF3355`. Interfaz compacta
  (la misma separación que la densidad "Compacta" de Apariencia).
- **cyberpunk**: violeta casi negro, texto lavanda (`text_tint`), borde de
  ventana activa magenta `#FF2BD6` (2 px), éxito `#39FF88`, aviso amarillo
  Cyberpunk 2077 `#FCEE0A`, peligro `#FF2A6D`. Contraste mínimo 5,2:1.

En los cuatro, la **paleta interna** (`palette`) está hecha a medida del tema
(antes era la de serie de COSMIC en todos). COSMIC la usa directamente en:

- `neutral_*`: texto de los campos de texto (`neutral_9`), interruptores,
  deslizadores, casillas y barras de desplazamiento.
- `accent_*`: los 9 colores que ofrece Apariencia → Color de énfasis.
- `accent_blue/green/yellow/red`, `ext_orange/purple`, `neutral_5`: los
  colores que COSMIC exporta a las apps GTK.

Qué controla cada tamaño de `corner_radii` (según el código de libcosmic):
`radius_xs` tarjetas · `radius_s` campos de texto y **esquinas de ventana**
(COSMIC le suma 4 px) · `radius_m` menús desplegables · `radius_l` menús del
panel y tooltips · `radius_xl` **botones**, interruptores y buscadores.

`gaps` es (exterior, interior); solo se usa el segundo.

## Esquemas de la Terminal (`Temas de terminal/`)

Uno por estilo, con el mismo nombre de archivo: Hacker Matrix, macOS Claro,
Nord, Windows Campbell (el de serie de Windows Terminal) y Cyberpunk Night
City. Se instalan y cambian solos:

- Al poner un estilo que tiene esquema, `estilo` añade a la Terminal los
  esquemas que le falten (en `color_schemes_dark`, o `color_schemes_light`
  para `macos.ron`), con números a partir de 1000 y sin tocar los que haya
  importado el usuario.
- Después escribe el nombre del esquema en `syntax_theme_dark` (o
  `syntax_theme_light` para macos). Los estilos sin esquema (como
  `original`) vuelven a `COSMIC Dark` / `COSMIC Light`.
- La Terminal abierta cambia de colores al momento.
- Si el usuario crea perfiles en la Terminal, cada perfil tiene su propio
  esquema y este automatismo no los cambia.

## Fondos animados (`fondo-vivo`)

Programa en Rust (`fondos-vivos/`) que dibuja con la tarjeta gráfica (EGL +
OpenGL ES 3) una escena animada por estilo. Se queda en marcha desde el
arranque automático y vigila `~/.local/share/estilos/.actual` cada 250 ms:

- Si el estilo tiene escena, la muestra; si no (`original`, `deshacer`, uno
  nuevo sin escena), quita sus superficies y se ve el fondo normal de COSMIC.
- Al cambiar de estilo hace una **transición** de 1,7 s entre las dos escenas
  (`shaders/transicion.frag`) y un **destello** de 1,1 s en la capa Overlay,
  por encima de las ventanas (`shaders/destello.frag`). El destello es
  transparente y tiene la región de entrada vacía: no tapa clics ni teclado.
- Escenas incluidas: `shaders/{hacker,cyberpunk,macos,windows,minimalista}.frag`
  (GLSL ES 3.00 con `mainImage`, como en Shadertoy; `comun.glsl` tiene ruido y
  azar). Si existe `~/.local/share/estilos/fondos-vivos/<estilo>.frag`, se usa
  esa en vez de la incluida: así se retoca una o se añade la de un estilo nuevo
  sin recompilar.

Por qué está en la capa **Background** y se "recoloca": cosmic-bg (el fondo
normal) también está en Background y, en cosmic-comp, dentro de una capa queda
encima la superficie creada más tarde (smithay `LayerMap` es un `IndexSet` y
cosmic-comp lo recorre con `.rev()`). cosmic-bg vuelve a crear sus superficies
cada vez que cambia su configuración. Por eso `fondo-vivo` vigila
`~/.config/cosmic/com.system76.CosmicBackground/v1/`, los PID de `cosmic-bg`
y la conexión de monitores, y entonces crea superficies nuevas (la vieja se
borra cuando la nueva ya tiene imagen, sin parpadeo). No se usa la capa Bottom
porque ahí están los iconos del escritorio de COSMIC Files.

Rendimiento: cada escena se dibuja a una fracción de la resolución (hacker
0,75; cyberpunk 0,6; windows y minimalista 0,5; macos 0,4) y se escala. Por
defecto 30 fps. Si el compositor deja de pedir fotogramas (fondo tapado), dibuja
como mucho uno por segundo.

Ajustes en `~/.config/estilos/fondos-vivos.conf` (se recargan solos):
`fps=5..60`, `calidad=baja|media|alta` (o un número), `destello=si|no`.

Otros usos del programa (sirven para probar sin escritorio):
`fondo-vivo captura ESTILO salida.png [ancho alto segundo]`,
`captura-transicion DE A PROGRESO salida.png`, `captura-destello ESTILO
PROGRESO salida.png`, `comprobar` (compila todas las escenas).

Compilar: `cd fondos-vivos && cargo build --release` y copiar
`target/release/fondo-vivo` a `bin/`. Compilarlo en Ubuntu 24.04 (o con una
glibc igual o más antigua) para que funcione en Pop!_OS 24.04: solo enlaza
libc; Wayland y EGL se cargan al arrancar.

## Terminal

`terminal/estilo-terminal.sh` se carga desde `~/.bashrc` y cambia el prompt
según `.actual` (se comprueba en cada prompt, así sigue al estilo sin abrir otra
terminal). Al abrir una terminal nueva lanza `terminal/intro.py ESTILO`, una
animación de ~1 s que se corta al pulsar una tecla (sin perderla). No se lanza
en una shell dentro de otra (bash, sudo, tmux…), ni con `ESTILO_INTRO=0`, ni si
existe `~/.config/estilos/sin-intro`. Solo usa caracteres que tiene la fuente
de COSMIC (latín, griego, cirílico, bloques).

## Instalador

`instalar.sh` (sin sudo, se puede repetir): copia temas y herramientas a
`~/Documentos/Estilos`, instala `fondo-vivo` y `estilo-animaciones` en
`~/.local/bin`, la terminal en `~/.local/share/estilos/terminal` y el bloque de
`~/.bashrc`, crea el estilo `cyberpunk` (si no existe) copiando `hacker` (o el
primero que encuentre, sin su `.ron-importado`), le cambia el fondo a
`cyberpunk.jpg` y le pone `.colores-pendientes`: la primera vez que se pone,
`estilo` importa `cyberpunk.ron` y su esquema de Terminal solo. Crea
`estilo-cyberpunk.desktop` y `estilo-animaciones.desktop` (copias en
`sistema/aplicaciones/`) y enciende las animaciones. No toca `~/.config/cosmic` ni `original`. `desinstalar.sh`
quita los fondos animados y la terminal y deja temas y estilos.

## Cómo cambiar o regenerar los temas

Los `.ron` se generan con scripts; no se editan a mano:

```
python3 herramientas/gen.py "Temas de color"
python3 herramientas/gen_term.py "Temas de terminal"
```

- `herramientas/gen.py`: un bloque por tema. Las escalas de grises
  (`neutral_0..10`) se calculan en Oklch con la misma luminosidad que la
  paleta de serie, así cambian de tono sin perder contraste.
- `herramientas/validar.rs`: carga los `.ron` con el mismo código que usa
  el botón Importar y muestra el contraste real del texto. Las instrucciones
  están al principio del archivo.
- El formato del `.ron` tiene que conservar exactamente estos campos (son los
  de la versión de COSMIC instalada). No añadir `frosted_maximized_apps` ni
  otros campos nuevos sin comprobar que la versión instalada los admite.
- Valores válidos de `frosted`: `ExtremelyLow`, `VeryLow`, `Low`, `Medium`,
  `High`, `VeryHigh`, `ExtremelyHigh` (y cada uno con `2` detrás).

## Si algo va mal

- Volver al aspecto inicial: `estilo poner original`
- Deshacer el último cambio de estilo:
  ```
  cp -a ~/.local/share/estilos/.anterior ~/.local/share/estilos/deshacer
  estilo poner deshacer
  ```
- Si la barra parpadea un segundo al cambiar de estilo, es normal: COSMIC la vuelve a lanzar sola.
- Si la barra desaparece o algo no se actualiza, cerrar sesión y volver a entrar.
- Si no hay barra para abrir la terminal, pulsar `Super + T`.

## Notas para una IA que toque esto

- El usuario habla español y no es técnico. Hay que explicarle los pasos de forma sencilla.
- Antes de cambiar nada en `~/.config/cosmic`, conviene guardar el aspecto actual con `estilo guardar <nombre>`.
- No borrar ni tocar `~/.local/share/estilos/original`.
- Si se añade un estilo nuevo, hacen falta:
  - su carpeta en `~/.local/share/estilos/`
  - su `.ron` en `Temas de color/` (añadir su bloque en `herramientas/gen.py`)
  - su fondo en `~/Imágenes/Fondos-estilos/`
  - su `.desktop` en `~/.local/share/applications/`
  - (opcional) su esquema en `Temas de terminal/` con el mismo nombre (añadir su bloque en `herramientas/gen_term.py`)
  - (opcional) su escena animada como `~/.local/share/estilos/fondos-vivos/<estilo>.frag`
    (o en `fondos-vivos/shaders/` + `ESCENAS`, `kind_of` y `base_scale` en
    `src/render.rs`), su intro en `terminal/intro.py` y su prompt en
    `terminal/estilo-terminal.sh`
- Para probar cambios en las escenas sin escritorio: `fondo-vivo captura`.
  Para probar el programa entero: sway sin pantalla
  (`WLR_BACKENDS=headless WLR_RENDERER=pixman sway`), `grim` para capturas y
  `LIBGL_ALWAYS_SOFTWARE=1`. Ojo: sway pone encima la capa más antigua, al
  revés que COSMIC.
