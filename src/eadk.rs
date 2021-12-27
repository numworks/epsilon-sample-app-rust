#[repr(C)]
#[derive(Clone, Copy)]
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
            height,
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
        Self { x, y }
    }
}

pub enum Key {
    Left,
    Up,
    Down,
    Right,
    OK,
    Back,
    Home,
    Shift,
    Alpha,
    XNT,
    Var,
    Toolbox,
    Backspace,
    Exp,
    Ln,
    Log,
    Imaginary,
    Comma,
    Power,
    Sine,
    Cosine,
    Tangent,
    Pi,
    Sqrt,
    Square,
    Seven,
    Eight,
    Nine,
    LeftParenthesis,
    RightParenthesis,
    Four,
    Five,
    Six,
    Multiplication,
    Division,
    One,
    Two,
    Three,
    Plus,
    Minus,
    Zero,
    Dot,
    EE,
    Ans,
    EXE,
}

impl Key {
    pub fn into_u64(self) -> u64 {
        match self {
            Self::Left => 0,
            Self::Up => 1,
            Self::Down => 2,
            Self::Right => 3,
            Self::OK => 4,
            Self::Back => 5,
            Self::Home => 6,
            Self::Shift => 12,
            Self::Alpha => 13,
            Self::XNT => 14,
            Self::Var => 15,
            Self::Toolbox => 16,
            Self::Backspace => 17,
            Self::Exp => 18,
            Self::Ln => 19,
            Self::Log => 20,
            Self::Imaginary => 21,
            Self::Comma => 22,
            Self::Power => 23,
            Self::Sine => 24,
            Self::Cosine => 25,
            Self::Tangent => 26,
            Self::Pi => 27,
            Self::Sqrt => 28,
            Self::Square => 29,
            Self::Seven => 30,
            Self::Eight => 31,
            Self::Nine => 32,
            Self::LeftParenthesis => 33,
            Self::RightParenthesis => 34,
            Self::Four => 36,
            Self::Five => 37,
            Self::Six => 38,
            Self::Multiplication => 39,
            Self::Division => 40,
            Self::One => 42,
            Self::Two => 43,
            Self::Three => 44,
            Self::Plus => 45,
            Self::Minus => 46,
            Self::Zero => 48,
            Self::Dot => 49,
            Self::EE => 50,
            Self::Ans => 51,
            Self::EXE => 52,
        }
    }
}

#[repr(C)]
pub struct State {
    keyboard: u64,
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
    use super::Point;
    use super::Rect;

    pub fn push_rect(rect: Rect, pixels: &[Color]) -> () {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) -> () {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    pub fn pull_rect(rect: Rect, pixels: &mut [Color]) -> () {
        unsafe {
            eadk_display_pull_rect(rect, pixels.as_mut_ptr());
        }
    }

    pub fn push_point(point: Point, color: Color) -> () {
        push_rect_uniform(Rect::new(point.x, point.y, 1, 1), color);
    }

    pub fn pull_point(point: Point) -> Color {
        let mut color = [Color::new(0)];
        pull_rect(Rect::new(point.x, point.y, 1, 1), &mut color);
        color[0]
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

pub mod keyboard {
    use super::State;

    pub fn scan() -> State {
        unsafe { eadk_keyboard_scan() }
    }

    extern "C" {
        fn eadk_keyboard_scan() -> State;
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
