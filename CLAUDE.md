# Contexto: sistema de "estilos" para el escritorio COSMIC (Pop!_OS / Linux)

Este documento explica a una IA (o a cualquiera) cómo está montado el sistema de
estilos de este escritorio. Para el usuario hay una ayuda más corta en
`LEEME - Si algo va mal.txt`, en esta misma carpeta, y los pasos para instalar
los temas en `README.md`.

## Qué es

Un sistema para cambiar el aspecto completo del escritorio COSMIC con un solo
comando. Cada estilo cambia a la vez los colores, la barra y el dock, el fondo de
pantalla, los bordes redondeados y la separación entre ventanas.

Estilos disponibles:

- `original`: copia de seguridad del aspecto inicial. Está protegida y no se puede borrar.
- `macos`
- `windows`
- `minimalista`
- `hacker`

El estilo puesto en cada momento está en `~/.local/share/estilos/.actual`.

## Piezas del sistema

| Qué | Dónde |
|---|---|
| El comando `estilo` (script bash) | `~/.local/bin/estilo` |
| Estilos guardados (una carpeta por estilo) | `~/.local/share/estilos/<nombre>/` |
| Estilo puesto ahora | `~/.local/share/estilos/.actual` |
| Copia automática del aspecto anterior | `~/.local/share/estilos/.anterior/` |
| Temas de color importables (`.ron`) | `~/Documentos/Estilos/Temas de color/` |
| Esquemas para la Terminal de COSMIC (`.ron`) | `~/Documentos/Estilos/Temas de terminal/` |
| Generadores y validador de los temas | `~/Documentos/Estilos/herramientas/` |
| Fondos de pantalla | `~/Imágenes/Fondos-estilos/` (`macos.png`, `windows.png`, `minimalista.png`, `hacker.png`) |
| Accesos en el menú ("Estilo: …") | `~/.local/share/applications/estilo-*.desktop` (uno por estilo, más `estilo-guardar.desktop`) |
| Apps ancladas al dock | `~/.config/cosmic/com.system76.CosmicAppList/v1/favorites` |

Este repositorio es una copia de `~/Documentos/Estilos/`.

## Cómo funciona el comando `estilo`

- `estilo lista`: muestra los estilos y marca con `*` el que está puesto.
- `estilo poner NOMBRE`:
  1. Guarda el aspecto actual en `.anterior`.
  2. Copia el estilo en `~/.config/cosmic/`, pero solo los archivos que cambian.
  3. Cada archivo se prepara primero fuera de la carpeta, en `~/.local/share/estilos/.escribiendo`, y se mueve de golpe con `mv`. Así COSMIC, que vigila esa carpeta, no lee archivos a medio escribir.

  COSMIC aplica el cambio al momento, sin reiniciar nada. No se borran carpetas; solo se sobrescriben archivos.
- `estilo guardar [NOMBRE]`: guarda la configuración actual de COSMIC en el estilo indicado o, sin nombre, en el que está puesto. Después comprueba si los colores del `.ron` ya se importaron: compara el acento del Builder con el del tema calculado. Si faltan, crea el archivo de marca `.colores-pendientes` dentro del estilo.
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

## La pega con los colores

COSMIC calcula el tema final a partir del "Builder", y copiando archivos los
colores no se aplican bien. Por eso, la primera vez que se pone cada estilo hay
que hacer esto a mano:

1. Apariencia → **Importar** → `Documentos/Estilos/Temas de color/<estilo>.ron`
2. Menú → **"Estilo: Guardar cambios"** (o `estilo guardar`)

Si el estilo tiene `.colores-pendientes`, al ponerlo el script avisa con una
notificación y abre Apariencia él solo (`cosmic-settings appearance`).

Cuando se cambia un `.ron`, hay que repetir estos dos pasos en ese estilo
aunque ya se hubieran hecho antes: su carpeta guarda los colores viejos y no
tiene `.colores-pendientes`, así que el script no avisa.

Las versiones recientes de COSMIC Settings traen
`cosmic-settings appearance import <archivo.ron>`, que hace lo mismo que el
botón Importar sin abrir la ventana (también cambia a modo claro u oscuro según
el tema). Si el sistema lo tiene, el script `estilo` podría llamarlo al poner un
estilo con `.colores-pendientes` y quitar el paso manual. Comprobarlo antes con
`cosmic-settings appearance --help`.

## Los temas de color (`.ron`)

| Tema | Modo | Acento | Fondo / tarjetas | Bordes (botones · ventanas) | Transparencias | Separación |
|---|---|---|---|---|---|---|
| macos | claro | `#007AFF` | `#F2F2F7` / `#FFFFFF` | cápsula · 16 px | media (barra, dock, menús) | 10 px |
| windows | oscuro | `#4CC2FF` | `#202020` / `#2B2B2B` | 4 px · 8 px | media (barra, dock, menús) | 4 px |
| minimalista | oscuro (Nord) | `#88C0D0` | `#2E3440` / `#3B4252` | cápsula · 12 px | no | 12 px |
| hacker | oscuro | verde Matrix `#00FF41` | `#050C06` / `#09160C` | rectos (0 px) | baja (barra, dock, menús) | 8 px |

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
Nord y Windows Campbell (el de serie de Windows Terminal). Se importan en la
Terminal de COSMIC: menú **Vista → Esquemas de color… → Importar** (el de macOS
en la lista de esquemas claros) y se eligen en **Vista → Ajustes… →
Apariencia**. La Terminal no forma parte de lo que guarda `estilo`; si se
quiere, habría que añadir `com.system76.CosmicTerm` a la lista de arriba.

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
