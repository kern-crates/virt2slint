//! Convert virtio_input_event to WindowEvent(slint)
//!
//! # Example
//! ```no_run
//! use virt2slint::Converter;
//! let converter = Converter::new(32767,1200,800);
//! let mut x = 0;
//! let mut y = 0;
//! let event = converter.convert(0x0,&mut x,&mut y).unwrap();
//! ```
//!
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{LogicalPosition, SharedString};
use virtio_input_decoder::{DecodeType, Decoder, Key, KeyType, Mouse};

/// The converter of virtio_input_event to WindowEvent
/// # Example
/// ```no_run
/// use virt2slint::Converter;
/// let converter = Converter::new(32767,1200,800);
/// let mut x = 0;
/// let mut y = 0;
/// let event = converter.convert(0x0,&mut x,&mut y).unwrap();
/// ```
///
pub struct Converter {
    x_res: isize,
    y_res: isize,
    virtual_range: isize,
}

macro_rules! press {
    ($x:expr,$y:expr,$button:expr) => {
        WindowEvent::PointerPressed {
            position: LogicalPosition::new($x, $y),
            button: $button,
        }
    };
}

macro_rules! release {
    ($x:expr,$y:expr,$button:expr) => {
        WindowEvent::PointerReleased {
            position: LogicalPosition::new($x, $y),
            button: $button,
        }
    };
}

macro_rules! moved {
    ($x:expr,$y:expr) => {
        WindowEvent::PointerMoved {
            position: LogicalPosition::new($x, $y),
        }
    };
}

macro_rules! scrolled {
    ($x:expr,$y:expr,$dx:expr,$dy:expr) => {
        WindowEvent::PointerScrolled {
            position: LogicalPosition::new($x, $y),
            delta_x: $dx,
            delta_y: $dy,
        }
    };
}

impl Converter {
    /// Create a new Converter
    pub fn new(virtual_range: isize, x_res: isize, y_res: isize) -> Self {
        Self {
            x_res,
            y_res,
            virtual_range,
        }
    }
    fn scale(&self, x: isize, y: isize) -> (f32, f32) {
        let x = x as f32 * self.x_res as f32 / self.virtual_range as f32;
        let y = y as f32 * self.y_res as f32 / self.virtual_range as f32;
        (x, y)
    }
    /// Convert virtio_input_event to WindowEvent
    pub fn convert(&self, event: u64, cx: &mut isize, cy: &mut isize) -> Option<WindowEvent> {
        let decoder = u64_to_decoder(event).ok()?;
        
        match decoder {
            DecodeType::Key(key, key_type) => {
                let button = match key {
                    Key::MouseLeft => PointerEventButton::Left,
                    Key::MouseMid => PointerEventButton::Middle,
                    Key::MouseRight => PointerEventButton::Right,
                    Key::MouseScrollDown | Key::MouseScrollUp => PointerEventButton::Other,
                    k => {
                        let str = key2special(k)?;
                        let event = match key_type {
                            KeyType::Press => WindowEvent::KeyPressed { text: str },
                            KeyType::Release => WindowEvent::KeyReleased { text: str },
                        };
                        return Some(event);
                    }
                };
                let (x, y) = self.scale(*cx, *cy);
                let event = match key_type {
                    KeyType::Press => {
                        press!(x, y, button)
                    }
                    KeyType::Release => {
                        release!(x, y, button)
                    }
                };
                Some(event)
            }
            DecodeType::Mouse(mouse) => match mouse {
                Mouse::X(abs_x) => {
                    *cx = abs_x;
                    let (x, y) = self.scale(abs_x, *cy);
                    Some(moved!(x, y))
                }
                Mouse::Y(abs_y) => {
                    *cy = abs_y;
                    let (x, y) = self.scale(*cx, abs_y);
                    Some(moved!(x, y))
                }
                Mouse::ScrollDown => {
                    let (x, y) = self.scale(*cx, *cy);
                    Some(scrolled!(x, y, 0.0, 1.0))
                }
                Mouse::ScrollUp => {
                    let (x, y) = self.scale(*cx, *cy);
                    Some(scrolled!(x, y, 0.0, -1.0))
                }
            },
        }
    }
}

/// u64 -> Virtio Decoder
fn u64_to_decoder(event: u64) -> Result<DecodeType, ()> {
    let dtype = (event >> 48) as usize;
    let code = (event >> 32) & 0xffff;
    let val = (event & 0xffffffff) as i32;
    
    Decoder::decode(dtype, code as usize, val as isize)
}

fn key2special(key: Key) -> Option<SharedString> {
    let key = match key {
        Key::ESC => slint::platform::Key::Escape.into(),
        Key::BackSpace => slint::platform::Key::Backspace.into(),
        Key::Tab => slint::platform::Key::Tab.into(),
        Key::Enter => slint::platform::Key::Return.into(),
        Key::LCTRL => slint::platform::Key::Control.into(),
        Key::LSHIFT => slint::platform::Key::Shift.into(),
        Key::RSHIFT => slint::platform::Key::ShiftR.into(),
        Key::LALT => slint::platform::Key::Alt.into(),
        Key::CAPS => slint::platform::Key::CapsLock.into(),
        Key::A => 'a'.into(),
        Key::B => 'b'.into(),
        Key::C => 'c'.into(),
        Key::D => 'd'.into(),
        Key::E => 'e'.into(),
        Key::F => 'f'.into(),
        Key::G => 'g'.into(),
        Key::H => 'h'.into(),
        Key::I => 'i'.into(),
        Key::J => 'j'.into(),
        Key::K => 'k'.into(),
        Key::L => 'l'.into(),
        Key::M => 'm'.into(),
        Key::N => 'n'.into(),
        Key::O => 'o'.into(),
        Key::P => 'p'.into(),
        Key::Q => 'q'.into(),
        Key::R => 'r'.into(),
        Key::S => 's'.into(),
        Key::T => 't'.into(),
        Key::U => 'u'.into(),
        Key::V => 'v'.into(),
        Key::W => 'w'.into(),
        Key::X => 'x'.into(),
        Key::Y => 'y'.into(),
        Key::Z => 'z'.into(),
        Key::Zero => '0'.into(),
        Key::One => '1'.into(),
        Key::Two => '2'.into(),
        Key::Three => '3'.into(),
        Key::Four => '4'.into(),
        Key::Five => '5'.into(),
        Key::Six => '6'.into(),
        Key::Seven => '7'.into(),
        Key::Eight => '8'.into(),
        Key::Nine => '9'.into(),
        Key::Space => ' '.into(),
        Key::Minus => '-'.into(),
        Key::Equal => '='.into(),
        Key::BackSlash => '\\'.into(),
        Key::Colon => ';'.into(),
        Key::Comma => ','.into(),
        Key::Dot => '.'.into(),
        Key::SineglePoint => '\''.into(),
        Key::Slash => '/'.into(),
        _ => return None,
    };
    Some(key)
}
