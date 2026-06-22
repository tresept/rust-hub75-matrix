use anyhow::{Result, bail};
use clap::{Parser, ValueEnum};
use image::imageops::FilterType;
use rust_hub75_matrix::{Matrix, MatrixConfig, Rp1Backend};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
#[derive(Clone, Copy, ValueEnum)]
enum Backend {
    Rio,
    Pio,
}
impl From<Backend> for Rp1Backend {
    fn from(v: Backend) -> Self {
        match v {
            Backend::Rio => Self::Rio,
            Backend::Pio => Self::Pio,
        }
    }
}
#[derive(Parser)]
struct Args {
    image: PathBuf,
    #[arg(long, default_value_t = 32)]
    rows: u32,
    #[arg(long, default_value_t = 64)]
    cols: u32,
    #[arg(long, default_value_t = 2)]
    chain_length: u32,
    #[arg(long, default_value_t = 1)]
    parallel: u32,
    #[arg(long, default_value_t = 30)]
    brightness: u8,
    #[arg(long,value_enum,default_value_t=Backend::Rio)]
    rp1_backend: Backend,
    #[arg(long)]
    resize: bool,
    #[arg(long)]
    duration: Option<u64>,
}
fn main() -> Result<()> {
    let a = Args::parse();
    let mut m = Matrix::new(MatrixConfig {
        rows: a.rows,
        cols: a.cols,
        chain_length: a.chain_length,
        parallel: a.parallel,
        brightness: a.brightness,
        rp1_backend: a.rp1_backend.into(),
        ..Default::default()
    })?;
    let mut image = image::open(&a.image)?.to_rgb8();
    let (w, h) = m.dimensions();
    if image.dimensions() != (w as u32, h as u32) {
        if !a.resize {
            bail!(
                "PNG is {}x{}, matrix is {}x{}; use --resize to resize",
                image.width(),
                image.height(),
                w,
                h
            );
        }
        image = image::imageops::resize(&image, w as u32, h as u32, FilterType::Nearest);
    }
    m.present_rgb(image.as_raw())?;
    let running = Arc::new(AtomicBool::new(true));
    let stop = running.clone();
    ctrlc::set_handler(move || stop.store(false, Ordering::Relaxed))?;
    let deadline = a
        .duration
        .map(|s| std::time::Instant::now() + Duration::from_secs(s));
    while running.load(Ordering::Relaxed) && deadline.is_none_or(|d| std::time::Instant::now() < d)
    {
        thread::sleep(Duration::from_millis(100));
    }
    m.clear()?;
    Ok(())
}
