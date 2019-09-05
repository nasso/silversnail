use educe::Educe;

use super::*;

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct State {
    // current value for clear_color
    pub clear_color: (f32, f32, f32, f32),

    // program currently being used
    pub program: Option<program::Handle>,

    // vbo bind points
    pub bound_array_buffer: Option<buffer::Handle>,
    // pub bound_copy_read_buffer: Option<buffer::Handle>,
    // pub bound_copy_write_buffer: Option<buffer::Handle>,
    // pub bound_element_array_buffer: Option<buffer::Handle>,
    // pub bound_pixel_pack_buffer: Option<buffer::Handle>,
    // pub bound_pixel_unpack_buffer: Option<buffer::Handle>,
    // pub bound_transform_feedback_buffer: Option<buffer::Handle>,
    // pub bound_uniform_buffer: Option<buffer::Handle>,

    // vao binding
    pub bound_vertex_array: Option<vao::Handle>,
}
