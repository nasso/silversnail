use super::{Context, GlVersion};
use glow::HasContext;
use std::{cell::RefCell, rc::Rc};

use log::trace;

pub type Handle = <glow::Context as HasContext>::Program;
pub type ShaderHandle = <glow::Context as HasContext>::Shader;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl Into<u32> for ShaderType {
    fn into(self) -> u32 {
        match self {
            ShaderType::Vertex => glow::VERTEX_SHADER,
            ShaderType::Fragment => glow::FRAGMENT_SHADER,
        }
    }
}

impl std::fmt::Display for ShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderType::Vertex => write!(f, "vertex"),
            ShaderType::Fragment => write!(f, "fragment"),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    gl: Rc<RefCell<glow::Context>>,
    obj: Handle,
}

impl Drop for Program {
    fn drop(&mut self) {
        {
            let gl = self.gl.borrow();

            unsafe {
                gl.delete_program(self.obj);
            }
        }

        trace!("Deleted dropped program {:?}", self.obj);
    }
}

fn create_shader(
    gl: &glow::Context,
    gl_version: GlVersion,
    defines: &Defines,
    shader_type: ShaderType,
    src: &str,
) -> ShaderHandle {
    // create shader
    let shader = unsafe { gl.create_shader(shader_type.into()) }.expect("Couldn't create shader");

    trace!("Created shader {:?}", shader);

    trace!("Loading {} shader source...", shader_type);

    // pre-process the source
    let src = format!(
        "#version {}\n{}\n{}",
        match gl_version {
            super::GlVersion::Es300 => "300 es",
            super::GlVersion::Core410 => "410",
        },
        match defines.0 {
            Some(ref defines) => defines
                .iter()
                .map(|def| format!("#define {} {}\n", def.0, def.1))
                .collect::<String>(),
            None => String::new(),
        },
        src
    );

    trace!("{}", src);

    // compile it!
    unsafe {
        gl.shader_source(shader, &src);
        gl.compile_shader(shader);
    }

    // in case it didn't compile...
    if !unsafe { gl.get_shader_compile_status(shader) } {
        // get the error message
        let log = unsafe { gl.get_shader_info_log(shader) };

        // panic!!!!
        panic!("Couldn't compile {} shader:\n\n{}", shader_type, log);
    }

    shader
}

// structures for named args
#[derive(Debug, PartialEq, Eq)]
pub struct Defines<'a, 'b, 'c>(pub Option<&'a [(&'b str, &'c str)]>);
#[derive(Debug, PartialEq, Eq)]
pub struct VertexSource<'a>(pub &'a str);
#[derive(Debug, PartialEq, Eq)]
pub struct FragmentSource<'a>(pub &'a str);

impl Context {
    pub fn use_program(&mut self, p: Option<&Program>) {
        let p = p.map(|ref prog| prog.obj);

        if p != self.state.program {
            unsafe {
                self.gl.borrow().use_program(p);
            }

            self.state.program = p;
        }
    }

    // pub fn get_attrib_location(&self, prog: &Program, name: &str) -> Option<u32> {
    //     let value = unsafe { self.gl.borrow().get_attrib_location(prog.obj, name) };

    //     if value != -1 {
    //         Some(value as u32)
    //     } else {
    //         None
    //     }
    // }

    pub fn make_program(&self, vs: VertexSource, fs: FragmentSource, defines: Defines) -> Program {
        let obj = {
            let gl = self.gl.borrow();

            // create program
            let program = unsafe { gl.create_program() }.expect("Couldn't create program");

            trace!("Created program {:?}", program);

            // vertex shader
            let vert_shader = create_shader(
                &self.gl.borrow(),
                self.gl_version,
                &defines,
                ShaderType::Vertex,
                vs.0,
            );

            // fragment shader
            let frag_shader = create_shader(
                &self.gl.borrow(),
                self.gl_version,
                &defines,
                ShaderType::Fragment,
                fs.0,
            );

            // attach the shaders
            unsafe {
                gl.attach_shader(program, vert_shader);
                gl.attach_shader(program, frag_shader);
            }

            // link the program!
            unsafe {
                gl.link_program(program);
            }

            // in case linking didn't go all
            if !unsafe { gl.get_program_link_status(program) } {
                // get the info!! tell us why!!!
                let log = unsafe { gl.get_program_info_log(program) };

                // share the info with the user because we're nice and we can't understand it anyway
                panic!("Couldn't link program:\n\n{}", log);
            }

            // delete the shaders cause we don't need them anymore
            for shader in &[vert_shader, frag_shader] {
                unsafe {
                    gl.detach_shader(program, *shader);
                    gl.delete_shader(*shader);
                }

                trace!("Deleted shader {:?}", *shader);
            }

            program
        };

        // return a wrapper on the program
        Program {
            gl: self.gl.clone(),
            obj,
        }
    }
}
