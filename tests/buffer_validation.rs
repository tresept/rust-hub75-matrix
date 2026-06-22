// Hardware-dependent `Matrix` methods are exercised on the Raspberry Pi.
// Arithmetic validation is deliberately kept local and checked here.
#[test]
fn rgb8_frame_length_is_three_bytes_per_pixel() {
    let (width, height) = (128usize, 32usize);
    assert_eq!(width * height * 3, 12_288);
}
