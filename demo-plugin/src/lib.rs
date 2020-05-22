#[no_mangle]
pub extern "C" fn version() -> u32 {
    1
}

/*
#[no_mangle]
pub extern "C" fn list_adapters() {
    println!("Enumerating available primary adapters...");
    println!(
        "{:#?}",
        wgpu::Instance::new()
            .enumerate_adapters(wgpu::BackendBit::PRIMARY)
            .map(|a: wgpu::Adapter| a.get_info())
            .collect::<Vec<_>>()
    );
}
*/
