mod glowrap;

pub use glow;
pub use glowrap::GlVersion;

use glowrap::buffer::{Buffer as Vbo, Target as BufferTarget, Usage as BufferUsage};
use glowrap::draw::{BufferBitMask, Mode as DrawMode};
use glowrap::program::{Defines, FragmentSource, Program, VertexSource};
use glowrap::vao::VertexArray as Vao;
use glowrap::{Context, DataType};

#[derive(Debug)]
pub struct Renderer {
    glw: Context,
    program: Program,
    quad_vbo: Vbo,
    full_quad_vao: Vao,
}

impl Renderer {
    pub fn new(gl: glow::Context, gl_version: GlVersion) -> Renderer {
        let mut glw = Context::new(gl, gl_version);

        let program = glw.make_program(
            VertexSource(include_str!("shaders/test.vs")),
            FragmentSource(include_str!("shaders/test.fs")),
            Defines(None),
        );

        let quad_vbo = {
            let vbo = glw.make_buffer();
            glw.bind_buffer(BufferTarget::ArrayBuffer, Some(&vbo));
            glw.buffer_data(
                BufferTarget::ArrayBuffer,
                &[-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0],
                BufferUsage::StaticDraw,
            );

            vbo
        };

        let full_quad_vao = {
            let vao = glw.make_vertex_array();
            glw.bind_vertex_array(Some(&vao));
            glw.enable_vertex_attrib_array(0);
            glw.vertex_attrib_pointer(0, 2, DataType::Float, false, 0, 0);

            vao
        };

        Renderer {
            glw,
            program,
            quad_vbo,
            full_quad_vao,
        }
    }

    pub fn render_frame(&mut self, _width: u32, _height: u32) {
        self.glw.use_program(Some(&self.program));

        self.glw.bind_vertex_array(Some(&self.full_quad_vao));
        self.glw.draw_arrays(DrawMode::TriangleStrip, 0, 4);
    }
}
