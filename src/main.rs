mod triangle;

use minifb::{Key, Window, WindowOptions};
use triangle::*;

fn main() {
    let mut ventana = Window::new(
        "Modelo 3D - Cubo con Triángulos",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let cubo = crear_cubo();
    let escala = 100.0;
    let offset_x = WINDOW_WIDTH as f64 / 2.0;
    let offset_y = WINDOW_HEIGHT as f64 / 2.0;
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
        let rotar_y = |v: Vec3| -> Vec3 {
            Vec3::new(
                v.x * cos_a + v.z * sin_a,
                v.y,
                -v.x * sin_a + v.z * cos_a,
            )
        };

        for tri in cubo.get_triangles() {
            let t_rotado = Triangle::new(
                rotar_y(tri.v1),
                rotar_y(tri.v2),
                rotar_y(tri.v3),
                tri.color,
            );
            dibujar_triangulo(&mut buffer, WINDOW_WIDTH, &t_rotado, escala, offset_x, offset_y);
        }

        ventana
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}