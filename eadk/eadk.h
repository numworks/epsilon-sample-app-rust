// Backlight
uint8_t eadk_backlight_brightness();
void eadk_backlight_set_brightness(uint8_t brightness);

// Display
typedef uint16_t eadk_color;
typedef struct {
  uint16_t x;
  uint16_t y;
  uint16_t width;
  uint16_t height;
} eadk_rect;
void eadk_display_push_rect(eadk_rect rect, const eadk_color * pixels);
void eadk_display_push_rect_uniform(eadk_rect rect, eadk_color color);
void eadk_display_pull_rect(eadk_rect rect, eadk_color * pixels);
void eadk_display_wait_for_vblank();

// Misc
uint32_t eadk_random();
