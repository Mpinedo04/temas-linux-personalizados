# Estilos para el escritorio COSMIC

Cuatro temas de color para tu Linux (macOS, Windows, Minimalista y Hacker),
rehechos a fondo para que cada uno se parezca de verdad a lo que imita, y un
esquema de colores a juego para la Terminal.

## Qué ha cambiado

**En los cuatro temas**

- Antes solo cambiaban los colores principales. Por dentro seguían usando la
  paleta gris de serie de COSMIC, que es la que colorea los campos de texto,
  los interruptores, las barras de desplazamiento y las aplicaciones GTK. Ahora
  esa paleta está hecha a medida de cada tema.
- Los 9 colores que ofrece **Apariencia → Color de énfasis** son ahora los del
  propio tema (por ejemplo, en macOS salen los de un Mac).
- El fondo de las barras laterales y tarjetas está fijado a mano, con los
  colores exactos de cada sistema.
- Se ha comprobado que los cuatro se importan sin errores y que el texto se lee
  bien en todos (se midió el contraste con el mismo código que usa COSMIC).

**macOS**: tarjetas blancas sobre el gris claro de Apple, botones en forma de
cápsula, ventanas más redondeadas, borde gris fino en la ventana activa (como en
un Mac, en vez de un borde azul) y verde/naranja/rojo más legibles.

**Windows**: ahora los botones tienen las esquinas de 4 px de Windows 11 (antes
salían en forma de píldora) y las ventanas y menús 8 px. Usa los colores
oficiales de Windows 11 en modo oscuro.

**Minimalista**: la paleta Nord completa, también en los grises internos
(ahora azul pizarra). El rojo de "borrar" se leía muy mal y se ha aclarado.
Un poco más de separación entre ventanas.

**Hacker**: estilo Matrix. Verde Matrix `#00FF41` como color principal, texto
verde fósforo, avisos en ámbar de monitor antiguo, interfaz más compacta y la
barra, el dock y los menús de cristal oscuro semitransparente.

## Cómo usarlos

Ya está todo instalado. No tienes que importar nada a mano:

- Pon el estilo que quieras desde el menú (**"Estilo: …"**). La primera vez
  que pongas cada uno tarda un segundo más porque importa sus colores nuevos
  y los guarda solo.
- La **Terminal** cambia también de colores sola, a juego con el estilo.
  Con "Estilo: Original" vuelve a sus colores de serie.

## Si algo no te gusta

- **La transparencia de la barra y los menús** se quita en Apariencia, en la
  opción **"Efecto de vidrio esmerilado"**. Luego pulsa "Estilo: Guardar
  cambios".
- **Volver a como estaba todo al principio**: abre la Terminal y escribe
  `estilo poner original`.
- Tus archivos `.ron` antiguos están guardados en
  `~/.local/share/estilos-backup/ron-antiguos/`. Si importas uno de ellos en
  Apariencia y pulsas "Estilo: Guardar cambios", ese estilo vuelve a verse
  como antes.

## Para quien quiera tocar los temas

Los `.ron` se crean con los programas de la carpeta `herramientas/`. Las
explicaciones técnicas están en `CLAUDE.md`.
