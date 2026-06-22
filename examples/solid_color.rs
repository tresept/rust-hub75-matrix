use anyhow::Result;
use clap::{Parser, ValueEnum};
use rust_hub75_matrix::{Matrix, MatrixConfig, Rp1Backend};
use std::{
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
    #[arg(long, default_value_t = 255)]
    red: u8,
    #[arg(long, default_value_t = 0)]
    green: u8,
    #[arg(long, default_value_t = 0)]
    blue: u8,
    #[arg(long, default_value_t = 30)]
    brightness: u8,
    #[arg(long, default_value_t = 32)]
    rows: u32,
    #[arg(long, default_value_t = 64)]
    cols: u32,
    #[arg(long, default_value_t = 1)]
    chain_length: u32,
    #[arg(long,value_enum,default_value_t=Backend::Rio)]
    rp1_backend: Backend,
}
fn main() -> Result<()> {
    let a = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let stop = running.clone();
    ctrlc::set_handler(move || stop.store(false, Ordering::Relaxed))?;
    let mut m = Matrix::new(MatrixConfig {
        rows: a.rows,
        cols: a.cols,
        chain_length: a.chain_length,
        brightness: a.brightness,
        rp1_backend: a.rp1_backend.into(),
        ..Default::default()
    })?;
    m.fill(a.red, a.green, a.blue)?;
    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
    }
    m.clear()?;
    thread::sleep(Duration::from_millis(100));
    Ok(())
}
