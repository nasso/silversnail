use plug_std::color::Color;
use plug_std::framebuffer::Framebuffer;

#[no_mangle]
pub extern "C" fn process(w: u32, h: u32, data: Box<[u8]>) {
    let mut fb = Framebuffer::new(w, h, data);

    fb.draw_rect(20, 40, 200, 100, Color::rgbf(0.0, 0.0, 1.0));
}
