use super::{GlVersion, Renderer};

use std::convert::{TryFrom, TryInto};
use std::sync::Once;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Reflect;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use cfg_if::cfg_if;

static INIT: Once = Once::new();

// enable logging!!! (maybe)
cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("couldn't initialize log :c");
        }
    } else {
        fn init_log() {}
    }
}

#[cfg(target_arch = "wasm32")]
fn init_once() {
    INIT.call_once(|| {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        init_log();
    });
}

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
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn js_constructor(options: js_sys::Object) -> Result<Renderer, JsValue> {
        init_once();

        // parse options
        let options: RendererOptions = options.try_into().unwrap();

        options.canvas.set_width(options.width);
        options.canvas.set_height(options.height);

        Ok(Renderer::new(
            glow::Context::from_webgl2_context(
                options
                    .canvas
                    .get_context("webgl2")?
                    .unwrap()
                    .dyn_into::<WebGl2RenderingContext>()?,
            ),
            GlVersion::Es300,
        ))
    }
}
