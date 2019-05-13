# pc-keyboard

A simple driver for handling PC keyboards, with both Scancode Set 1 (when
running on a PC) and Scancode Set 2 support (when reading a PS/2 keyboard
output directly).

## Supports:

* Scancode Set 1 and 2
* US 104-key layout
* UK 105-key layout
* JIS 109-key layout

## Usage

```rust
extern crate pc_keyboard;

use pc_keyboard::{Keyboard, layouts, ScancodeSet2, HandleControlPlusLetter};

fn main() {
	let mut kb = pc_keyboard::Keyboard::new(layouts::Us104Key, ScancodeSet2, HandleControlPlusLetter::MapToUnicode);
	match kb.add_byte(0x20) {
		Ok(Some(event)) => {
			println!("Event {:?}", event);
		}
		Ok(None) => {
			println!("Need more data");
		}
		Err(e) => {
			println!("Error decoding: {:?}", e);
		}
	}
}
```
