use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub static SCREEN_VERTS: [Vertex; 6] = [
    // Triangle 0
    Vertex {
        // bottom-left
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // bottom-right
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // top-right
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
    // Triangle 1
    Vertex {
        // top-right
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // top-left
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // bottom-left
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
];
