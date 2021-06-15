#![no_std]
#![no_main]

mod eadk;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static eadk_app_name: [u8; 6] = *b"Hello\0";

fn random_u16() -> u16 {
    return eadk::random() as u16;
}

fn random_coordinate() -> u16 {
    return (eadk::random() % 0xFF) as u16;
}

#[no_mangle]
fn eadk_main() {
    for _ in 0..100 {
        let c = eadk::Color { rgb565: random_u16() };
        let r = eadk::Rect { x: random_coordinate(), y: random_coordinate(), width: random_coordinate(), height: random_coordinate() };
        eadk::display::push_rect_uniform(r, c);
    }
    loop {}
}
