#include "bridge.h"
#include "led-matrix-c.h"
#include <chrono>
#include <cstring>
#include <new>
#include <thread>

struct RhmMatrix { struct RGBLedMatrix *matrix; struct LedCanvas *offscreen; int width; int height; };

static void write_error(char *out, size_t len, const char *message) {
  if (out != nullptr && len != 0) { std::strncpy(out, message, len - 1); out[len - 1] = '\0'; }
}
static int expected_bytes(int width, int height, size_t *out) {
  if (width <= 0 || height <= 0) return -2;
  const size_t w = static_cast<size_t>(width), h = static_cast<size_t>(height);
  if (w > SIZE_MAX / h || w * h > SIZE_MAX / 3) return -3;
  *out = w * h * 3; return 0;
}
static int swap(RhmMatrix *state) {
  struct LedCanvas *next = led_matrix_swap_on_vsync(state->matrix, state->offscreen);
  if (next == nullptr) return -4;
  state->offscreen = next; return 0;
}
extern "C" RhmMatrix *rhm_matrix_create(const RhmConfig *config, char *error, size_t error_len) {
  if (config == nullptr) { write_error(error, error_len, "config is null"); return nullptr; }
  try {
    struct RGBLedMatrixOptions options{}; struct RGBLedRuntimeOptions runtime{};
    options.rows=config->rows; options.cols=config->cols; options.chain_length=config->chain_length;
    options.parallel=config->parallel; options.brightness=config->brightness;
    options.multiplexing=config->multiplexing; options.row_address_type=config->row_address_type;
    options.pwm_bits=config->pwm_bits; options.pwm_lsb_nanoseconds=config->pwm_lsb_nanoseconds;
    options.pwm_dither_bits=config->pwm_dither_bits; options.scan_mode=config->scan_mode;
    options.show_refresh_rate=config->show_refresh_rate != 0; options.inverse_colors=config->inverse_colors != 0;
    options.disable_hardware_pulsing=config->disable_hardware_pulsing != 0;
    options.hardware_mapping=config->hardware_mapping; options.led_rgb_sequence=config->led_rgb_sequence;
    options.pixel_mapper_config=config->pixel_mapper_config; options.panel_type=config->panel_type;
    runtime.gpio_slowdown=config->gpio_slowdown; runtime.rp1_pio=config->rp1_pio;
    struct RGBLedMatrix *matrix=led_matrix_create_from_options_and_rt_options(&options, &runtime);
    if (!matrix) { write_error(error,error_len,"failed to create RGB LED matrix"); return nullptr; }
    struct LedCanvas *offscreen=led_matrix_create_offscreen_canvas(matrix);
    if (!offscreen) { led_matrix_delete(matrix); write_error(error,error_len,"failed to create offscreen canvas"); return nullptr; }
    int width=0,height=0; led_canvas_get_size(offscreen,&width,&height);
    RhmMatrix *result=new (std::nothrow) RhmMatrix{matrix,offscreen,width,height};
    if (!result) { led_matrix_delete(matrix); write_error(error,error_len,"failed to allocate bridge state"); }
    return result;
  } catch (...) { write_error(error,error_len,"C++ exception during matrix initialization"); return nullptr; }
}
extern "C" void rhm_matrix_destroy(RhmMatrix *state) {
  if (!state) return;

  // Leave the panel with a latched black frame before its refresh thread and
  // output-enable state are torn down. This is best-effort cleanup only.
  if (state->matrix && state->offscreen) {
    led_canvas_clear(state->offscreen);
    struct LedCanvas *next = led_matrix_swap_on_vsync(state->matrix, state->offscreen);
    if (next) state->offscreen = next;
    std::this_thread::sleep_for(std::chrono::milliseconds(50));
  }

  led_matrix_delete(state->matrix);
  delete state;
}
extern "C" int rhm_matrix_width(const RhmMatrix *state) { return state ? state->width : -1; }
extern "C" int rhm_matrix_height(const RhmMatrix *state) { return state ? state->height : -1; }
extern "C" int rhm_matrix_present_rgb_at(RhmMatrix *state,const uint8_t *pixels,size_t bytes,int width,int height,int x,int y) {
  if (!state || !pixels) return -1;
  size_t expected=0;
  int check=expected_bytes(width,height,&expected);
  if (check) return check;
  if (bytes != expected) return -5;
  set_image(state->offscreen,x,y,pixels,bytes,width,height,0);
  return swap(state);
}
extern "C" int rhm_matrix_present_rgb(RhmMatrix *state,const uint8_t *pixels,size_t bytes,int width,int height) {
  if (!state) return -1;
  if (width != state->width || height != state->height) return -2;
  return rhm_matrix_present_rgb_at(state,pixels,bytes,width,height,0,0);
}
extern "C" int rhm_matrix_clear(RhmMatrix *state) { if (!state) return -1; led_canvas_clear(state->offscreen); return swap(state); }
extern "C" int rhm_matrix_fill(RhmMatrix *state,uint8_t r,uint8_t g,uint8_t b) { if (!state) return -1; led_canvas_fill(state->offscreen,r,g,b); return swap(state); }
extern "C" int rhm_matrix_get_brightness(const RhmMatrix *state) { return state ? led_matrix_get_brightness(state->matrix) : -1; }
extern "C" int rhm_matrix_set_brightness(RhmMatrix *state,uint8_t value) { if (!state) return -1; led_matrix_set_brightness(state->matrix,value); return 0; }
