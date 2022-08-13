use core::f32::consts::PI;

#[repr(C)]
pub struct Color {
    pub rgb565: u16,
}

impl Color {
    #[must_use]
    pub const fn new(rgb565: u16) -> Self {
        Self { rgb565 }
    }

    #[must_use]
    pub const fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb565: ((r as u16 & 0b1111_1000) << 8)
                | ((g as u16 & 0b1111_1100) << 3)
                | (b as u16 >> 3),
        }
    }

    #[must_use]
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        let f = |n: f32| {
            let k: f32 = (n + hue / PI * 3.) % 6.;
            value * (1. - saturation * k.min(4. - k).min(1.).max(0.))
        };
        Color::from_rgb888(
            (f(5.) * 255.) as u8,
            (f(3.) * 255.) as u8,
            (f(1.) * 255.) as u8,
        )
    }
}

#[repr(C)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
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
    use super::Color;
    use super::Rect;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }

    extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
        fn eadk_display_push_rect(rect: Rect, color: *const Color);
        fn eadk_display_wait_for_vblank();
    }
}

pub mod timing {
    pub fn usleep(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    pub fn msleep(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    pub fn millis() -> u64 {
        unsafe {
            return eadk_timing_millis();
        }
    }

    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

pub fn random() -> u32 {
    unsafe { return eadk_random() }
}

extern "C" {
    fn eadk_random() -> u32;
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {} // FIXME: Do something better. Exit the app maybe?
}
