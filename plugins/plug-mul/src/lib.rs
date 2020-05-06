extern "C" {
    fn mul(x: f64, y: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn process(x: f64, y: f64) -> f64 {
    unsafe { mul(x, y) }
}
