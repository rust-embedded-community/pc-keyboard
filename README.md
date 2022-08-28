# pc-keyboard

A simple driver for handling PC keyboards, with both Scancode Set 1 (when
running on a PC) and Scancode Set 2 support (when reading a PS/2 keyboard
output directly).

## Supports:

* Scancode Set 1 and 2
* Dvorak 104-key layout
* US 104-key layout
* UK 105-key layout
* JIS 109-key layout
* Azerty full layout

## Usage

```rust
extern crate pc_keyboard;

use pc_keyboard::{Keyboard, layouts, ScancodeSet2, HandleControl};

fn main() {
	let mut kb = pc_keyboard::Keyboard::new(layouts::Us104Key, ScancodeSet2, HandleControl::MapLettersToUnicode);
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

## [Documentation](https://docs.rs/crate/pc-keyboard)

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.40 and up. It might compile with older versions but that may change in any new patch release.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above, without
any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Rust Embedded Community][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/orgs/rust-embedded-community/teams/all
