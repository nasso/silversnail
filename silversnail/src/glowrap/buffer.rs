use super::Context;
use glow::HasContext;
use std::{cell::RefCell, rc::Rc};

use log::trace;

pub type Handle = <glow::Context as HasContext>::Buffer;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Target {
    ArrayBuffer,
    // CopyReadBuffer,
    // CopyWriteBuffer,
    // ElementArrayBuffer,
    // PixelPackBuffer,
    // PixelUnpackBuffer,
    // TransformFeedbackBuffer,
    // UniformBuffer,
}

impl Into<u32> for Target {
    fn into(self) -> u32 {
        match self {
            Target::ArrayBuffer => glow::ARRAY_BUFFER,
            // Target::CopyReadBuffer => glow::COPY_READ_BUFFER,
            // Target::CopyWriteBuffer => glow::COPY_WRITE_BUFFER,
            // Target::ElementArrayBuffer => glow::ELEMENT_ARRAY_BUFFER,
            // Target::PixelPackBuffer => glow::PIXEL_PACK_BUFFER,
            // Target::PixelUnpackBuffer => glow::PIXEL_UNPACK_BUFFER,
            // Target::TransformFeedbackBuffer => glow::TRANSFORM_FEEDBACK_BUFFER,
            // Target::UniformBuffer => glow::UNIFORM_BUFFER,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Usage {
    StaticDraw,
    // DynamicDraw,
    // StreamDraw,
    // StaticRead,
    // DynamicRead,
    // StreamRead,
    // StaticCopy,
    // DynamicCopy,
    // StreamCopy,
}

impl Into<u32> for Usage {
    fn into(self) -> u32 {
        match self {
            Usage::StaticDraw => glow::STATIC_DRAW,
            // Usage::DynamicDraw => glow::DYNAMIC_DRAW,
            // Usage::StreamDraw => glow::STREAM_DRAW,
            // Usage::StaticRead => glow::STATIC_READ,
            // Usage::DynamicRead => glow::DYNAMIC_READ,
            // Usage::StreamRead => glow::STREAM_READ,
            // Usage::StaticCopy => glow::STATIC_COPY,
            // Usage::DynamicCopy => glow::DYNAMIC_COPY,
            // Usage::StreamCopy => glow::STREAM_COPY,
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    gl: Rc<RefCell<glow::Context>>,
    obj: <glow::Context as HasContext>::Buffer,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.borrow().delete_buffer(self.obj);
        }

        trace!("Deleted dropped buffer {:?}", self.obj);
    }
}

impl Context {
    pub fn bind_buffer(&mut self, target: Target, p: Option<&Buffer>) {
        let obj = p.map(|ref wrapper| wrapper.obj);

        let bind_slot = match target {
            Target::ArrayBuffer => &mut self.state.bound_array_buffer,
        };

        if obj != *bind_slot {
            unsafe {
                self.gl.borrow().bind_buffer(target.into(), obj);
            }

            *bind_slot = obj;
        }
    }

    pub fn buffer_data(&self, target: Target, data: &[f32], usage: Usage) {
        unsafe {
            self.gl.borrow().buffer_data_u8_slice(
                target.into(),
                data.iter()
                    .copied()
                    .map(f32::to_bits)
                    .collect::<Vec<_>>()
                    .align_to::<u8>()
                    .1,
                usage.into(),
            );
        }

        trace!("Uploaded data to {:?} for {:?} usage", target, usage);
    }

    pub fn make_buffer(&self) -> Buffer {
        let gl = self.gl.clone();
        let obj = unsafe { gl.borrow().create_buffer() }.expect("Couldn't create buffer");

        trace!("Created buffer {:?}", obj);

        Buffer { gl, obj }
    }
}
