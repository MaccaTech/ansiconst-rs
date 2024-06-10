//! # Ansi Constants
//!
//! A library for declaring nestable ANSI styles in const context.
//!
//! ## Motivation
//!
//! The primary motivation of this crate is to provide the ability to use
//! ANSI colours/effects in command-line programs by identifying them
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
//! use ansiconst::*;
//! use ansiconst::Colour::{Green, Blue};
//! use ansiconst::Effect::{Bold, Underline, Italic};
//!
//! // Define styles as Ansi structs:
//! const    HEADING_ANSI: Ansi = ansi!(Green, Bold, Underline);
//! const SUBHEADING_ANSI: Ansi = ansi!(Blue, Italic);
//! const      RESET_ANSI: Ansi = Ansi::reset();
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
//! For this reason, the use of [`Ansi256`](Colour::Ansi256) and [`Rgb`](Colour::Rgb) colours
//! is gated behind feature flags, because supporting them means [`Ansi`] instances
//! must be ever so slightly bigger. Consider the memory sizes:
//!
//! | Type                   | Bytes |
//! |------------------------|-------|
//! | `Ansi`                 |    6  |
//! | `Ansi feature=Ansi256` |    8  |
//! | `Ansi feature=Rgb`     |   12  |
//! | `&'static str`         |   16  |
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
//! use ansiconst::{*, Colour::Red, Effect::Bold};
//!
//! let pet = "cat";
//! let age = 5;
//! let string1 =             styled!(Red, Bold, "My cat is 5 years old").to_string();
//! let string2 =      styled_format!(Red, Bold, "My {} is {} years old", pet, age);
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
//! let mut sink = String::new();
//! styled_writeln!(&mut sink, Red, Bold, "My {} is {} years old", pet, age).unwrap();
//! assert_eq!(sink, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n");
//! ```
//!
//! ### Effortless Nesting
//!
//! Nesting of ANSI codes is automatically handled, and uses the minimum ANSI code
//! sequences when transitioning between nesting levels.
//!
//! Additionally, nested ANSI codes can be disabled entirely, or on a per-attribute basis.
//! Parent [`Ansi`]s can prevent nested [`Ansi`]s from rendering ANSI codes for any/all
//! attributes by *protecting* those attributes in the outer [`Ansi`], using methods such as
//! [`.protect_attrs()`](Ansi::protect_attrs) and [`.only()`](Ansi::only).
//!
//! ##### Examples
//!
//! ```
//! use ansiconst::{*, Effect::{Bold, Underline}};
//!
//! const INNER:           Styled<&str> = styled!(Underline,        "Inner");
//! const INNER_PROTECTED: Styled<&str> = styled!(Underline.only(), "Inner");
//!
//! // Example 1: blended styles
//! assert_eq!(
//!     styled_format!(Bold, "Bold {INNER} Bold again"),
//!     // "Inner" is both Bold and Underline
//!     "\x1B[1mBold \x1B[4mInner\x1B[24m Bold again\x1B[22m"
//! );
//!
//! // Example 2: protected inner style
//! assert_eq!(
//!     styled_format!(Bold, "Bold {INNER_PROTECTED} Bold again"),
//!     // "Inner" is not Bold, only Underline, due to inner's .only()
//!     "\x1B[1mBold \x1B[22;4mInner\x1B[24;1m Bold again\x1B[22m"
//! );
//!
//! // Example 3: protected outer style
//! assert_eq!(
//!     // Note: outer Bold.only() this time
//!     styled_format!(Bold.only(), "Bold {INNER} Bold again"),
//!     // Entire string is Bold, nested Underline was ignored
//!     "\x1B[1mBold Inner Bold again\x1B[22m"
//! );
//!
//! // Example 4: both protected
//! assert_eq!(
//!     // Note: Bold.only() again
//!     styled_format!(Bold.only(), "Bold {INNER_PROTECTED} Bold again"),
//!     // Entire string is Bold, because outer's .only() takes precedence over inner's
//!     "\x1B[1mBold Inner Bold again\x1B[22m"
//! );
//!
//! ```
//!
//! _Note:_ automatic handling of nested styles is achieved by storing the last-applied
//! ANSI style in a [`thread_local!`] static variable, and therefore this library
//! requires `std`. See [`Styled<T>`] for details.
//!
//! ## Examples
//!
//! ```
//! use ansiconst::*;
//! use ansiconst::Colour::{Green, Cyan, Yellow, Purple};
//! use ansiconst::Effect::{Bold, NotBold, Italic, Underline, Blink};
//!
//! const HEADING:    Ansi = ansi!(Green, Bold, Underline);
//! const SUBHEADING: Ansi = ansi!(Cyan, Italic);
//! const STRONG:     Ansi = ansi!(Yellow, Bold);
//! const STRONGER:   Ansi = ansi!(Blink);
//! const STRONGEST:  Ansi = ansi!(Purple, NotBold);
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
//! println!("This sentence shows another {} colours/effects.",
//!     styled_format_args!(Green, Italic, "way of styling {} i.e. with inline",
//!         styled_format_args!(Yellow, Bold, "your text,")
//!     )
//! );
//! ```

