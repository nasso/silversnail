extern "C" {
    fn add(x: f64, y: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn process(x: f64, y: f64) -> f64 {
    unsafe { add(x, y) }
}
