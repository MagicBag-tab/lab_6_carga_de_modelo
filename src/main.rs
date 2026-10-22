mod triangle;

use minifb::{Key, Window, WindowOptions};
use triangle::*;

#[derive(Clone, Copy, PartialEq)]
enum ModoGiro {
    Automatico,
    Manual,
}

#[derive(Clone, Copy, PartialEq)]
enum ModoVista {
    Esqueleto,
    Pintado,
}

fn color_para_cara(normal: Vec3, centro: Vec3) -> u32 {
    // Vector desde el centro de la cara hacia la cámara (en el origen)
    let vista = Vec3::new(0.0, 0.0, 0.0).sub(centro);
    // Producto punto: si es positivo, el normal apunta hacia la cámara (cara frontal)
    let dp = normal.dot(vista);
    if dp > 0.0 {
        0x87CEEB
    } else {
        0x5E8EA8 // Celeste más oscuro (cara trasera/hundida)
    }
}

fn main() {
    let mut ventana = Window::new(
        "Modelo 3D - Nave desde OBJ [AUTO][ESQUELETO]",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Cargar modelo desde archivo OBJ
    let mesh = Mesh::from_obj("assets/nave_1.obj")
        .expect("No se pudo cargar assets/nave_1.obj");

    println!("Modelo cargado: {} vértices, {} caras", mesh.vertices.len(), mesh.faces.len());
    println!("Controles:");
    println!("  ESPACIO  - Cambiar entre modo automático y manual");
    println!("  T        - Cambiar entre esqueleto y pintado");
    println!("  <- / ->  - Rotar en Y (modo manual)");
    println!("  ^ / v    - Rotar en X (modo manual)");
    println!("  ESC      - Salir");

    // Calcular centro y radio para centrar y escalar el modelo
    let centro = mesh.calcular_centro();
    let radio_max = mesh.calcular_radio_max(centro);

    // Escala para que el modelo ocupe aproximadamente el 75% del ancho de ventana
    let escala = (WINDOW_WIDTH as f64 * 0.75) / radio_max;
    let offset_x = WINDOW_WIDTH as f64 / 2.0;
    let offset_y = WINDOW_HEIGHT as f64 / 2.0;

    // Distancia para alejar el modelo de la cámara (evitar división por cero en proyección)
    let distancia_z = 8.0;

    let mut modo_giro = ModoGiro::Automatico;
    let mut modo_vista = ModoVista::Esqueleto;
    let mut angulo_y = 0.0f64;
    let mut angulo_x = 0.0f64;

    while ventana.is_open() && !ventana.is_key_down(Key::Escape) {
        // Alternar modo de giro con ESPACIO
        if ventana.is_key_down(Key::Space) {
            modo_giro = match modo_giro {
                ModoGiro::Automatico => ModoGiro::Manual,
                ModoGiro::Manual => ModoGiro::Automatico,
            };
            // Pequeña pausa para evitar cambio rápido al mantener presionada
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        // Alternar modo de vista con T
        if ventana.is_key_down(Key::T) {
            modo_vista = match modo_vista {
                ModoVista::Esqueleto => ModoVista::Pintado,
                ModoVista::Pintado => ModoVista::Esqueleto,
            };
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        // Actualizar ángulos según el modo de giro
        match modo_giro {
            ModoGiro::Automatico => {
                angulo_y += 0.01;
                if angulo_y > std::f64::consts::PI * 2.0 {
                    angulo_y = 0.0;
                }
            }
            ModoGiro::Manual => {
                let paso = 0.03;
                if ventana.is_key_down(Key::Left) {
                    angulo_y -= paso;
                }
                if ventana.is_key_down(Key::Right) {
                    angulo_y += paso;
                }
                if ventana.is_key_down(Key::Up) {
                    angulo_x -= paso;
                }
                if ventana.is_key_down(Key::Down) {
                    angulo_x += paso;
                }
                // Normalizar ángulos al rango [0, 2π)
                angulo_y = angulo_y.rem_euclid(std::f64::consts::PI * 2.0);
                angulo_x = angulo_x.rem_euclid(std::f64::consts::PI * 2.0);
            }
        }

        // Actualizar título de ventana según ambos modos
        let etiqueta_giro = match modo_giro {
            ModoGiro::Automatico => "AUTO",
            ModoGiro::Manual => "MANUAL",
        };
        let etiqueta_vista = match modo_vista {
            ModoVista::Esqueleto => "ESQUELETO",
            ModoVista::Pintado => "PINTADO",
        };
        ventana.set_title(&format!(
            "Modelo 3D - Nave desde OBJ [{}] [{}]",
            etiqueta_giro, etiqueta_vista
        ));

        let mut buffer = vec![0u32; WINDOW_WIDTH * WINDOW_HEIGHT];
        for pixel in buffer.iter_mut() {
            *pixel = 0x1A1A1A;
        }

        let cos_y = angulo_y.cos();
        let sin_y = angulo_y.sin();
        let cos_x = angulo_x.cos();
        let sin_x = angulo_x.sin();

        for (i, tri) in mesh.get_triangles().iter().enumerate() {
            // Centrar vértices (mover origen al centro del modelo)
            let v1_centrado = tri.v1.sub(centro);
            let v2_centrado = tri.v2.sub(centro);
            let v3_centrado = tri.v3.sub(centro);

            // Rotar en Y alrededor del centro
            let rotar_y = |v: Vec3| -> Vec3 {
                Vec3::new(
                    v.x * cos_y + v.z * sin_y,
                    v.y,
                    -v.x * sin_y + v.z * cos_y,
                )
            };

            let v1_y = rotar_y(v1_centrado);
            let v2_y = rotar_y(v2_centrado);
            let v3_y = rotar_y(v3_centrado);

            // Rotar en X alrededor del centro
            let rotar_x = |v: Vec3| -> Vec3 {
                Vec3::new(
                    v.x,
                    v.y * cos_x - v.z * sin_x,
                    v.y * sin_x + v.z * cos_x,
                )
            };

            let v1_rot = rotar_x(v1_y);
            let v2_rot = rotar_x(v2_y);
            let v3_rot = rotar_x(v3_y);

            // Alejar en Z para que esté frente a la cámara
            let v1_final = Vec3::new(v1_rot.x, v1_rot.y, v1_rot.z - distancia_z);
            let v2_final = Vec3::new(v2_rot.x, v2_rot.y, v2_rot.z - distancia_z);
            let v3_final = Vec3::new(v3_rot.x, v3_rot.y, v3_rot.z - distancia_z);

            // Asignar color según el modo de vista
            let color = if modo_vista == ModoVista::Pintado {
                color_por_indice(i)
            } else {
                tri.color
            };
            let t_rotado = Triangle::new(v1_final, v2_final, v3_final, color);

            match modo_vista {
                ModoVista::Esqueleto => {
                    dibujar_triangulo(&mut buffer, WINDOW_WIDTH, &t_rotado, escala, offset_x, offset_y);
                }
                ModoVista::Pintado => {
                    dibujar_triangulo_relleno(
                        &mut buffer,
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                        &t_rotado,
                        escala,
                        offset_x,
                        offset_y,
                    );
                }
            }
        }

        ventana
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}