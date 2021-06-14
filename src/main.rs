#![no_std]
#![no_main]

mod eadk;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static eadk_app_name: [u8; 6] = *b"Hello\0";

#[no_mangle]
fn eadk_main() {
    let r = eadk::Rect { x: 0, y: 0, width: 100, height: 100 };
    let c = eadk::Color { rgb565: 0x301F };
    eadk::display::push_rect_uniform(r, c);
    loop {}
}
