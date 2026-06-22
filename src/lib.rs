//! Safe RGB8 frame output for HUB75 panels using `rpi-rgb-led-matrix`.
mod config;
mod error;
mod ffi;
mod matrix;
pub use config::{MatrixConfig, Rp1Backend};
pub use error::{Error, Result};
pub use matrix::Matrix;
