use super::{Context, DataType};
use glow::HasContext;
use std::{cell::RefCell, rc::Rc};

use log::trace;

pub type Handle = <glow::Context as HasContext>::VertexArray;

#[derive(Debug)]
pub struct VertexArray {
    gl: Rc<RefCell<glow::Context>>,
    obj: <glow::Context as HasContext>::VertexArray,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.borrow().delete_vertex_array(self.obj);
        }

        trace!("Deleted dropped VAO {:?}", self.obj);
    }
}

impl Context {
    pub fn bind_vertex_array(&mut self, p: Option<&VertexArray>) {
        let obj = p.map(|ref wrapper| wrapper.obj);

        if obj != self.state.bound_vertex_array {
            unsafe {
                self.gl.borrow().bind_vertex_array(obj);
            }

            self.state.bound_vertex_array = obj;
        }
    }

    pub fn enable_vertex_attrib_array(&self, i: u32) {
        unsafe {
            self.gl.borrow().enable_vertex_attrib_array(i);
        }
    }

    pub fn vertex_attrib_pointer(
        &self,
        i: u32,
        size: i32,
        data_type: DataType,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        unsafe {
            self.gl.borrow().vertex_attrib_pointer_f32(
                i,
                size,
                data_type.into(),
                normalized,
                stride,
                offset,
            );
        }
    }

    pub fn make_vertex_array(&mut self) -> VertexArray {
        let gl = self.gl.clone();
        let obj = unsafe { gl.borrow().create_vertex_array() }.expect("Couldn't create VAO");

        trace!("Created VAO {:?}", obj);

        VertexArray { gl, obj }
    }
}
