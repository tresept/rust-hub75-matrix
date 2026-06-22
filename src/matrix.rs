use crate::{Error, MatrixConfig, Result, ffi};
use std::{
    ffi::{CStr, c_char},
    marker::PhantomData,
    ptr::NonNull,
    rc::Rc,
};

pub struct Matrix {
    raw: NonNull<ffi::RhmMatrix>,
    width: usize,
    height: usize,
    _not_send_sync: PhantomData<Rc<()>>,
}
impl Matrix {
    pub fn new(config: MatrixConfig) -> Result<Self> {
        let native = config.to_native()?;
        let mut error = [0 as c_char; 256];
        let raw = unsafe { ffi::rhm_matrix_create(&native.raw, error.as_mut_ptr(), error.len()) };
        let raw = NonNull::new(raw).ok_or_else(|| {
            Error::Initialization(
                unsafe { CStr::from_ptr(error.as_ptr()) }
                    .to_string_lossy()
                    .into_owned(),
            )
        })?;
        let width = usize::try_from(unsafe { ffi::rhm_matrix_width(raw.as_ptr()) })
            .map_err(|_| Error::Native(-1))?;
        let height = usize::try_from(unsafe { ffi::rhm_matrix_height(raw.as_ptr()) })
            .map_err(|_| Error::Native(-1))?;
        if width == 0 || height == 0 {
            unsafe { ffi::rhm_matrix_destroy(raw.as_ptr()) };
            return Err(Error::Initialization(
                "native backend returned an invalid canvas size".into(),
            ));
        }
        Ok(Self {
            raw,
            width,
            height,
            _not_send_sync: PhantomData,
        })
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    fn expected_bytes(width: usize, height: usize) -> Result<usize> {
        width
            .checked_mul(height)
            .and_then(|n| n.checked_mul(3))
            .ok_or(Error::DimensionOverflow)
    }
    fn native_dimension(value: usize, field: &'static str) -> Result<i32> {
        i32::try_from(value).map_err(|_| Error::IntegerConversion { field })
    }
    fn native_result(code: i32) -> Result<()> {
        if code == 0 {
            Ok(())
        } else {
            Err(Error::Native(code))
        }
    }
    pub fn present_rgb(&mut self, pixels: &[u8]) -> Result<()> {
        let expected = Self::expected_bytes(self.width, self.height)?;
        if pixels.len() != expected {
            return Err(Error::BufferLengthMismatch {
                expected,
                actual: pixels.len(),
            });
        }
        Self::native_result(unsafe {
            ffi::rhm_matrix_present_rgb(
                self.raw.as_ptr(),
                pixels.as_ptr(),
                pixels.len(),
                Self::native_dimension(self.width, "width")?,
                Self::native_dimension(self.height, "height")?,
            )
        })
    }
    pub fn present_rgb_at(
        &mut self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        pixels: &[u8],
    ) -> Result<()> {
        let expected = Self::expected_bytes(width, height)?;
        if pixels.len() != expected {
            return Err(Error::BufferLengthMismatch {
                expected,
                actual: pixels.len(),
            });
        }
        Self::native_result(unsafe {
            ffi::rhm_matrix_present_rgb_at(
                self.raw.as_ptr(),
                pixels.as_ptr(),
                pixels.len(),
                Self::native_dimension(width, "width")?,
                Self::native_dimension(height, "height")?,
                x,
                y,
            )
        })
    }
    pub fn clear(&mut self) -> Result<()> {
        Self::native_result(unsafe { ffi::rhm_matrix_clear(self.raw.as_ptr()) })
    }
    pub fn fill(&mut self, red: u8, green: u8, blue: u8) -> Result<()> {
        Self::native_result(unsafe { ffi::rhm_matrix_fill(self.raw.as_ptr(), red, green, blue) })
    }
    pub fn brightness(&self) -> Result<u8> {
        let brightness = unsafe { ffi::rhm_matrix_get_brightness(self.raw.as_ptr()) };
        u8::try_from(brightness).map_err(|_| Error::Native(brightness))
    }
    pub fn set_brightness(&mut self, brightness: u8) -> Result<()> {
        if brightness > 100 {
            return Err(Error::InvalidBrightness(brightness));
        }
        Self::native_result(unsafe {
            ffi::rhm_matrix_set_brightness(self.raw.as_ptr(), brightness)
        })
    }
}
impl Drop for Matrix {
    fn drop(&mut self) {
        // Drop cannot return failures. Ask the bridge to latch a black frame
        // before releasing the hardware as a best-effort fallback.
        unsafe {
            let _ = ffi::rhm_matrix_clear(self.raw.as_ptr());
            ffi::rhm_matrix_destroy(self.raw.as_ptr());
        }
    }
}
