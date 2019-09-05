use super::Context;
use glow::HasContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    TriangleStrip,
    TriangleFan,
    Triangles,
}

impl Into<u32> for Mode {
    fn into(self) -> u32 {
        match self {
            Mode::Points => glow::POINTS,
            Mode::LineStrip => glow::LINE_STRIP,
            Mode::LineLoop => glow::LINE_LOOP,
            Mode::Lines => glow::LINES,
            Mode::TriangleStrip => glow::TRIANGLE_STRIP,
            Mode::TriangleFan => glow::TRIANGLE_FAN,
            Mode::Triangles => glow::TRIANGLES,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BufferBitMask {
    pub color: bool,
    pub depth: bool,
    pub stencil: bool,
}

impl Into<u32> for BufferBitMask {
    fn into(self) -> u32 {
        let mut value = 0;

        if self.color {
            value |= glow::COLOR_BUFFER_BIT;
        }

        if self.depth {
            value |= glow::DEPTH_BUFFER_BIT;
        }

        if self.stencil {
            value |= glow::STENCIL_BUFFER_BIT;
        }

        value
    }
}

impl Context {
    pub fn clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        if self.state.clear_color != (r, g, b, a) {
            unsafe {
                self.gl.borrow().clear_color(r, g, b, a);
            }

            self.state.clear_color = (r, g, b, a);
        }
    }

    pub fn clear(&self, mask: BufferBitMask) {
        unsafe {
            self.gl.borrow().clear(mask.into());
        }
    }

    pub fn draw_arrays(&self, mode: Mode, first: i32, count: i32) {
        unsafe {
            self.gl.borrow().draw_arrays(mode.into(), first, count);
        }
    }
}
