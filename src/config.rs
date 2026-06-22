use crate::{Error, Result, ffi};
use std::ffi::CString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rp1Backend {
    Rio,
    Pio,
}

#[derive(Debug, Clone)]
pub struct MatrixConfig {
    pub rows: u32,
    pub cols: u32,
    pub chain_length: u32,
    pub parallel: u32,
    pub brightness: u8,
    pub gpio_slowdown: u32,
    pub rp1_backend: Rp1Backend,
    pub hardware_mapping: Option<String>,
    pub rgb_sequence: Option<String>,
    pub pixel_mapper: Option<String>,
    pub panel_type: Option<String>,
    pub multiplexing: u32,
    pub row_address_type: u32,
    pub pwm_bits: u32,
    pub pwm_lsb_nanoseconds: u32,
    pub pwm_dither_bits: u32,
    pub scan_mode: u32,
    pub show_refresh_rate: bool,
    pub inverse_colors: bool,
    pub disable_hardware_pulsing: bool,
}
impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            rows: 32,
            cols: 64,
            chain_length: 1,
            parallel: 1,
            brightness: 30,
            gpio_slowdown: 1,
            rp1_backend: Rp1Backend::Rio,
            hardware_mapping: Some("regular".into()),
            rgb_sequence: None,
            pixel_mapper: None,
            panel_type: None,
            multiplexing: 0,
            row_address_type: 0,
            pwm_bits: 11,
            pwm_lsb_nanoseconds: 130,
            pwm_dither_bits: 0,
            scan_mode: 0,
            show_refresh_rate: false,
            inverse_colors: false,
            disable_hardware_pulsing: false,
        }
    }
}
pub(crate) struct NativeConfig {
    pub raw: ffi::RhmConfig,
    _strings: Vec<CString>,
}
impl MatrixConfig {
    /// Validates this configuration without initializing GPIO hardware.
    pub fn validate(&self) -> Result<()> {
        self.validate_inner()
    }
    pub(crate) fn validate_inner(&self) -> Result<()> {
        for (field, value) in [
            ("rows", self.rows),
            ("cols", self.cols),
            ("chain_length", self.chain_length),
            ("parallel", self.parallel),
        ] {
            if value == 0 {
                return Err(Error::InvalidConfig(format!(
                    "{field} must be greater than zero"
                )));
            }
        }
        if self.brightness > 100 {
            return Err(Error::InvalidBrightness(self.brightness));
        }
        Ok(())
    }
    pub(crate) fn to_native(&self) -> Result<NativeConfig> {
        self.validate_inner()?;
        fn int(value: u32, field: &'static str) -> Result<i32> {
            i32::try_from(value).map_err(|_| Error::IntegerConversion { field })
        }
        fn string(
            value: &Option<String>,
            field: &'static str,
            strings: &mut Vec<CString>,
        ) -> Result<*const std::ffi::c_char> {
            match value {
                None => Ok(std::ptr::null()),
                Some(value) => {
                    strings.push(
                        CString::new(value.as_str())
                            .map_err(|_| Error::InvalidCString { field })?,
                    );
                    Ok(strings.last().expect("just pushed").as_ptr())
                }
            }
        }
        let mut strings = Vec::with_capacity(4);
        let hardware_mapping = string(&self.hardware_mapping, "hardware_mapping", &mut strings)?;
        let led_rgb_sequence = string(&self.rgb_sequence, "rgb_sequence", &mut strings)?;
        let pixel_mapper_config = string(&self.pixel_mapper, "pixel_mapper", &mut strings)?;
        let panel_type = string(&self.panel_type, "panel_type", &mut strings)?;
        Ok(NativeConfig {
            raw: ffi::RhmConfig {
                rows: int(self.rows, "rows")?,
                cols: int(self.cols, "cols")?,
                chain_length: int(self.chain_length, "chain_length")?,
                parallel: int(self.parallel, "parallel")?,
                brightness: i32::from(self.brightness),
                gpio_slowdown: int(self.gpio_slowdown, "gpio_slowdown")?,
                rp1_pio: if matches!(self.rp1_backend, Rp1Backend::Pio) {
                    1
                } else {
                    0
                },
                hardware_mapping,
                led_rgb_sequence,
                pixel_mapper_config,
                panel_type,
                multiplexing: int(self.multiplexing, "multiplexing")?,
                row_address_type: int(self.row_address_type, "row_address_type")?,
                pwm_bits: int(self.pwm_bits, "pwm_bits")?,
                pwm_lsb_nanoseconds: int(self.pwm_lsb_nanoseconds, "pwm_lsb_nanoseconds")?,
                pwm_dither_bits: int(self.pwm_dither_bits, "pwm_dither_bits")?,
                scan_mode: int(self.scan_mode, "scan_mode")?,
                show_refresh_rate: if self.show_refresh_rate { 1 } else { 0 },
                inverse_colors: if self.inverse_colors { 1 } else { 0 },
                disable_hardware_pulsing: if self.disable_hardware_pulsing { 1 } else { 0 },
            },
            _strings: strings,
        })
    }
}
