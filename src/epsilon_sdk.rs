/*
uint8_t epsilon_backlight_brightness();
void epsilon_backlight_set_brightness(uint8_t brightness);

void epsilon_display_push_rect(uint16_t x, uint16_t y, uint16_t width, uint16_t height, uint16_t * pixels);
void epsilon_display_push_rect_uniform(uint16_t x, uint16_t y, uint16_t width, uint16_t height, uint16_t color);
void epsilon_display_pull_rect(uint16_t x, uint16_t y, uint16_t width, uint16_t height, uint16_t * pixels);
void epsilon_display_wait_for_vblank();
*/

#[repr(C)]
pub struct Color {
    pub rgb565: u16
}

#[repr(C)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}

pub mod backlight {
    pub fn set_brightness(brightness: u8) {
        unsafe {
            epsilon_sdk_backlight_set_brightness(brightness);
        }
    }
    pub fn brightness() -> u8 {
        unsafe {
            return epsilon_sdk_backlight_brightness();
        }
    }

    extern "C" {
        fn epsilon_sdk_backlight_set_brightness(brightness: u8);
        fn epsilon_sdk_backlight_brightness() -> u8;
    }

}

pub mod display {
    use super::Rect;
    use super::Color;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            epsilon_sdk_display_push_rect_uniform(rect, color);
        }
    }

    extern "C" {
        fn epsilon_sdk_display_push_rect_uniform(rect: Rect, color: Color);
    }
}
