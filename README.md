# ansiconst

[View the Rustdoc](https://docs.rs/ansiconst)

## Ansi Constants

A library for declaring nestable ANSI styles in const context.

#### Motivation

The primary motivation of this crate is to provide the ability to use
ANSI colours/effects in command-line programs by identifying them
*semantically* rather than *literally*.

For example, when calling [`println!`], instead of applying the style
*Green, Bold* to the output, it has more semantic meaning to apply
a style named *Subheading* that, when rendered (or "formatted" in Rust),
produces the output that activates the *Green, Bold* styling.

Furthermore, semantic styles should be *nestable*, like styling frameworks
such as HTML's CSS. For example, it should be possible to nest text styled
with a name like *Manufacturer* inside text styled with a name like
*Product Details* and have the literal styles that these semantic styles
translate to applied automatically. The nested style's attributes
should temporarily layer on top of, or maybe replace, the parent style's
attributes as appropriate.

The second key motivation of this crate is to support the above at
compile-time. I.e. define semantic styles as `const`, and then use
them with minimal overhead throughout a command-line program.

There are other crates that provide terminal-styling functionality,
but none appear to fully support the use-case outlined above.
The API provided by this crate in support of this use-case is
presented as follows:

#### Compile-Time

Declare ANSI codes as `const`. This means the compiler will inline them
wherever they are used, potentially improving runtime performance.

###### Examples

```rust
use ansiconst::*;
use ansiconst::Colour::{Green, Blue};
use ansiconst::Effect::{Bold, Underline, Italic};

// Define styles as Ansi structs:
const    HEADING_ANSI: Ansi = ansi!(Green, Bold, Underline);
const SUBHEADING_ANSI: Ansi = ansi!(Blue, Italic);
const      RESET_ANSI: Ansi = Ansi::reset();

assert_eq!(   HEADING_ANSI.to_string(), "\x1B[1;4;32m");
assert_eq!(SUBHEADING_ANSI.to_string(), "\x1B[3;34m");
assert_eq!(     RESET_ANSI.to_string(), "\x1B[0m");

// Or, define styles as ANSI codes:
const    HEADING_CODE: &str = ansi_code!(Green, Bold, Underline);
const SUBHEADING_CODE: &str = ansi_code!(Blue, Italic);
const      RESET_CODE: &str = ansi_code!(Ansi::reset());

assert_eq!(               HEADING_CODE, "\x1B[1;4;32m");
assert_eq!(            SUBHEADING_CODE, "\x1B[3;34m");
assert_eq!(                 RESET_CODE, "\x1B[0m");
```

#### Small

[`Ansi`] instances are designed to be as small as possible. For example, [`Effect`]s
are represented internally using bit flags rather than simple `bool`s.

For this reason, the use of [`Ansi256`](Colour::Ansi256) and [`Rgb`](Colour::Rgb) colours
is gated behind feature flags, because supporting them means [`Ansi`] instances
must be ever so slightly bigger. Consider the memory sizes:

| Type                   | Bytes |
|------------------------|-------|
| `Ansi`                 |    6  |
| `Ansi feature=Ansi256` |    8  |
| `Ansi feature=Rgb`     |   12  |
| `&'static str`         |   16  |

#### Simple Macros

Apply ANSI codes using macros:

- [`styled!`] creates ANSI-styled values without interpolation (e.g. `&'static str`, `u8`).
- [`styled_format!`], [`styled_format_args!`] are analogous to
[`format!`], [`format_args!`] except that they create ANSI-styled results.
- [`styled_write!`], [`styled_writeln!`] are analogous to
[`write!`], [`writeln!`] except that they write ANSI-styled output.
- [`paint!`], [`paintln!`], [`epaint!`], [`epaintln!`] are analogous to
[`print!`], [`println!`], [`eprint!`], [`eprintln!`] except that they print ANSI-styled output.

###### Examples

```rust
use ansiconst::{*, Colour::Red, Effect::Bold};

let pet = "cat";
let age = 5;
let string1 =             styled!(Red, Bold, "My cat is 5 years old").to_string();
let string2 =      styled_format!(Red, Bold, "My {} is {} years old", pet, age);
let string3 = styled_format_args!(Red, Bold, "My {} is {} years old", pet, age).to_string();

assert_eq!(string1, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");
assert_eq!(string2, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");
assert_eq!(string3, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");

// Print "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n" to stdout and stderr:
paintln! (Red, Bold, "My {} is {} years old", pet, age);
epaintln!(Red, Bold, "My {} is {} years old", pet, age);

// Write "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n" to a writer:
use std::fmt::Write;
let mut sink = String::new();
styled_writeln!(&mut sink, Red, Bold, "My {} is {} years old", pet, age).unwrap();
assert_eq!(sink, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n");
```

#### Effortless Nesting

Nesting of ANSI codes is automatically handled, and uses the minimum ANSI code
sequences when transitioning between nesting levels.

Additionally, nested ANSI codes can be disabled entirely, or on a per-attribute basis.
Parent [`Ansi`]s can prevent nested [`Ansi`]s from rendering ANSI codes for any/all
attributes by *protecting* those attributes in the outer [`Ansi`], using methods such as
[`.protect_attrs()`](Ansi::protect_attrs) and [`.only()`](Ansi::only).

###### Examples

```rust
use ansiconst::{*, Effect::{Bold, Underline}};

const INNER:           Styled<&str> = styled!(Underline,        "Inner");
const INNER_PROTECTED: Styled<&str> = styled!(Underline.only(), "Inner");

// Example 1: blended styles
assert_eq!(
    styled_format!(Bold, "Bold {INNER} Bold again"),
    // "Inner" is both Bold and Underline
    "\x1B[1mBold \x1B[4mInner\x1B[24m Bold again\x1B[22m"
);

// Example 2: protected inner style
assert_eq!(
    styled_format!(Bold, "Bold {INNER_PROTECTED} Bold again"),
    // "Inner" is not Bold, only Underline, due to inner's .only()
    "\x1B[1mBold \x1B[22;4mInner\x1B[24;1m Bold again\x1B[22m"
);

// Example 3: protected outer style
assert_eq!(
    // Note: outer Bold.only() this time
    styled_format!(Bold.only(), "Bold {INNER} Bold again"),
    // Entire string is Bold, nested Underline was ignored
    "\x1B[1mBold Inner Bold again\x1B[22m"
);

// Example 4: both protected
assert_eq!(
    // Note: Bold.only() again
    styled_format!(Bold.only(), "Bold {INNER_PROTECTED} Bold again"),
    // Entire string is Bold, because outer's .only() takes precedence over inner's
    "\x1B[1mBold Inner Bold again\x1B[22m"
);

```

_Note:_ automatic handling of nested styles is achieved by storing the last-applied
ANSI style in a [`thread_local!`] static variable, and therefore this library
requires `std`. See [`Styled<T>`] for details.

## Examples

```rust
use ansiconst::*;
use ansiconst::Colour::{Green, Cyan, Yellow, Purple};
use ansiconst::Effect::{Bold, NotBold, Italic, Underline, Blink};

const HEADING:    Ansi = ansi!(Green, Bold, Underline);
const SUBHEADING: Ansi = ansi!(Cyan, Italic);
const STRONG:     Ansi = ansi!(Yellow, Bold);
const STRONGER:   Ansi = ansi!(Blink);
const STRONGEST:  Ansi = ansi!(Purple, NotBold);

// Styling with paintln!
paintln!(HEADING,    "The Book of Rust");
paintln!();
paintln!(SUBHEADING, "Chapter 1");
paintln!();

// Styling with println!
println!("This sentence shows how {} as you would expect.",
    styled_format_args!(STRONG, "styles can be {}, and they combine",
        styled_format_args!(STRONGER, "nested to {} depths",
            styled_format_args!(STRONGEST, "arbitrary")
        )
    )
);
println!("This sentence shows another {} colours/effects.",
    styled_format_args!(Green, Italic, "way of styling {} i.e. with inline",
        styled_format_args!(Yellow, Bold, "your text,")
    )
);
```

[`print!`]:              https://doc.rust-lang.org/std/macro.print.html
[`println!`]:            https://doc.rust-lang.org/std/macro.println.html
[`eprint!`]:             https://doc.rust-lang.org/std/macro.eprint.html
[`eprintln!`]:           https://doc.rust-lang.org/std/macro.eprintln.html
[`write!`]:              https://doc.rust-lang.org/std/macro.write.html
[`writeln!`]:            https://doc.rust-lang.org/std/macro.writeln.html
[`format!`]:             https://doc.rust-lang.org/std/macro.format.html
[`format_args!`]:        https://doc.rust-lang.org/std/macro.format_args.html
[`thread_local!`]:       https://doc.rust-lang.org/std/macro.thread_local.html

[`Ansi`]:                https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html
[`Colour`]:              https://docs.rs/ansiconst/latest/ansiconst/enum.Colour.html
[`Effect`]:              https://docs.rs/ansiconst/latest/ansiconst/enum.Effect.html
[`Styled<T>`]:           https://docs.rs/ansiconst/latest/ansiconst/struct.Styled.html
[`styled!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.styled.html
[`styled_format!`]:      https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format.html
[`styled_format_args!`]: https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format_args.html
[`styled_write!`]:       https://docs.rs/ansiconst/latest/ansiconst/macro.styled_write.html
[`styled_writeln!`]:     https://docs.rs/ansiconst/latest/ansiconst/macro.styled_writeln.html
[`paint!`]:              https://docs.rs/ansiconst/latest/ansiconst/macro.paint.html
[`paintln!`]:            https://docs.rs/ansiconst/latest/ansiconst/macro.paintln.html
[`epaint!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.epaint.html
[`epaintln!`]:           https://docs.rs/ansiconst/latest/ansiconst/macro.epaintln.html
[`Ansi256`]:             https://docs.rs/ansiconst/latest/ansiconst/enum.Colour.html#variant.Ansi256
[`Rgb`]:                 https://docs.rs/ansiconst/latest/ansiconst/enum.Colour.html#variant.Rgb

License: MIT
