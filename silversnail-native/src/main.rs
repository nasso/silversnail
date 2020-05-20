use wgpu::{
    Adapter, BackendBit, Device, DeviceDescriptor, PowerPreference, Queue, RequestAdapterOptions,
    ShaderModule,
};

async fn get_adapter() -> Adapter {
    println!("Enumerating available primary adapters...");
    println!(
        "{:#?}",
        Adapter::enumerate(BackendBit::PRIMARY)
            .iter()
            .map(Adapter::get_info)
            .collect::<Vec<_>>()
    );

    let adapter = Adapter::request(
        &RequestAdapterOptions {
            power_preference: PowerPreference::Default,
            compatible_surface: None,
        },
        BackendBit::PRIMARY,
    )
    .await
    .expect("Couldn't find a suitable Adapter.");

    println!("Using: {:#?}", adapter.get_info());
    adapter
}

async fn get_device(adapter: &Adapter) -> (Device, Queue) {
    let device_desc = DeviceDescriptor::default();
    adapter.request_device(&device_desc).await
}

async fn create_shaders(device: &Device) -> (ShaderModule, ShaderModule) {
    let vs = include_bytes!("shader.vert.spv");
    let fs = include_bytes!("shader.frag.spv");

    (
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap()),
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap()),
    )
}

async fn run() {
    let adapter = get_adapter().await;
    let (device, queue) = get_device(&adapter).await;
    let (vs_module, fs_module) = create_shaders(&device).await;
}

fn main() {
    futures::executor::block_on(run());
}
