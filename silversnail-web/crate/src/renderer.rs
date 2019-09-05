use super::init_once;

use std::convert::{TryFrom, TryInto};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use js_sys::Reflect;

/// Options passed to the renderer constructor
struct RendererOptions {
    canvas: HtmlCanvasElement,
    width: u32,
    height: u32,
}

impl TryFrom<js_sys::Object> for RendererOptions {
    type Error = JsValue;

    fn try_from(value: js_sys::Object) -> Result<Self, Self::Error> {
        Ok(RendererOptions {
            canvas: Reflect::get(&value, &"canvas".into())?.dyn_into::<HtmlCanvasElement>()?,
            width: Reflect::get(&value, &"width".into())?
                .as_f64()
                .expect("The given value for \"width\" is not a number") as u32,
            height: Reflect::get(&value, &"height".into())?
                .as_f64()
                .expect("The given value for \"height\" is not a number")
                as u32,
        })
    }
}

#[wasm_bindgen]
pub struct Renderer {
    canvas: HtmlCanvasElement,
    renderer: silversnail::Renderer,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(options: js_sys::Object) -> Result<Renderer, JsValue> {
        init_once();

        // parse options
        let options: RendererOptions = options.try_into().unwrap();

        options.canvas.set_width(options.width);
        options.canvas.set_height(options.height);

        Ok(Renderer {
            renderer: silversnail::Renderer::new(
                silversnail::glow::Context::from_webgl2_context(
                    options
                        .canvas
                        .get_context("webgl2")?
                        .unwrap()
                        .dyn_into::<WebGl2RenderingContext>()?,
                ),
                silversnail::GlVersion::Es300,
            ),
            canvas: options.canvas,
        })
    }

    pub fn render(&mut self) {
        init_once();

        self.renderer
            .render_frame(self.canvas.width(), self.canvas.height());
    }
}
