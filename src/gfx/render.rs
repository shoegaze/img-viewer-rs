use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType::TrianglesList;
use glium::{Display, Program, Surface, Texture2d, VertexBuffer, uniform};

use crate::gfx::vertex::SCREEN_VERTS;

static IMAGE_VERT_SHADER: &str = include_str!("../../resources/shaders/image_vert.glsl");
static IMAGE_FRAG_SHADER: &str = include_str!("../../resources/shaders/image_frag.glsl");

pub fn render(
    display: &Display<WindowSurface>,
    texture: &Texture2d,
) -> Result<(), Box<dyn std::error::Error>> {
    let program =
        Program::from_source(display, IMAGE_VERT_SHADER, IMAGE_FRAG_SHADER, None).unwrap();

    let vertex_buffer = VertexBuffer::new(display, &SCREEN_VERTS).unwrap();
    let index_buffer = glium::index::NoIndices(TrianglesList);

    {
        let uniforms = &uniform! {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0_f32],
            ],
            tex: texture,
        };

        let mut frame = display.draw();

        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            uniforms,
            &Default::default(),
        )?;

        frame.finish()?;

        Ok(())
    }
}
