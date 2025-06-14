# pc-keyboard

A simple driver for handling PC keyboards, with both Scancode Set 1 (when
running on a PC) and Scancode Set 2 support (when reading a PS/2 keyboard
output directly).

See <https://docs.rs/pc-keyboard> for documentation, or read [`src/lib.rs`](./src/lib.rs).

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.61 and up. It might compile with older versions but that may change in any new patch release.

## Changelog

There is a changelog in [CHANGELOG.md](./CHANGELOG.md).

## License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above, without
any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][coc], the maintainer of this crate, the [Rust Embedded Community][team], promises
to intervene to uphold that code of conduct.

[coc]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/orgs/rust-embedded-community/people
