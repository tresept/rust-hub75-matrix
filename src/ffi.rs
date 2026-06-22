use std::ffi::{c_char, c_int};

#[repr(C)]
pub(crate) struct RhmMatrix {
    _private: [u8; 0],
}
#[repr(C)]
pub(crate) struct RhmConfig {
    pub rows: c_int,
    pub cols: c_int,
    pub chain_length: c_int,
    pub parallel: c_int,
    pub brightness: c_int,
    pub gpio_slowdown: c_int,
    pub rp1_pio: c_int,
    pub hardware_mapping: *const c_char,
    pub led_rgb_sequence: *const c_char,
    pub pixel_mapper_config: *const c_char,
    pub panel_type: *const c_char,
    pub multiplexing: c_int,
    pub row_address_type: c_int,
    pub pwm_bits: c_int,
    pub pwm_lsb_nanoseconds: c_int,
    pub pwm_dither_bits: c_int,
    pub scan_mode: c_int,
    pub show_refresh_rate: c_int,
    pub inverse_colors: c_int,
    pub disable_hardware_pulsing: c_int,
}
unsafe extern "C" {
    pub(crate) fn rhm_matrix_create(
        config: *const RhmConfig,
        error: *mut c_char,
        error_len: usize,
    ) -> *mut RhmMatrix;
    pub(crate) fn rhm_matrix_destroy(matrix: *mut RhmMatrix);
    pub(crate) fn rhm_matrix_width(matrix: *const RhmMatrix) -> c_int;
    pub(crate) fn rhm_matrix_height(matrix: *const RhmMatrix) -> c_int;
    pub(crate) fn rhm_matrix_present_rgb(
        matrix: *mut RhmMatrix,
        pixels: *const u8,
        bytes: usize,
        width: c_int,
        height: c_int,
    ) -> c_int;
    pub(crate) fn rhm_matrix_present_rgb_at(
        matrix: *mut RhmMatrix,
        pixels: *const u8,
        bytes: usize,
        width: c_int,
        height: c_int,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub(crate) fn rhm_matrix_clear(matrix: *mut RhmMatrix) -> c_int;
    pub(crate) fn rhm_matrix_fill(matrix: *mut RhmMatrix, red: u8, green: u8, blue: u8) -> c_int;
    pub(crate) fn rhm_matrix_get_brightness(matrix: *const RhmMatrix) -> c_int;
    pub(crate) fn rhm_matrix_set_brightness(matrix: *mut RhmMatrix, brightness: u8) -> c_int;
}
