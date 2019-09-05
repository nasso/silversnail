use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GlVersion {
    Es300,
    Core410,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataType {
    Byte,
    Short,
    UnsignedByte,
    UnsignedShort,
    Float,
}

impl Into<u32> for DataType {
    fn into(self) -> u32 {
        match self {
            DataType::Byte => glow::BYTE,
            DataType::Short => glow::SHORT,
            DataType::UnsignedByte => glow::UNSIGNED_BYTE,
            DataType::UnsignedShort => glow::UNSIGNED_SHORT,
            DataType::Float => glow::FLOAT,
        }
    }
}

mod state;
use state::State;

#[derive(Debug)]
pub struct Context {
    state: State,
    gl_version: GlVersion,
    gl: Rc<RefCell<glow::Context>>,
}

impl Context {
    pub fn new(gl: glow::Context, gl_version: GlVersion) -> Self {
        Context {
            state: State::default(),
            gl_version,
            gl: Rc::new(RefCell::new(gl)),
        }
    }
}

pub mod buffer;
pub mod draw;
pub mod program;
pub mod vao;
