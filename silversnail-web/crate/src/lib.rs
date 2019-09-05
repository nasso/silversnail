use std::sync::Once;

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

fn init_once() {
    INIT.call_once(|| {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        init_log();
    });
}

mod renderer;
pub use renderer::Renderer;
