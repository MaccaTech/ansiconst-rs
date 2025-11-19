//! # Ansi Constants
//!
//! A library for declaring nestable ANSI styles in const context.
//!
//! ## Motivation
//!
//! The primary motivation of this crate is to provide the ability to use
//! ANSI colors/effects in command-line programs by identifying them
//! *semantically* rather than *literally*.
//!
//! For example, when calling [`println!`], instead of applying the style
//! *Green, Bold* to the output, it has more semantic meaning to apply
//! a style named *Subheading* that, when rendered (or "formatted" in Rust),
//! produces the output that activates the *Green, Bold* styling.
//!
//! Furthermore, semantic styles should be *nestable*, like styling frameworks
//! such as HTML's CSS. For example, it should be possible to nest text styled
//! with a name like *Manufacturer* inside text styled with a name like
//! *Product Details* and have the literal styles that these semantic styles
//! translate to applied automatically. The nested style's attributes
//! should temporarily layer on top of, or maybe replace, the parent style's
//! attributes as appropriate.
//!
//! The second key motivation of this crate is to support the above at
//! compile-time. I.e. define semantic styles as `const`, and then use
//! them with minimal overhead throughout a command-line program.
//!
//! There are other crates that provide terminal-styling functionality,
//! but none appear to fully support the use-case outlined above.
//! The API provided by this crate in support of this use-case is
//! presented in the following section.
//!
//! ## Key Features
//!
//! ### Compile-Time
//!
//! Declare ANSI codes as `const`. This means the compiler will inline them
//! wherever they are used, potentially improving runtime performance.
//!
//! ##### Examples
//!
//! ```
//! use ansiconst::{Ansi, ansi, ansi_code};
//!
//! // Define styles as Ansi structs:
//! const    HEADING_ANSI: Ansi = ansi!(Green, Bold, Underline);
//! const SUBHEADING_ANSI: Ansi = ansi!(Blue, Italic);
//! const      RESET_ANSI: Ansi = ansi!(Ansi::reset());
//!
//! assert_eq!(   HEADING_ANSI.to_string(), "\x1B[1;4;32m");
//! assert_eq!(SUBHEADING_ANSI.to_string(), "\x1B[3;34m");
//! assert_eq!(     RESET_ANSI.to_string(), "\x1B[0m");
//!
//! // Or, define styles as ANSI codes:
//! const    HEADING_CODE: &str = ansi_code!(Green, Bold, Underline);
//! const SUBHEADING_CODE: &str = ansi_code!(Blue, Italic);
//! const      RESET_CODE: &str = ansi_code!(Ansi::reset());
//!
//! assert_eq!(               HEADING_CODE, "\x1B[1;4;32m");
//! assert_eq!(            SUBHEADING_CODE, "\x1B[3;34m");
//! assert_eq!(                 RESET_CODE, "\x1B[0m");
//! ```
//!
//! ### Small
//!
//! [`Ansi`] instances are designed to be as small as possible. For example, [`Effect`]s
//! are represented internally using bit flags rather than simple `bool`s.
//!
//! For this reason, the use of [`8-bit`](Color::num) and [`RGB`](Color::rgb) colors
//! is gated behind feature flags, because supporting them means [`Ansi`] instances
//! must be ever so slightly bigger. Consider the memory sizes:
//!
//! | Type                      | Bytes |
//! |---------------------------|:-----:|
//! | `Ansi`                    |     6 |
//! | `Ansi feature="color256"` |     8 |
//! | `Ansi feature="rgb"`      |    12 |
//! | `&'static str`            |    16 |
//!
//! ### Simple Macros
//!
//! Apply ANSI codes using macros:
//!
//! - [`styled!`] creates ANSI-styled values without interpolation (e.g. `&'static str`, `u8`).
//! - [`styled_format!`], [`styled_format_args!`] are analogous to
//! [`format!`], [`format_args!`] except that they create ANSI-styled results.
//! - [`styled_write!`], [`styled_writeln!`] are analogous to
//! [`write!`], [`writeln!`] except that they write ANSI-styled output.
//! - [`paint!`], [`paintln!`], [`epaint!`], [`epaintln!`] are analogous to
//! [`print!`], [`println!`], [`eprint!`], [`eprintln!`] except that they print ANSI-styled output.
//!
//! ##### Examples
//!
//! ```
//! use ansiconst::{styled, styled_format, styled_format_args, styled_writeln};
//! use ansiconst::{paintln, epaintln};
//!
//! // Notice how "Red" and "Bold" are automatically available inside the macros.
//! let pet = "cat";
//! let age = 5;
//! let string1 =             styled!(Red, Bold, "My cat is 5 years old").to_string();
//! let string2 =      styled_format!(Red, Bold, "My {} is {} years old", pet, age).to_string();
//! let string3 = styled_format_args!(Red, Bold, "My {} is {} years old", pet, age).to_string();
//!
//! assert_eq!(string1, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");
//! assert_eq!(string2, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");
//! assert_eq!(string3, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");
//!
//! // Print "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n" to stdout and stderr:
//! paintln! (Red, Bold, "My {} is {} years old", pet, age);
//! epaintln!(Red, Bold, "My {} is {} years old", pet, age);
//!
//! // Write "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n" to a writer:
//! use std::fmt::Write;
//! let mut buffer = String::new();
//! styled_writeln!(&mut buffer, Red, Bold, "My {} is {} years old", pet, age).unwrap();
//! assert_eq!(buffer, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n");
//! ```
//!
//! ### Effortless Nesting
//!
//! Nesting of ANSI codes is automatically handled, and uses the minimum ANSI code
//! sequences when transitioning between nesting levels.
//!
//! Additionally, nested ANSI codes can be effectively disabled by setting an
//! [`important`](Ansi::important) modifier on the outer [`Ansi`]'s attributes.
//! This works similarly to HTML's CSS [`!important`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/important)
//! rule, in that an inner `normal` attribute is ignored if the same outer attribute
//! is `important`.
//!
//! Furthermore, nested [`Ansi`]s can prevent automatic inheriting of the style attributes
//! of enclosing [`Ansi`]s by explicitly resetting them using [`only`](Ansi::only).
//! Finally, nested [`Ansi`]s can be disabled entirely with [`no_ansi`](Ansi::no_ansi).
//!
//! ##### Examples
//!
//! ```
//! use ansiconst::{Styled, styled, styled_format_args};
//!
//! const INNER:           Styled<&str> = styled!(Underline,             "Inner");
//! const INNER_ONLY:      Styled<&str> = styled!(Underline.only(),      "Inner");
//! const INNER_IMPORTANT: Styled<&str> = styled!(Underline.important(), "Inner");
//!
//! // Example 1: blended styles
//! assert_eq!(
//!     styled_format_args!(Bold, "Bold {INNER} Bold again").to_string(),
//!     // "Inner" is both Bold and Underline
//!     "\x1B[1mBold \x1B[4mInner\x1B[24m Bold again\x1B[22m"
//! );
//!
//! // Example 2: inner style only
//! assert_eq!(
//!     styled_format_args!(Bold, "Bold {INNER_ONLY} Bold again").to_string(),
//!     // "Inner" is not Bold, only Underline, due to inner's .only()
//!     "\x1B[1mBold \x1B[22;4mInner\x1B[24;1m Bold again\x1B[22m"
//! );
//!
//! // Example 3: outer style only + important
//! assert_eq!(
//!     styled_format_args!(Bold.only().important(), "Bold {INNER_ONLY} Bold again").to_string(),
//!     // Entire string is Bold, nested Underline was ignored due to .only().important()
//!     "\x1B[0;1mBold Inner Bold again\x1B[22m"
//! );
//!
//! // Example 4: both important
//! assert_eq!(
//!     styled_format_args!(Bold.only().important(), "Bold {INNER_IMPORTANT} Bold again").to_string(),
//!     // "Inner" is Bold and Underline, due to inner's .important()
//!     "\x1B[0;1mBold \x1B[4mInner\x1B[24m Bold again\x1B[22m"
//! );
//!
//! ```
//!
//! _Note:_ automatic handling of nested styles is achieved by storing the last-applied
//! ANSI style in a [`thread_local!`] static variable, and therefore this library
//! requires `std`. See [`Styled<T>`] and [`StyledString`] for details.
//!
//! ## Examples
//!
//! ```
//! use ansiconst::{Ansi, ansi, paintln, styled_format_args};
//!
//! const HEADING:    Ansi = ansi!(Green, Bold, Underline);
//! const SUBHEADING: Ansi = ansi!(Cyan, Italic);
//! const STRONG:     Ansi = ansi!(Yellow, Bold);
//! const STRONGER:   Ansi = ansi!(Blink);
//! const STRONGEST:  Ansi = ansi!(Purple, Bold.not());
//!
//! // Styling with paintln!
//! paintln!(HEADING,    "The Book of Rust");
//! paintln!();
//! paintln!(SUBHEADING, "Chapter 1");
//! paintln!();
//!
//! // Styling with println!
//! println!("This sentence shows how {} as you would expect.",
//!     styled_format_args!(STRONG, "styles can be {}, and they combine",
//!         styled_format_args!(STRONGER, "nested to {} depths",
//!             styled_format_args!(STRONGEST, "arbitrary")
//!         )
//!     )
//! );
//! println!("This sentence shows another {} colors/effects.",
//!     styled_format_args!(Green, Italic, "way of styling {} i.e. with inline",
//!         styled_format_args!(Yellow, Bold, "your text,")
//!     )
//! );
//! ```
//! ## Upgrading from v0.1.x
//!
//! Required actions due to breaking changes:
//!
//! - Rename `ansiconst::Colour` to `ansiconst::Color`, or omit this import entirely,
//! since color names (e.g. `Red`) are now automatically available inside API macros.
//! - Rename `features = ["ansi256"]` to `features = ["color256"]` inside `Cargo.toml`.
//! - Rename `Colour::Ansi256(n)` to [`Color::num(n)`](Color::num).
//! - Rename `Colour::Rgb(r,g,b)` to [`Color::rgb(r,g,b)`](Color::rgb).
//! - Rename `Color::Unspecified`, `Effect::Unspecified` to [`Ansi::empty()`].
//! - Rename `Effect::NotBold` to [`Effect::Bold.not()`](Effect::not) (same for other effects).
//! - Rename `styled_format!(...)` to [`styled_format!(...).to_string()`](`styled_format!`) or
//! [`styled_format_args!(...).to_string()`](`styled_format_args!`).

