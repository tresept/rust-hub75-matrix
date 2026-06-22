use anyhow::Result;
use rust_hub75_matrix::{Matrix, MatrixConfig};
use std::{
    thread,
    time::{Duration, Instant},
};
fn main() -> Result<()> {
    let mut matrix = Matrix::new(MatrixConfig::default())?;
    let (w, h) = matrix.dimensions();
    let mut frame = vec![0; w * h * 3];
    let start = Instant::now();
    loop {
        frame.fill(0);
        let x = (start.elapsed().as_millis() / 25 % w as u128) as usize;
        for y in 0..h {
            let i = (y * w + x) * 3;
            frame[i] = 255;
            frame[i + 1] = 64;
            frame[i + 2] = 0;
        }
        matrix.present_rgb(&frame)?;
        thread::sleep(Duration::from_millis(16));
    }
}
