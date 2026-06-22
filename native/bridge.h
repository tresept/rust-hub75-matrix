#ifndef RUST_HUB75_MATRIX_BRIDGE_H
#define RUST_HUB75_MATRIX_BRIDGE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RhmMatrix RhmMatrix;
typedef struct RhmConfig {
  int rows, cols, chain_length, parallel, brightness;
  int gpio_slowdown, rp1_pio;
  const char *hardware_mapping, *led_rgb_sequence, *pixel_mapper_config, *panel_type;
  int multiplexing, row_address_type;
  int pwm_bits, pwm_lsb_nanoseconds, pwm_dither_bits;
  int scan_mode, show_refresh_rate, inverse_colors, disable_hardware_pulsing;
} RhmConfig;

RhmMatrix *rhm_matrix_create(const RhmConfig *, char *error_buffer, size_t error_buffer_length);
void rhm_matrix_destroy(RhmMatrix *);
int rhm_matrix_width(const RhmMatrix *);
int rhm_matrix_height(const RhmMatrix *);
int rhm_matrix_present_rgb(RhmMatrix *, const uint8_t *, size_t, int, int);
int rhm_matrix_present_rgb_at(RhmMatrix *, const uint8_t *, size_t, int, int, int, int);
int rhm_matrix_clear(RhmMatrix *);
int rhm_matrix_fill(RhmMatrix *, uint8_t, uint8_t, uint8_t);
int rhm_matrix_get_brightness(const RhmMatrix *);
int rhm_matrix_set_brightness(RhmMatrix *, uint8_t);

#ifdef __cplusplus
}
#endif
#endif
