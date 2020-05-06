use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use wasmtime::*;

type PluginImports = HashMap<&'static str, Extern>;

struct Plugin {
    name: String,
    process_fn: Box<dyn Fn(f64, f64) -> Result<f64, Trap>>,
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
        let process = instance
            .get_func("process")
            .expect("Plugin doesn't have a <process> function")
            .get2::<f64, f64, f64>()?;

        Ok(Plugin {
            name: path.file_name().unwrap().to_str().unwrap().to_owned(),
            process_fn: Box::new(process),
        })
    }

    fn process(&self, x: f64, y: f64) -> Result<f64, Trap> {
        (self.process_fn)(x, y)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let store = Store::default();
    let mut imports = PluginImports::new();
    imports.insert("add", Func::wrap(&store, |x: f64, y: f64| x + y).into());
    imports.insert("sub", Func::wrap(&store, |x: f64, y: f64| x - y).into());
    imports.insert("mul", Func::wrap(&store, |x: f64, y: f64| x * y).into());
    imports.insert("div", Func::wrap(&store, |x: f64, y: f64| x / y).into());

    let plugins: Result<Vec<_>, _> = fs::read_dir("./plugins")?
        .map(|path| Plugin::load(&store, &imports, path.unwrap().path()))
        .collect();
    let plugins = plugins?;

    for plugin in plugins.iter() {
        println!(
            "Value given by {}: {}",
            plugin.name,
            plugin.process(16.0, 14.0)?
        );
    }
    Ok(())
}
