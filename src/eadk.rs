// This is a simple macro named `say_hello`.
#[macro_export]
macro_rules! eadk_app_name {
    // `()` indicates that the macro takes no argument.
    ($app_name:expr) => {
        //pub static app_name: *const u8 = $app_name.as_ptr();//str = *$app_name;
        const foo: &[u8] = $app_name.as_bytes();
        #[no_mangle]
        pub static app_name: [u8;3] = [foo[0], foo[1], 0];
    };
}

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

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {} // FIXME: Do something better. Exit the app maybe?
}
