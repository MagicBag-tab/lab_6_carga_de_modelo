# Lab 6 - Carga de Modelo OBJ

Renderizador de modelos 3D en OBJ escrito en Rust. Soporta dos modos de visualización:

- **Esqueleto**: dibuja únicamente las aristas de cada triángulo (alambrado).
- **Pintado**: rellena cada triángulo con un color celeste agrisado (`Light Sky Blue`).

## Controles

| Tecla | Acción |
|-------|--------|
| `ESPACIO` | Cambiar entre modo automático y manual de rotación |
| `T` | Cambiar entre modo Esqueleto y modo Pintado |
| `←` / `→` | Rotar en Y (modo manual) |
| `↑` / `↓` | Rotar en X (modo manual) |
| `ESC` | Salir |

## Cómo compilar y ejecutar

```bash
cargo build       # Compila en modo debug
cargo run         # Ejecuta el programa
cargo run --release  # Ejecuta en modo optimizado
```

## Repositorio

El código fuente completo está disponible en:

🔗 https://github.com/MagicBag-tab/lab_6_carga_de_modelo

## Modelo renderizado

A continuación se muestra el modelo (`assets/nave_1.obj`) renderizado en modo **Esqueleto**, centrado en pantalla (aproximadamente el 75% del ancho de la ventana):

![Modelo renderizado en modo Esqueleto](assets/Imagen_Esqueleto.png)

## Características implementadas

- Parser de archivos OBJ (vértices y caras)
- Proyección 3D a 2D con matriz de rotación en X y Y
- Rasterización de triángulos con edge functions (funciones de borde)
- Dos modos de visualización toggleables en tiempo real
- Título de ventana dinámico que muestra el modo actual (`[AUTO/MANUAL][ESQUELETO/PINTADO]`)

## Correcci?n de visibilidad

El relleno comparte un Z-buffer entre todas las caras y lo limpia cada fotograma.
Con la c?mara en `z = 5`, interpola baric?ntricamente `1 / (5 - z)` en pantalla;
el valor mayor corresponde a la superficie m?s cercana. Recorta contra una
distancia cercana de `0.1` antes de proyectar y limita el recorrido a la ventana.
Los tri?ngulos degenerados o no finitos se omiten.

El OBJ incluido tiene 730 v?rtices y 1512 caras, todas triangulares. Hay 756
copias geom?tricas exactas; el cargador conserva una por material, incluso si
la copia usa otros ?ndices u otro orden. El archivo original se conserva.
El MTL original estaba en el directorio superior: se incluye ahora en `assets/`
para resolver `mtllib nave_1.mtl`. Define un solo material gris (`Kd 0.8 0.8 0.8`),
por lo que este modelo no contiene colores distintos por pieza. `usemtl` se
conserva por cara y la iluminaci?n no depende del ?ndice del tri?ngulo.
La inspecci?n tambi?n detect? 128 caras de ?rea pr?cticamente nula en el archivo
original (incluidas sus copias), 32 aristas con seis incidencias y 32 aristas
de dos incidencias con orientaci?n inconsistente. No se repar? la topolog?a
del asset. Se mantienen ambas orientaciones visibles, sin backface culling.
El cargador requiere OBJ triangulados y devuelve un error ante otros pol?gonos,
en lugar de crear abanicos incorrectos sobre caras c?ncavas.

Validaci?n reproducible:

```bash
cargo fmt --check
cargo check
cargo test
cargo run --release
```

Las pruebas cubren profundidad en perspectiva, independencia del orden de dibujo,
ambos ?rdenes de v?rtices, recorte cercano, degenerados, caras fuera de pantalla,
buffers inv?lidos, materiales, ?ndices OBJ negativos y cuatro rotaciones de la
nave. Para comprobar visualmente: pulsa `T`, cambia a manual con `ESPACIO` y usa
las flechas para observar el frente, los lados y la parte posterior.
