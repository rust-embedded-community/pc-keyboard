//! A collection of Scancode implementations

mod set1;
mod set2;
mod usbhid;

pub use self::set1::ScancodeSet1;
pub use self::set2::ScancodeSet2;
pub use self::usbhid::{convert as usb_convert, Modifiers as UsbModifiers};
