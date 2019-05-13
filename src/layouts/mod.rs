//! Implements the various keyboard layouts.
//!
//! We have one layout per file, but where two layouts are similar, you can
//! handle all the 'different' keys first, and then jump to another handler -
//! see UK105 and US104 as an example of that.

mod us104;
pub use self::us104::Us104Key;

mod uk105;
pub use self::uk105::Uk105Key;

mod jis109;
pub use self::jis109::Jis109Key;