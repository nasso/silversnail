use crate::color::Color;

#[derive(Debug)]
pub struct Framebuffer {
    pub w: usize,
    pub h: usize,
    pub data: Box<[u8]>,
}

impl Framebuffer {
    pub fn new(w: u32, h: u32, data: Box<[u8]>) -> Self {
        Framebuffer {
            w: w as usize,
            h: h as usize,
            data,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.data[(y * self.w + x) * 4 + 0] = color.r;
        self.data[(y * self.w + x) * 4 + 1] = color.g;
        self.data[(y * self.w + x) * 4 + 2] = color.b;
        self.data[(y * self.w + x) * 4 + 3] = color.a;
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        for x in x..w {
            for y in y..h {
                self.set_pixel(x, y, color);
            }
        }
    }
}