mod ansi;
mod fmt;
pub mod io;
pub(crate) mod write;
#[doc(hidden)]
pub mod str;

pub use ansi::{Ansi, Attrs, Colour, Effect};
pub use fmt::Styled;

/// Creates an ANSI style as an [`Ansi`] `const`.
///
/// Accepts any number of [`Ansi`]s, [`Colour`]s, [`Effect`]s or any values with an
/// `ansi()` method.
///
/// The benefit of an [`Ansi`] `const` over a `&'static str` ANSI code is that
/// nesting of styles is handled automatically. See [`Styled<T>`] for details.
///
/// ### Example
///
/// ```
/// use ansiconst::*;
/// use ansiconst::Colour::{Green, Blue};
/// use ansiconst::Effect::{Bold, Underline, Italic};
///
/// const MY_ANSI: Ansi = ansi!(Green, Blue.bg(), Bold, Underline, Italic);
///
/// assert_eq!(&MY_ANSI.to_string(), "\x1B[1;3;4;32;44m");
/// ```
#[macro_export]
macro_rules! ansi {
    // Base case:
    () => ($crate::Ansi::unspecified());
    // Base case:
    ($x:expr) => ($x.ansi());
    // Recurse:
    ($x:expr, $($y:expr),+) => (
        $x.ansi().add($crate::ansi!($($y),+))
    )
}

/// Creates an ANSI style as a `&'static str`.
///
/// Accepts any number of [`Ansi`]s, [`Colour`]s, [`Effect`]s or any values with an
/// `ansi()` method.
///
/// ### Example
///
/// ```
/// use ansiconst::*;
/// use ansiconst::Colour::{Green, Blue};
/// use ansiconst::Effect::{Bold, Underline, Italic};
///
/// const MY_ANSI: &str = ansi_code!(Green, Blue.bg(), Bold, Underline, Italic);
///
/// assert_eq!(MY_ANSI, "\x1B[1;3;4;32;44m");
/// ```
#[macro_export]
macro_rules! ansi_code {
    ($ansi:expr) => {{
        const CODES: $crate::str::Buffer<[u8;25]> = $crate::str::Buffer::from_ansi($ansi.ansi());
        const BYTES_LEN: usize                    = $crate::str::len_as_ansi_bytes(&CODES);
        const BYTES: [u8; BYTES_LEN]              = $crate::str::to_ansi_bytes::<BYTES_LEN>(&CODES);
        const BYTES_PTR: *const [u8]              = &BYTES;
        const STR: &str                           = unsafe { std::mem::transmute(BYTES_PTR) };
        STR
    }};
    ($($ansi:expr),+) => {{
        const ANSI: $crate::Ansi = $crate::ansi!($($ansi),+);
        $crate::ansi_code!(ANSI)
    }}
}

