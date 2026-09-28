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

## Cómo instalarlos

**1. Copia los archivos nuevos**

Copia la carpeta `Temas de color` de aquí dentro de `Documentos/Estilos/`,
reemplazando los archivos que ya tienes. Copia también la carpeta
`Temas de terminal`.

**2. Vuelve a importar cada estilo, una sola vez**

Aunque ya lo hicieras antes, hay que repetirlo porque los colores han cambiado.
Para cada estilo (macOS, Windows, Minimalista y Hacker):

1. Ponlo desde el menú: **"Estilo: …"**.
2. Abre **Configuración → Escritorio → Apariencia** y pulsa **Importar**.
3. Elige `Documentos/Estilos/Temas de color/<estilo>.ron`.
4. En el menú, pulsa **"Estilo: Guardar cambios"**.

Así queda guardado y la próxima vez que pongas ese estilo saldrá ya bien.

**3. (Opcional) Colores a juego en la Terminal**

1. Abre la Terminal y ve al menú **Vista → Esquemas de color… → Importar**.
2. Elige los archivos de `Documentos/Estilos/Temas de terminal/`. El de macOS
   va en la lista de esquemas **claros**; los otros tres, en la de **oscuros**.
3. Ve a **Vista → Ajustes… → Apariencia** y elige el esquema que quieras.

## Si algo no te gusta

- **La transparencia de la barra y los menús** se quita en Apariencia, en la
  opción **"Efecto de vidrio esmerilado"**. Luego pulsa "Estilo: Guardar
  cambios".
- **Volver a como estaba todo al principio**: abre la Terminal y escribe
  `estilo poner original`.
- Tus archivos `.ron` antiguos siguen sirviendo: si importas uno de ellos, ese
  estilo vuelve a verse como antes.

## Para quien quiera tocar los temas

Los `.ron` se crean con los programas de la carpeta `herramientas/`. Las
explicaciones técnicas están en `CLAUDE.md`.
