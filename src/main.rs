#![no_std]
#![no_main]

pub mod eadk;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static eadk_app_name: [u8; 10] = *b"HelloRust\0";

fn random_u16() -> u16 {
    return eadk::random() as u16;
}

fn random_coordinate() -> u16 {
    return (eadk::random() % 0xFF) as u16;
}

fn eadk_main() {
    for _ in 0..100 {
        let c = eadk::Color { rgb565: random_u16() };
        let r = eadk::Rect { x: random_coordinate(), y: random_coordinate(), width: random_coordinate(), height: random_coordinate() };
        eadk::display::push_rect_uniform(r, c);
    }
    loop {}
}

use core::{
    mem::zeroed,
    ptr::{read, write_volatile},
};

#[no_mangle]
fn eadk_start() {
    extern "C" {
        static mut _bss_section_start_ram: u32;
        static mut _bss_section_end_ram: u32;
        static mut _data_section_start_ram: u32;
        static mut _data_section_end_ram: u32;
        static _data_section_start_flash: u32;
    }

    // Initialize bss
    unsafe {
        let mut bss_curr: *mut u32 = &mut _bss_section_start_ram;
        let bss_end: *mut u32 = &mut _bss_section_end_ram;

        while bss_curr < bss_end {
            write_volatile(bss_curr, zeroed());
            bss_curr = bss_curr.offset(1);
        }
    }

    // Initialize data
    unsafe {
        let mut data_curr: *mut u32 = &mut _data_section_start_ram;
        let data_end: *mut u32 = &mut _data_section_end_ram;
        let mut data_flash: *const u32 = &_data_section_start_flash;

        while data_curr < data_end {
            write_volatile(data_curr, read(data_flash));
            data_curr = data_curr.offset(1);
            data_flash = data_flash.offset(1);
        }
    }

    // Call user's main function
    eadk_main()
}
