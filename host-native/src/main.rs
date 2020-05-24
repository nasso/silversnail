use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use wasmtime::*;

static TEXTURE_SIZE: (u32, u32) = (512, 512);
type PluginImports = HashMap<&'static str, Extern>;

#[derive(Debug)]
struct Framebuffer {
    w: usize,
    h: usize,
    data: Vec<u8>,
}

impl Framebuffer {
    fn blank(w: usize, h: usize) -> Self {
        Framebuffer {
            w,
            h,
            data: vec![0u8; w * h * 4],
        }
    }
}

struct Plugin {
    name: String,
    process_fn: Box<dyn Fn(&mut Framebuffer) -> Result<(), Trap>>,
}

impl Plugin {
    fn load(
        store: &Store,
        imports: &PluginImports,
        path: impl AsRef<Path>,
    ) -> Result<Self, Box<dyn Error>> {
        let path = path.as_ref();
        let module = Module::from_file(&store, path)?;
        let imports = {
            let mut import_vec = Vec::new();

            for import in module.imports() {
                let name = import.name();

                import_vec.push(
                    imports
                        .get(name)
                        .expect(&format!("Plugin imported unknown symbol \"{}\"", name))
                        .clone(),
                );
            }

            import_vec
        };
        let instance = Instance::new(&module, &imports)?;
        let alloc_fb = instance
            .get_func("alloc_framebuffer")
            .expect("Plugin doesn't have a <alloc_framebuffer> function")
            .get2::<i32, i32, i32>()?;
        let free_fb = instance
            .get_func("free_framebuffer")
            .expect("Plugin doesn't have a <free_framebuffer> function")
            .get3::<i32, i32, i32, ()>()?;
        let process = instance
            .get_func("process")
            .expect("Plugin doesn't have a <process> function")
            .get4::<i32, i32, i32, i32, ()>()?;
        let mem = instance.get_memory("memory").expect("Couldn't get memory");

        Ok(Plugin {
            name: path.file_name().unwrap().to_str().unwrap().to_owned(),
            process_fn: Box::new(move |fb| {
                let fb_data_ptr = alloc_fb(fb.w as i32, fb.h as i32)?;

                unsafe {
                    let data = &mut mem.data_unchecked_mut()
                        [fb_data_ptr as usize..fb_data_ptr as usize + fb.w * fb.h * 4];

                    data.copy_from_slice(&fb.data[..]);
                }

                process(
                    fb.w as i32,
                    fb.h as i32,
                    fb_data_ptr,
                    (fb.w * fb.h * 4) as i32,
                )?;

                unsafe {
                    let data = &mem.data_unchecked_mut()
                        [fb_data_ptr as usize..fb_data_ptr as usize + fb.w * fb.h * 4];

                    fb.data.copy_from_slice(data);
                }

                free_fb(fb_data_ptr, fb.w as i32, fb.h as i32)?;
                Ok(())
            }),
        })
    }

    fn process(&self, fb: &mut Framebuffer) -> Result<(), Trap> {
        (self.process_fn)(fb)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let store = Store::default();
    let imports = PluginImports::new();

    let plugins: Result<Vec<_>, _> = fs::read_dir("./plugins")?
        .map(|path| Plugin::load(&store, &imports, path.unwrap().path()))
        .collect();
    let plugins = plugins?;

    let mut fb = Framebuffer::blank(TEXTURE_SIZE.0 as usize, TEXTURE_SIZE.1 as usize);

    for plugin in plugins.iter() {
        println!("Running plugin: {}", plugin.name);
        plugin.process(&mut fb)?
    }

    let mut encoder = png::Encoder::new(
        File::create("output.png").unwrap(),
        TEXTURE_SIZE.0,
        TEXTURE_SIZE.1,
    );
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&fb.data).unwrap();

    Ok(())
}
