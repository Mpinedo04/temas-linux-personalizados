# Estilos para el escritorio COSMIC · versión 2

Cinco estilos completos para tu Linux: **macOS**, **Windows**, **Minimalista**,
**Hacker** y el nuevo **Cyberpunk**. Cada uno trae su tema de colores, un
**fondo de pantalla animado**, un **efecto al cambiar de estilo** y su propia
**terminal**, con colores, intro animada y línea de comandos a juego.

## Qué hay en cada estilo

| Estilo | Fondo animado | Al cambiar a este estilo | Terminal |
|---|---|---|---|
| **Cyberpunk** (nuevo) | Night City: sol synthwave, rascacielos con ventanas que parpadean, letreros de neón, coches voladores, lluvia y suelo de rejilla en movimiento, con algún «glitch» | El fondo se rompe en bloques con fallos de color y unas franjas de neón cruzan la pantalla | Letrero NIGHT CITY que aparece con glitch, tus datos de «netrunner» y una línea de comandos en magenta y cian |
| **Hacker** | Lluvia digital de Matrix en tres capas de profundidad, con efecto de monitor antiguo | La lluvia cae y va descubriendo el fondo nuevo; una línea verde barre la pantalla | Lluvia Matrix, «Despierta, …» y línea de comandos estilo Kali |
| **macOS** | Olas de color por capas que se mecen, como los fondos de macOS Big Sur | Fundido suave con un ligero zoom y un brillo blanco | «Último inicio de sesión…» y la línea de comandos de la Terminal de Mac |
| **Windows** | La flor «Bloom» de Windows 11: pétalos azules translúcidos que respiran | Barrido diagonal de luz azul | Saludo de PowerShell y línea `PS C:\Users\…>` |
| **Minimalista** | Aurora boreal sobre montañas nevadas, con los colores Nord | Un círculo que se abre desde el centro | Un saludo discreto según la hora |

Los fondos animados se dibujan con la tarjeta gráfica y están pensados para
gastar poco: se paran solos cuando una ventana tapa el fondo entero.

## Cómo instalarlo

1. Descarga esta carpeta: en GitHub, botón **Code → Download ZIP**, y
   descomprímela.
2. Abre la carpeta, pulsa con el botón derecho en un hueco vacío y elige
   **Abrir en la terminal** (o abre una terminal y entra en la carpeta).
3. Escribe esto y pulsa Intro:

   ```
   bash instalar.sh
   ```

   No pide contraseña ni cambia tus estilos actuales.
4. Pon el estilo nuevo desde el menú: **«Estilo: Cyberpunk»**. Usa la misma
   barra y el mismo dock que tu estilo hacker, con el fondo de Night City.

Abre una terminal nueva para ver la intro.

## Cómo usarlos

No tienes que importar nada a mano:

- Pon el estilo que quieras desde el menú (**«Estilo: …»**). La primera vez
  que pongas cada uno tarda un segundo más porque importa sus colores nuevos
  y los guarda solo.
- La **Terminal** cambia también de colores sola, a juego con el estilo.
  Con «Estilo: Original» vuelve a sus colores de serie.
- El **fondo animado** cambia con el estilo, con su transición. En
  «Estilo: Original» se ve tu fondo de siempre.

## Encender, apagar y ajustar las animaciones

- **Menú → «Estilo: Animaciones sí/no»** enciende o apaga los fondos
  animados. Apagados, vuelve el fondo normal de cada estilo.
- En una terminal:

  | Para… | Escribe |
  |---|---|
  | Gastar menos batería | `estilo-animaciones calidad baja` |
  | Más nitidez | `estilo-animaciones calidad alta` |
  | Quitar el efecto al cambiar de estilo | `estilo-animaciones destello no` |
  | Quitar la intro de la terminal | `estilo-animaciones intro no` |
  | Ver cómo está todo | `estilo-animaciones estado` |

## Si algo no te gusta

- **La transparencia de la barra y los menús** se quita en Apariencia, en la
  opción **«Efecto de vidrio esmerilado»**. Luego pulsa «Estilo: Guardar
  cambios».
- **Apagar las animaciones:** menú «Estilo: Animaciones sí/no», o
  `estilo-animaciones no`.
- **Quitar todo lo de la versión 2** (fondos animados e intro de la terminal):
  `bash desinstalar.sh`. Los temas y los estilos se quedan.
- **Volver a como estaba todo al principio**: abre la Terminal y escribe
  `estilo poner original`.
- Tus archivos `.ron` antiguos están guardados en
  `~/.local/share/estilos-backup/ron-antiguos/`. Si importas uno de ellos en
  Apariencia y pulsas «Estilo: Guardar cambios», ese estilo vuelve a verse
  como antes.

Hace falta el escritorio COSMIC (Pop!_OS 24.04 o parecido) y una tarjeta
gráfica con OpenGL ES 3, que tiene cualquier ordenador de los últimos diez años.

## Los colores de cada tema

- **En todos**: la paleta interna (campos de texto, interruptores, barras de
  desplazamiento y aplicaciones GTK) y los 9 colores de **Apariencia → Color
  de énfasis** están hechos a medida de cada tema, y se ha comprobado que el
  texto se lee bien en todos con el mismo código que usa COSMIC.
- **Cyberpunk**: violeta casi negro, acento cian neón, borde de la ventana
  activa en magenta, avisos en el amarillo de Cyberpunk 2077 y texto lavanda.
- **macOS**: tarjetas blancas sobre el gris claro de Apple, botones cápsula,
  ventanas redondeadas y borde gris fino en la ventana activa.
- **Windows**: botones con las esquinas de 4 px de Windows 11 y ventanas y
  menús de 8 px, con los colores oficiales del modo oscuro.
- **Minimalista**: la paleta Nord completa, también en los grises internos.
- **Hacker**: verde Matrix `#00FF41`, texto verde fósforo, avisos en ámbar e
  interfaz más compacta.

## Para quien quiera tocar los temas

Los temas se crean con los programas de `herramientas/` y los fondos animados
están en `fondos-vivos/`. En `sistema/` hay una copia del comando `estilo` y de
los accesos del menú. Las explicaciones técnicas están en `CLAUDE.md`.
