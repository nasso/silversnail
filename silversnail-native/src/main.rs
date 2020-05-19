use wgpu::{Adapter, BackendBit};

fn main() {
    println!("Enumerating adapters...");
    for adapter in Adapter::enumerate(BackendBit::PRIMARY).iter() {
        let info = adapter.get_info();

        println!("Found adapter:");
        println!("Name: {}", info.name);
        println!("Vendor: {}", info.vendor);
        println!("Device: {}", info.device);
        println!("Type: {:?}", info.device_type);
        println!("Backend: {:?}", info.backend);
        println!();
    }
}
