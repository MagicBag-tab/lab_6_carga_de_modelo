mod triangle;

use minifb::{Key, Window, WindowOptions};
use triangle::*;

fn main() {
    let mut ventana = Window::new(
        "Modelo 3D - Nave desde OBJ",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Cargar modelo desde archivo OBJ
    let mesh = Mesh::from_obj("assets/nave_1.obj")
        .expect("No se pudo cargar assets/nave_1.obj");

    println!("Modelo cargado: {} vértices, {} caras", mesh.vertices.len(), mesh.faces.len());

    // Calcular centro y radio para centrar y escalar el modelo
    let centro = mesh.calcular_centro();
    let radio_max = mesh.calcular_radio_max(centro);

    // Escala para que el modelo ocupe aproximadamente el 40% del ancho de ventana
    let escala = (WINDOW_WIDTH as f64 * 0.4) / radio_max;
    let offset_x = WINDOW_WIDTH as f64 / 2.0;
    let offset_y = WINDOW_HEIGHT as f64 / 2.0;

    // Distancia para alejar el modelo de la cámara (evitar división por cero en proyección)
    let distancia_z = 8.0;

    let mut angulo = 0.0f64;

    while ventana.is_open() && !ventana.is_key_down(Key::Escape) {
        angulo += 0.01;
        if angulo > std::f64::consts::PI * 2.0 {
            angulo = 0.0;
        }

        let mut buffer = vec![0u32; WINDOW_WIDTH * WINDOW_HEIGHT];
        for pixel in buffer.iter_mut() {
            *pixel = 0x1A1A1A;
        }

        let cos_a = angulo.cos();
        let sin_a = angulo.sin();

        for tri in mesh.get_triangles() {
            // Centrar vértices (mover origen al centro del modelo)
            let v1_centrado = tri.v1.sub(centro);
            let v2_centrado = tri.v2.sub(centro);
            let v3_centrado = tri.v3.sub(centro);

            // Rotar en Y alrededor del centro
            let rotar_y = |v: Vec3| -> Vec3 {
                Vec3::new(
                    v.x * cos_a + v.z * sin_a,
                    v.y,
                    -v.x * sin_a + v.z * cos_a,
                )
            };

            let v1_rot = rotar_y(v1_centrado);
            let v2_rot = rotar_y(v2_centrado);
            let v3_rot = rotar_y(v3_centrado);

            // Alejar en Z para que esté frente a la cámara (cam_z = 5.0 en proyectar)
            let v1_final = Vec3::new(v1_rot.x, v1_rot.y, v1_rot.z - distancia_z);
            let v2_final = Vec3::new(v2_rot.x, v2_rot.y, v2_rot.z - distancia_z);
            let v3_final = Vec3::new(v3_rot.x, v3_rot.y, v3_rot.z - distancia_z);

            let t_rotado = Triangle::new(v1_final, v2_final, v3_final, tri.color);
            dibujar_triangulo(&mut buffer, WINDOW_WIDTH, &t_rotado, escala, offset_x, offset_y);
        }

        ventana
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}