/// Creates an ANSI-styled value.
///
/// Accepts any number of [`Ansi`]s, [`Colour`]s, [`Effect`]s or any values with an
/// `ansi()` method, followed by the final argument that is an instance of `T`.
///
/// Returns a [`Styled<T>`].
///
/// ### Example
/// ```
/// use ansiconst::{*, Colour::Red, Effect::{Italic, Blink}};
///
/// const HELLO: Styled<&str> = styled!(Red.bg(), Italic, Blink, "Hello World!");
///
/// assert_eq!(HELLO.to_string(), String::from("\x1B[3;5;41mHello World!\x1B[23;25;49m"));
/// ```
#[macro_export]
macro_rules! styled {
    // Base case:
    ($ansi:expr, $target:expr) => ($crate::Styled::new($ansi.ansi(), $target));
    // Recurse:
    ($x:expr, $y:expr, $($args:tt)+) => (
        $crate::styled!($x.ansi().add($y.ansi()), $($args)+)
    )
}

/// Like [`format!`] except creates an ANSI-styled `String`.
///
/// The syntax is the same as [`format!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// Note that the result is a plain `String` whose ANSI codes are now baked-in,
/// and so can no longer be changed by nesting inside other styles, unlike
/// [`Styled<T>`].
///
/// ### Example
/// ```
/// use ansiconst::{*, Colour::Red, Effect::{Italic, Blink}};
///
/// let pet = "cat";
/// let age = 5;
/// let styled_string = styled_format!(Red.bg(), Italic, Blink, "My {} is {} years old", pet, age);
///
/// assert_eq!(
///     styled_string,
///     "\x1B[3;5;41mMy cat is 5 years old\x1B[23;25;49m"
/// );
/// ```
#[macro_export]
macro_rules! styled_format {
    ($($args:tt)*) => {{
        $crate::styled_format_args!($($args)*).to_string()
    }}
}

/// Like [`format_args!`] except creates ANSI-styled `Arguments`.
///
/// The syntax is the same as [`format_args!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// Returns a [`Styled<std::fmt::Arguments>`].
///
/// ### Example
/// ```
/// use ansiconst::{*, Colour::Red, Effect::{Italic, Blink}};
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
    ($ansi:expr, $lit:literal $(,)?) => ($crate::Styled::new($ansi.ansi(), format_args!($lit)));
    // Base case:
    ($ansi:expr, $lit:literal, $($args:tt)*) => ($crate::Styled::new($ansi.ansi(), format_args!($lit, $($args)*)));
    // Recurse:
    ($x:expr, $y:expr, $($args:tt)+) => (
        $crate::styled_format_args!($x.ansi().add($y.ansi()), $($args)+)
    )
}

/// Like [`write!`] except with ANSI-styled output.
///
/// The syntax is the same as [`write!`], except that any parameters before the
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// ### Example
/// ```
/// use ansiconst::{*, Colour::Red, Effect::{Italic, Blink}};
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
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// ### Example
/// ```
/// use ansiconst::{*, Colour::Red, Effect::{Italic, Blink}};
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
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// Prints to [`io::ansiout()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::{paint, Colour::Red, Effect::{Italic, Blink}};
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
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// Prints to [`io::ansiout()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::{paintln, Colour::Red, Effect::{Italic, Blink}};
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
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// Prints to [`io::ansierr()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::{epaint, Colour::Red, Effect::{Italic, Blink}};
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
/// format literal must be either instances of [`Ansi`], [`Colour`] or [`Effect`],
/// or else values that have an `ansi()` method.
///
/// Prints to [`io::ansierr()`], which may optionally disable ANSI-styles.
///
/// ### Example
/// ```
/// use ansiconst::{epaintln, Colour::Red, Effect::{Italic, Blink}};
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
