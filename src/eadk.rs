#[repr(C)]
pub struct Color {
    pub rgb565: u16,
}

impl Color {
    pub fn new(rgb565: u16) -> Self {
        Self { rgb565 }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb565: ((r as u16 & 0b11111000) << 8)
                | ((g as u16 & 0b11111100) << 3)
                | (b as u16 >> 3),
        }
    }

    pub fn white() -> Self {
        Self::from_rgb(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::from_rgb(0, 0, 0)
    }

    pub fn red() -> Self {
        Self::from_rgb(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::from_rgb(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::from_rgb(0, 0, 255)
    }
}

#[repr(C)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height
        }
    }

    pub fn size(&self) -> u16 {
        self.width * self.height
    }

    pub fn up_corner(&self) -> Point {
        Point::new(self.x, self.y)
    }
    
    pub fn down_corner(&self) -> Point {
        Point::new(self.x + self.width, self.y + self.height)
    }
}

#[repr(C)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y
        }
    }
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
        fn eadk_display_pull_rect(rect: Rect, color: *const Color);
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