mod ansi;
mod color;
mod effect;
mod fmt;
pub mod introspect;
pub mod io;
pub(crate) mod write;
#[doc(hidden)]
pub mod str;

pub(crate) use ansi::{Toggle, ToggleColor};
pub use ansi::Ansi;
pub use color::{Color, ColorReset, Coloree};
pub use effect::Effect;
pub use fmt::{Styled, StyledString};

/// Creates an ANSI style as an [`Ansi`] `const`.
///
/// Accepts any number of [`Ansi`]s, [`Color`]s, [`Effect`]s or any values with an
/// [`ansi()`](Ansi::ansi) method.
///
/// The benefit of an [`Ansi`] `const` over a `&'static str` ANSI code is that
/// nesting of styles is handled automatically. See [`Styled<T>`] and [`StyledString`]
/// for details.
///
/// ### Example
///
/// ```
/// use ansiconst::{Ansi, ansi};
///
/// const MY_ANSI: Ansi = ansi!(Green, Blue.bg(), Bold, Underline, Italic);
///
/// assert_eq!(&MY_ANSI.to_string(), "\x1B[1;3;4;32;44m");
/// ```
#[macro_export]
macro_rules! ansi {
    // Base case:
    () => ($crate::Ansi::empty());
    // Base case:
    ($x:expr $(,)?) => ({ #![allow(unused_imports)] use $crate::{Ansi, Color::{self, *}, Effect::*}; $x }.ansi());
    // Recurse:
    ($x:expr, $($y:expr),+ $(,)?) => (
        { #![allow(unused_imports)] use $crate::{Ansi, Color::{self, *}, Effect::*}; $x }.ansi().add($crate::ansi!($($y),+))
    )
}

/// Creates an ANSI style as a `&'static str`.
///
/// Accepts any number of [`Ansi`]s, [`Color`]s, [`Effect`]s or any values with an
/// [`ansi()`](Ansi::ansi) method.
///
/// ### Example
///
/// ```
/// use ansiconst::ansi_code;
///
/// const MY_ANSI: &str = ansi_code!(Green, Blue.bg(), Bold, Underline, Italic);
///
/// assert_eq!(MY_ANSI, "\x1B[1;3;4;32;44m");
/// ```
#[macro_export]
macro_rules! ansi_code {
    ($ansi:expr $(,)?) => {{
        const CODES: $crate::str::Buffer<[u8;25]> = $crate::str::Buffer::from_ansi($crate::ansi!($ansi));
        const BYTES_LEN: usize                    = $crate::str::len_as_ansi_bytes(&CODES);
        const BYTES: [u8; BYTES_LEN]              = $crate::str::to_ansi_bytes::<BYTES_LEN>(&CODES);
        const BYTES_PTR: *const [u8]              = &BYTES;
        const STR: &str                           = unsafe { std::mem::transmute(BYTES_PTR) };
        STR
    }};
    ($($ansi:expr),+ $(,)?) => {{
        const ANSI: $crate::Ansi = $crate::ansi!($($ansi),+);
        $crate::ansi_code!(ANSI)
    }}
}

/// Creates an ANSI-styled value.
///
/// Accepts any number of [`Ansi`]s, [`Color`]s, [`Effect`]s or any values with an
/// [`ansi()`](Ansi::ansi) method, followed by the final argument that is an instance of `T`.
///
/// Returns a [`Styled<T>`].
///
/// ### Example
/// ```
/// use ansiconst::{Styled, styled};
///
/// const HELLO: Styled<&str> = styled!(Red.bg(), Italic, Blink, "Hello World!");
///
/// assert_eq!(HELLO.to_string(), String::from("\x1B[3;5;41mHello World!\x1B[23;25;49m"));
/// ```
#[macro_export]
macro_rules! styled {
    // Base case:
    ($ansi:expr, $target:expr) => ($crate::Styled::new($crate::ansi!($ansi), $target));
    // Recurse:
    ($x:expr, $y:expr, $($args:tt)+) => (
        $crate::styled!($crate::ansi!($x).add($crate::ansi!($y)), $($args)+)
    )
}

/// Like [`format!`] except creates an ANSI [`StyledString`].
///
/// The syntax is the same as [`format!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// Returns a [`StyledString`], which differs from a plain [`String`] in that the
/// contained [`Ansi`] styles can still be overridden by nesting the [`StyledString`]
/// in a [`styled_format`] or [`styled_format_args`].
///
/// Note that this capability comes with some additional runtime overhead. Therefore in
/// performance-critical situations, [`styled_format_args`] is preferable, since it
/// is more efficient with minimal runtime overhead.
///
/// ### Example
/// ```
/// use ansiconst::styled_format;
///
/// let pet = "cat";
/// let age = 5;
/// let styled_string = styled_format!(Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
///
/// assert_eq!(
///     styled_string.to_string(),
///     "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m"
/// );
///
/// // Override the style
/// assert_eq!(
///     styled_format!(Ansi::no_ansi(), "{}", styled_string).to_string(),
///     "My cat is 5 years old"
/// );
/// ```
#[macro_export]
macro_rules! styled_format {
    ($($args:tt)*) => {{
        $crate::styled_format_args!($($args)*).to_styled_string()
    }}
}

/// Like [`format_args!`] except creates ANSI-styled [`Arguments`](std::fmt::Arguments).
///
/// The syntax is the same as [`format_args!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// Returns a [`Styled<std::fmt::Arguments>`].
///
/// ### Example
/// ```
/// use ansiconst::styled_format_args;
///
/// let pet = "cat";
/// let age = 5;
///
/// assert_eq!(
///     styled_format_args!(Red.bg(), Italic, Blink, "My {} is {} years old", pet, age).to_string(),
///     "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m"
/// );
/// ```
#[macro_export]
macro_rules! styled_format_args {
    // Base case:
    ($ansi:expr, $lit:literal $(,)?) => ($crate::Styled::new($crate::ansi!($ansi), format_args!($lit)));
    // Base case:
    ($ansi:expr, $lit:literal, $($args:tt)*) => ($crate::Styled::new($crate::ansi!($ansi), format_args!($lit, $($args)*)));
    // Recurse:
    ($x:expr, $y:expr, $($args:tt)+) => (
        $crate::styled_format_args!($crate::ansi!($x).add($crate::ansi!($y)), $($args)+)
    )
}

/// Like [`write!`] except with ANSI-styled output.
///
/// The syntax is the same as [`write!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// ### Example
/// ```
/// use ansiconst::styled_write;
/// use std::fmt::Write;
///
/// let pet = "cat";
/// let age = 5;
///
/// let mut output = String::new();
/// styled_write!(&mut output, Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
///
/// assert_eq!(output, "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m");
/// ```
#[macro_export]
macro_rules! styled_write {
    // Unstyled
    ($dst:expr, $lit:literal) => {{ write!($dst, $lit) }};
    ($dst:expr, $lit:literal, $($args:tt)*) => {{ write!($dst, $lit, $($args)*) }};
    // Styled
    ($dst:expr, $($args:tt)+) => {{ write!($dst, "{}", $crate::styled_format_args!($($args)*)) }};
}

/// Like [`writeln!`] except with ANSI-styled output.
///
/// The syntax is the same as [`writeln!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// ### Example
/// ```
/// use ansiconst::styled_writeln;
/// use std::fmt::Write;
///
/// let pet = "cat";
/// let age = 5;
///
/// let mut output = String::new();
/// styled_writeln!(&mut output, Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
///
/// assert_eq!(output, "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m\n");
/// ```
#[macro_export]
macro_rules! styled_writeln {
    // Unstyled
    ($dst:expr $(,)?) => {{ writeln!($dst) }};
    ($dst:expr, $lit:literal) => {{ writeln!($dst, $lit) }};
    ($dst:expr, $lit:literal, $($args:tt)*) => {{ writeln!($dst, $lit, $($args)*) }};
    // Styled
    ($dst:expr, $($args:tt)+) => {{ writeln!($dst, "{}", $crate::styled_format_args!($($args)*)) }};
}

/// Like [`print!`] except with ANSI-styled output.
///
/// The syntax is the same as [`print!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// Prints to [`io::ansiout()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::paint;
///
/// let pet = "cat";
/// let age = 5;
///
/// paint!(Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
/// // Prints "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m"
/// ```
#[macro_export]
macro_rules! paint {
    // Unstyled
    ($lit:literal) => {{ print!($lit) }};
    ($lit:literal, $($args:tt)*) => {{ print!($lit, $($args)*) }};
    // Styled
    ($($args:tt)*) => {{
        write!($crate::io::ansiout(), "{}", $crate::styled_format_args!($($args)*)).unwrap()
    }};
}

/// Like [`println!`] except with ANSI-styled output.
///
/// The syntax is the same as [`println!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// Prints to [`io::ansiout()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::paintln;
///
/// let pet = "cat";
/// let age = 5;
///
/// paintln!(Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
/// // Prints "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m\n"
/// ```
#[macro_export]
macro_rules! paintln {
    // Unstyled
    () => {{ println!() }};
    ($lit:literal) => {{ println!($lit) }};
    ($lit:literal, $($args:tt)*) => {{ println!($lit, $($args)*) }};
    // Styled
    ($($args:tt)*) => {{
        writeln!($crate::io::ansiout(), "{}", $crate::styled_format_args!($($args)*)).unwrap()
    }};
}

/// Like [`eprint!`] except with ANSI-styled output.
///
/// The syntax is the same as [`eprint!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// Prints to [`io::ansierr()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::epaint;
///
/// let pet = "cat";
/// let age = 5;
///
/// epaint!(Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
/// // Prints "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m"
/// ```
#[macro_export]
macro_rules! epaint {
    // Unstyled
    ($lit:literal) => {{ eprint!($lit) }};
    ($lit:literal, $($args:tt)*) => {{ eprint!($lit, $($args)*) }};
    // Styled
    ($($args:tt)*) => {{
        write!($crate::io::ansierr(), "{}", $crate::styled_format_args!($($args)*)).unwrap()
    }};
}

/// Like [`eprintln!`] except with ANSI-styled output.
///
/// The syntax is the same as [`eprintln!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Color`] or [`Effect`],
/// or else values that have an [`ansi()`](Ansi::ansi) method.
///
/// Prints to [`io::ansierr()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::epaintln;
///
/// let pet = "cat";
/// let age = 5;
///
/// epaintln!(Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
/// // Prints "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m\n"
/// ```
#[macro_export]
macro_rules! epaintln {
    // Unstyled
    () => {{ eprintln!() }};
    ($lit:literal) => {{ eprintln!($lit) }};
    ($lit:literal, $($args:tt)*) => {{ eprintln!($lit, $($args)*) }};
    // Styled
    ($($args:tt)*) => {{
        writeln!($crate::io::ansierr(), "{}", $crate::styled_format_args!($($args)*)).unwrap()
    }};
}
