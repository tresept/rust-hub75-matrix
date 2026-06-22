use anyhow::{Result, bail};
use clap::{Parser, ValueEnum};
use image::{
    AnimationDecoder, DynamicImage, Frame, ImageFormat, ImageReader, RgbImage,
    codecs::gif::GifDecoder, imageops::FilterType,
};
use rust_hub75_matrix::{Matrix, MatrixConfig, Rp1Backend};
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, ValueEnum)]
enum Backend {
    Rio,
    Pio,
}

impl From<Backend> for Rp1Backend {
    fn from(value: Backend) -> Self {
        match value {
            Backend::Rio => Self::Rio,
            Backend::Pio => Self::Pio,
        }
    }
}

#[derive(Parser)]
struct Args {
    /// A PNG or animated GIF file.
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
    #[arg(long, value_enum, default_value_t = Backend::Rio)]
    rp1_backend: Backend,
    /// Resize images that do not match the matrix dimensions.
    #[arg(long)]
    resize: bool,
    /// Stop after this many seconds. GIFs otherwise repeat until Ctrl+C.
    #[arg(long)]
    duration: Option<u64>,
}

fn prepare_image(image: DynamicImage, width: u32, height: u32, resize: bool) -> Result<RgbImage> {
    let image = image.to_rgb8();
    if image.dimensions() == (width, height) {
        return Ok(image);
    }
    if !resize {
        bail!(
            "image is {}x{}, matrix is {}x{}; use --resize to resize",
            image.width(),
            image.height(),
            width,
            height
        );
    }
    Ok(image::imageops::resize(
        &image,
        width,
        height,
        FilterType::Nearest,
    ))
}

fn should_continue(running: &AtomicBool, deadline: Option<Instant>) -> bool {
    running.load(Ordering::SeqCst) && deadline.is_none_or(|deadline| Instant::now() < deadline)
}

fn wait_for_frame(delay: Duration, running: &AtomicBool, deadline: Option<Instant>) {
    let until = Instant::now() + delay;
    while should_continue(running, deadline) && Instant::now() < until {
        let remaining = until.saturating_duration_since(Instant::now());
        thread::sleep(remaining.min(Duration::from_millis(10)));
    }
}

fn gif_delay(frame: &Frame) -> Duration {
    let (numerator, denominator) = frame.delay().numer_denom_ms();
    let milliseconds = u64::from(numerator) / u64::from(denominator.max(1));
    Duration::from_millis(milliseconds.max(10))
}

fn play_gif(
    matrix: &mut Matrix,
    path: &PathBuf,
    width: u32,
    height: u32,
    resize: bool,
    running: &AtomicBool,
    deadline: Option<Instant>,
) -> Result<()> {
    let decoder = GifDecoder::new(BufReader::new(File::open(path)?))?;
    let frames = decoder.into_frames().collect_frames()?;
    if frames.is_empty() {
        bail!("GIF contains no frames");
    }

    let frames = frames
        .into_iter()
        .map(|frame| {
            let delay = gif_delay(&frame);
            let image = prepare_image(
                DynamicImage::ImageRgba8(frame.into_buffer()),
                width,
                height,
                resize,
            )?;
            Ok((image, delay))
        })
        .collect::<Result<Vec<_>>>()?;

    while should_continue(running, deadline) {
        for (image, delay) in &frames {
            if !should_continue(running, deadline) {
                return Ok(());
            }
            matrix.present_rgb(image.as_raw())?;
            wait_for_frame(*delay, running, deadline);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let signal_running = Arc::clone(&running);
    ctrlc::set_handler(move || signal_running.store(false, Ordering::SeqCst))?;

    let mut matrix = Matrix::new(MatrixConfig {
        rows: args.rows,
        cols: args.cols,
        chain_length: args.chain_length,
        parallel: args.parallel,
        brightness: args.brightness,
        rp1_backend: args.rp1_backend.into(),
        ..Default::default()
    })?;
    let (width, height) = matrix.dimensions();
    let (width, height) = (u32::try_from(width)?, u32::try_from(height)?);
    let deadline = args
        .duration
        .map(|seconds| Instant::now() + Duration::from_secs(seconds));

    let format = ImageReader::open(&args.image)?
        .with_guessed_format()?
        .format();
    if format == Some(ImageFormat::Gif) {
        play_gif(
            &mut matrix,
            &args.image,
            width,
            height,
            args.resize,
            &running,
            deadline,
        )?;
    } else {
        let image = prepare_image(
            ImageReader::open(&args.image)?.decode()?,
            width,
            height,
            args.resize,
        )?;
        matrix.present_rgb(image.as_raw())?;
        while should_continue(&running, deadline) {
            thread::sleep(Duration::from_millis(10));
        }
    }

    matrix.clear()?;
    thread::sleep(Duration::from_millis(100));
    Ok(())
}
