#![no_main]
#![no_std]

#[macro_use]
mod eadk;

/*
 * #[no_mangle]
pub static app_name: [u8;7] = *b"ZogZog\0";
*/

eadk_app_name!("HaHa");

#[no_mangle]
fn main() {
    let r = eadk::Rect { x: 0, y: 0, width: 100, height: 100 };
    let c = eadk::Color { rgb565: 0x301F };
    eadk::display::push_rect_uniform(r, c);
    loop {}
}
