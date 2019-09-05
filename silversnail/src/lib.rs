mod glowrap;
mod renderer;

#[cfg(target_arch = "wasm32")]
mod web;

pub use glow;
pub use glowrap::GlVersion;
pub use renderer::Renderer;

// Expose WASM interface
#[cfg(target_arch = "wasm32")]
pub use web::*;
