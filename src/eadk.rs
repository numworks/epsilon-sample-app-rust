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
            eadk_backlight_set_brightness(brightness);
        }
    }
    pub fn brightness() -> u8 {
        unsafe {
            return eadk_backlight_brightness();
        }
    }

    extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }

}

pub mod display {
    use super::Rect;
    use super::Color;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
    }
}

pub fn random() -> u32 {
    unsafe {
        return eadk_random()
    }
}

extern "C" {
    fn eadk_random() -> u32;
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {} // FIXME: Do something better. Exit the app maybe?
}
