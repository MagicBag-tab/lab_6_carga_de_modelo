# Idea de coloreado para modelo 3D de Blender

Para colorear un modelo 3D exportado desde Blender dentro de un proyecto Rust, se pueden seguir estos pasos generales:

1. **Exportar el modelo**
   - En Blender, exporta el modelo en formato OBJ, glTF (.gltf) o FBX.
   - Asegúrate de que los vértices tengan atributos de color (o que vayas a asignarlos).

2. **Leer el modelo en Rust**
   - Usa una librería como `obj` o `gltf` (crates en Cargo) para parsear el archivo.
   - Extrae la posición (`[f32; 3]`) y, si existen, el color (`[f32; 3]` o `[f32; 4]`) por vértice.

3. **Configurar la pipeline de renderizado**
   - Con `wgpu` o `glium`, crea buffers de vértices que incluyan tanto posición como color.
   - Ejemplo de struct de vértice:
     ```rust
     #[repr(C)]
     #[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
     struct Vertex {
         position: [f32; 3],
         color: [f32; 3],
     }
     ```
   - Implementa el trait `VertexBufferLayout` para decirle a la GPU cómo interpretar los bytes.

4. **Crear shaders**
   - Escribe un shader WGSL o GLSL que reciba la posición y el color como entradas y los pase al fragmento.
   - En el fragmento, simplemente usa el color interpolado para pintar cada pixel.

5. **Dibujar**
   - Llena el buffer de vértices con los datos del modelo exportado.
   - Ejecuta el comando de drawcall correspondiente.

6. **Resultado**
   - El modelo aparecerá con los colores asignados, ya sea que vengan del archivo OBJ/glTF o que sean colores uniformes/por vértice que definas.

### Próximos pasos en este proyecto
- Agregar la dependencia `gltf` o `obj` a `Cargo.toml`.
- Implementar la lectura de un archivo `../models/blende_model.gltf`.
- Crear el shader WGSL y el pipeline de renderizado.
- Integrar el modelo en la ventana principal.

Empieza agregando la dependencia y la estructura de vértice; luego iteraremos sobre la carga del modelo y el shader.