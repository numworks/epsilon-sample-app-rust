#![no_main]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;
mod epsilon_sdk;

#[no_mangle]
pub static app_name: [u8;7] = *b"ZogZog\0";

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub fn app_main() {
    //let b = epsilon_sdk::backlight::brightness();
    //epsilon_sdk::backlight::set_brightness(10);
    let r = epsilon_sdk::Rect { x: 0, y: 0, width: 100, height: 100 };
    let c = epsilon_sdk::Color { rgb565: 0x0 };
    epsilon_sdk::display::push_rect_uniform(r, c);
    //epsilon_sdk::backlight::set_brightness(b);
    loop {}
}
