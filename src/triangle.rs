pub const WINDOW_WIDTH: usize = 800;
pub const WINDOW_HEIGHT: usize = 600;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn sub(self, otro: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - otro.x,
            y: self.y - otro.y,
            z: self.z - otro.z,
        }
    }
    pub fn add(self, otro: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + otro.x,
            y: self.y + otro.y,
            z: self.z + otro.z,
        }
    }
    pub fn dot(self, otro: Vec3) -> f64 {
        self.x * otro.x + self.y * otro.y + self.z * otro.z
    }
    pub fn cross(self, otro: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * otro.z - self.z * otro.y,
            y: self.z * otro.x - self.x * otro.z,
            z: self.x * otro.y - self.y * otro.x,
        }
    }
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub color: u32,
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, color: u32) -> Self {
        Triangle { v1, v2, v3, color }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }
    pub fn add_vertex(&mut self, v: Vec3) -> usize {
        self.vertices.push(v);
        self.vertices.len() - 1
    }
    pub fn add_face(&mut self, i1: usize, i2: usize, i3: usize) {
        self.faces.push((i1, i2, i3));
    }
    pub fn get_triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        for &(i1, i2, i3) in &self.faces {
            let v1 = self.vertices[i1];
            let v2 = self.vertices[i2];
            let v3 = self.vertices[i3];
            let color = 0x00FF00;
            triangles.push(Triangle::new(v1, v2, v3, color));
        }
        triangles
    }
}

pub fn crear_cubo() -> Mesh {
    let mut mesh = Mesh::new();
    let v0 = mesh.add_vertex(Vec3::new(-1.0, -1.0, -1.0));
    let v1 = mesh.add_vertex(Vec3::new(1.0, -1.0, -1.0));
    let v2 = mesh.add_vertex(Vec3::new(1.0, 1.0, -1.0));
    let v3 = mesh.add_vertex(Vec3::new(-1.0, 1.0, -1.0));
    let v4 = mesh.add_vertex(Vec3::new(-1.0, -1.0, 1.0));
    let v5 = mesh.add_vertex(Vec3::new(1.0, -1.0, 1.0));
    let v6 = mesh.add_vertex(Vec3::new(1.0, 1.0, 1.0));
    let v7 = mesh.add_vertex(Vec3::new(-1.0, 1.0, 1.0));
    mesh.add_face(v0, v1, v2);
    mesh.add_face(v2, v3, v0);
    mesh.add_face(v4, v7, v6);
    mesh.add_face(v6, v5, v4);
    mesh.add_face(v0, v4, v7);
    mesh.add_face(v7, v3, v0);
    mesh.add_face(v1, v5, v6);
    mesh.add_face(v6, v2, v1);
    mesh.add_face(v0, v5, v1);
    mesh.add_face(v4, v0, v5);
    mesh.add_face(v3, v2, v6);
    mesh.add_face(v6, v7, v3);
    mesh
}

pub fn proyectar(v: Vec3, escala: f64, offset_x: f64, offset_y: f64) -> (i32, i32) {
    let cam_z = 5.0;
    let factor = cam_z / (cam_z - v.z);
    let x = (v.x * factor * escala + offset_x) as i32;
    let y = (-v.y * factor * escala + offset_y) as i32;
    (x, y)
}

pub fn dibujar_linea(buffer: &mut [u32], width: usize, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    loop {
        if x0 >= 0 && x0 < width as i32 && y0 >= 0 && y0 < WINDOW_HEIGHT as i32 {
            buffer[y0 as usize * width + x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn dibujar_triangulo(
    buffer: &mut [u32],
    width: usize,
    t: &Triangle,
    escala: f64,
    offset_x: f64,
    offset_y: f64,
) {
    let (x1, y1) = proyectar(t.v1, escala, offset_x, offset_y);
    let (x2, y2) = proyectar(t.v2, escala, offset_x, offset_y);
    let (x3, y3) = proyectar(t.v3, escala, offset_x, offset_y);
    dibujar_linea(buffer, width, x1, y1, x2, y2, t.color);
    dibujar_linea(buffer, width, x2, y2, x3, y3, t.color);
    dibujar_linea(buffer, width, x3, y3, x1, y1, t.color);
}