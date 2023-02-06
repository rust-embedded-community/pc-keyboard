# Changelog

## Unreleased

* Changed ordering of `enum KeyCode` and names of several of the keys
* Support the mysterious 'Right Control 2' and 'Right Alt 2' so that Pause/Break
  and Print Screen do the right thing.
* Fix the Backslash/Tilde swap on the UK Layout.

## v0.6.1 (20 October 2022)

* Fix Control-Letter codes on AZERTY

## v0.6.0 (29 Aug 2022)

* `Keyboard::new` is now const
* Layout and Scan Code Set are now const-generics, not arguments
* Pause/Break and Print Screen now work correctly
* KeyCode is now non-exhaustive
* KeyState includes 'SingleShot' for keys with no break code
* Add Colemak, Dvorak Programmer and DE layouts
* MSRV increased to Rust 1.61

## v0.5.1 (19 July 2020)

* Add Dvorak, JIS and Azerty layouts

## v0.5.0 (6 Feb 2019)

* Support Ctrl+C generating Unicode U+0003, etc.
* Added tests

## v0.4.1 (5 Feb 2019)

* Support PowerOnTestOk

## v0.4.0 (4 Feb 2019)

* Fixed decoding issues (including Backslash/HashTilde mix-up)

## v0.3.1 (16 Nov 2018)

* Fixed decoding issues
* Added more tests

## v0.3.0 (9 Nov 2018)

* Fixed Scancode Set 1

## v0.2.0 (28 Octo 2018)

* Added Scancode Set 1

## v0.1.1 (22 Aug 2018)

* Metadata fixes

## v0.1.0 (26 Apr 2018)

* First version
