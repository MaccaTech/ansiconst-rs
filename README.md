# ansiconst

[![Docs](https://docs.rs/ansiconst/badge.svg)](https://docs.rs/ansiconst) [![Crates.io](https://img.shields.io/crates/v/ansiconst.svg)](https://crates.io/crates/ansiconst)

## Contents

* [Motivation](#Motivation)
* [Key Features](#Key-Features)
* [Examples](#Examples-3)
* [Upgrading from v0.1.x](#upgrading-from-v01x)
* [Version History](#Version-History)
* [Licence](#Licence)

## Ansi Constants

A library for declaring nestable ANSI styles in const context.

### Motivation

The primary motivation of this crate is to provide the ability to use
ANSI colors/effects in command-line programs by identifying them
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
presented in the following section.

### Key Features

#### Compile-Time

Declare ANSI codes as `const`. This means the compiler will inline them
wherever they are used, potentially improving runtime performance.

###### Examples

```rust
use ansiconst::{Ansi, ansi, ansi_code};

// Define styles as Ansi structs:
const    HEADING_ANSI: Ansi = ansi!(Green, Bold, Underline);
const SUBHEADING_ANSI: Ansi = ansi!(Blue, Italic);
const      RESET_ANSI: Ansi = ansi!(Ansi::reset());

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

For this reason, the use of [`8-bit`][Color::num] and [`RGB`][Color::rgb] colors
is gated behind feature flags, because supporting them means [`Ansi`] instances
must be ever so slightly bigger. Consider the memory sizes:

| Type                      | Bytes |
|---------------------------|:-----:|
| `Ansi`                    |     6 |
| `Ansi feature="color256"` |     8 |
| `Ansi feature="rgb"`      |    12 |
| `&'static str`            |    16 |

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
use ansiconst::{styled, styled_format, styled_format_args, styled_writeln};
use ansiconst::{paintln, epaintln};

// Notice how "Red" and "Bold" are automatically available inside the macros.
let pet = "cat";
let age = 5;
let string1 =             styled!(Red, Bold, "My cat is 5 years old").to_string();
let string2 =      styled_format!(Red, Bold, "My {} is {} years old", pet, age).to_string();
let string3 = styled_format_args!(Red, Bold, "My {} is {} years old", pet, age).to_string();

assert_eq!(string1, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");
assert_eq!(string2, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");
assert_eq!(string3, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m");

// Print "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n" to stdout and stderr:
paintln! (Red, Bold, "My {} is {} years old", pet, age);
epaintln!(Red, Bold, "My {} is {} years old", pet, age);

// Write "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n" to a writer:
use std::fmt::Write;
let mut buffer = String::new();
styled_writeln!(&mut buffer, Red, Bold, "My {} is {} years old", pet, age).unwrap();
assert_eq!(buffer, "\x1B[1;31mMy cat is 5 years old\x1B[22;39m\n");
```

#### Effortless Nesting

Nesting of ANSI codes is automatically handled, and uses the minimum ANSI code
sequences when transitioning between nesting levels.

Additionally, nested ANSI codes can be effectively disabled by setting an
[`important`][Ansi::important] modifier on the outer [`Ansi`]'s attributes.
This works similarly to HTML's CSS [`!important`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/important)
rule, in that an inner `normal` attribute is ignored if the same outer attribute
is `important`.

Furthermore, nested [`Ansi`]s can prevent automatic inheriting of the style attributes
of enclosing [`Ansi`]s by explicitly resetting them using [`only`][Ansi::only].
Finally, nested [`Ansi`]s can be disabled entirely with [`no_ansi`][Ansi::no_ansi].

###### Examples

```rust
use ansiconst::{Styled, styled, styled_format_args};

const INNER:           Styled<&str> = styled!(Underline,             "Inner");
const INNER_ONLY:      Styled<&str> = styled!(Underline.only(),      "Inner");
const INNER_IMPORTANT: Styled<&str> = styled!(Underline.important(), "Inner");

// Example 1: blended styles
assert_eq!(
    styled_format_args!(Bold, "Bold {INNER} Bold again").to_string(),
    // "Inner" is both Bold and Underline
    "\x1B[1mBold \x1B[4mInner\x1B[24m Bold again\x1B[22m"
);

// Example 2: inner style only
assert_eq!(
    styled_format_args!(Bold, "Bold {INNER_ONLY} Bold again").to_string(),
    // "Inner" is not Bold, only Underline, due to inner's .only()
    "\x1B[1mBold \x1B[22;4mInner\x1B[24;1m Bold again\x1B[22m"
);

// Example 3: outer style only + important
assert_eq!(
    styled_format_args!(Bold.only().important(), "Bold {INNER_ONLY} Bold again").to_string(),
    // Entire string is Bold, nested Underline was ignored due to .only().important()
    "\x1B[0;1mBold Inner Bold again\x1B[22m"
);

// Example 4: both important
assert_eq!(
    styled_format_args!(Bold.only().important(), "Bold {INNER_IMPORTANT} Bold again").to_string(),
    // "Inner" is Bold and Underline, due to inner's .important()
    "\x1B[0;1mBold \x1B[4mInner\x1B[24m Bold again\x1B[22m"
);

```

_Note:_ automatic handling of nested styles is achieved by storing the last-applied
ANSI style in a [`thread_local!`] static variable, and therefore this library
requires `std`. See [`Styled<T>`] and [`StyledString`] for details.

### Examples

```rust
use ansiconst::{Ansi, ansi, paintln, styled_format_args};

const HEADING:    Ansi = ansi!(Green, Bold, Underline);
const SUBHEADING: Ansi = ansi!(Cyan, Italic);
const STRONG:     Ansi = ansi!(Yellow, Bold);
const STRONGER:   Ansi = ansi!(Blink);
const STRONGEST:  Ansi = ansi!(Purple, Bold.not());

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
println!("This sentence shows another {} colors/effects.",
    styled_format_args!(Green, Italic, "way of styling {} i.e. with inline",
        styled_format_args!(Yellow, Bold, "your text,")
    )
);
```
### Upgrading from v0.1.x

Required actions due to breaking changes:

- Rename `ansiconst::Colour` to `ansiconst::Color`, or omit this import entirely,
since color names (e.g. `Red`) are now automatically available inside API macros.
- Rename `features = ["ansi256"]` to `features = ["color256"]` inside `Cargo.toml`.
- Rename `Colour::Ansi256(n)` to [`Color::num(n)`][Color::num].
- Rename `Colour::Rgb(r,g,b)` to [`Color::rgb(r,g,b)`][Color::rgb].
- Rename `Color::Unspecified`, `Effect::Unspecified` to [`Ansi::empty()`].
- Rename `Effect::NotBold` to [`Effect::Bold.not()`][Effect::not] (same for other effects).
- Rename `styled_format!(...)` to [`styled_format!(...).to_string()`][`styled_format!`] or
[`styled_format_args!(...).to_string()`][`styled_format_args!`].

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
[`Color`]:               https://docs.rs/ansiconst/latest/ansiconst/enum.Color.html
[`Effect`]:              https://docs.rs/ansiconst/latest/ansiconst/enum.Effect.html
[`Styled<T>`]:           https://docs.rs/ansiconst/latest/ansiconst/struct.Styled.html
[`StyledString`]:        https://docs.rs/ansiconst/latest/ansiconst/struct.StyledString.html
[`styled!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.styled.html
[`styled_format!`]:      https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format.html
[`styled_format_args!`]: https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format_args.html
[`styled_write!`]:       https://docs.rs/ansiconst/latest/ansiconst/macro.styled_write.html
[`styled_writeln!`]:     https://docs.rs/ansiconst/latest/ansiconst/macro.styled_writeln.html
[`paint!`]:              https://docs.rs/ansiconst/latest/ansiconst/macro.paint.html
[`paintln!`]:            https://docs.rs/ansiconst/latest/ansiconst/macro.paintln.html
[`epaint!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.epaint.html
[`epaintln!`]:           https://docs.rs/ansiconst/latest/ansiconst/macro.epaintln.html
[Color::num]:            https://docs.rs/ansiconst/latest/ansiconst/enum.Color.html#method.num
[Color::rgb]:            https://docs.rs/ansiconst/latest/ansiconst/enum.Color.html#method.rgb
[Effect::not]:           https://docs.rs/ansiconst/latest/ansiconst/enum.Effect.html#method.not
[`Ansi::empty()`]:       https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.empty
[Ansi::important]:       https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.important
[Ansi::only]:            https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.only
[Ansi::no_ansi]:         https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.no_ansi

## Version History

<table>
<thead>
<tr>
    <th>Version</th>
    <th>Date</th>
    <th>Comments</th>
</tr>
</thead>
<tbody>
<tr>
    <td>v0.2.1</td>
    <td>19-Nov 2025</td>
    <td>Enhancement release
        <ul>
        <li>Add <code>important</code> modifier (similar to CSS <code>!important</code>)</li>
        <li>Add <code>StyledString</code> to allow overriding ANSI styles inside plain Strings</li>
        <li>Automatically import colours/effects inside macros</li>
        <li>Add improved introspection API</li>
        <li>Improve test coverage</li>
        <li>Fix some edge cases where suboptimal ANSI codes were emitted to transition between styles</li>
        <li>Fix not treating identical 4-bit/8-bit/rgb colors as equal</li>
        <li>Rename <code>Colour</code> to <code>Color</code></li>
        </ul>
    </td>
</tr>
<tr>
    <td>v0.1.1</td>
    <td>10-Jun 2024</td>
    <td>Bugfix/docs release
        <ul>
        <li>Fix handling of <code>FORCE_COLOR</code>, <code>NO_COLOR</code> env vars</li>
        <li>Fix missing <code>fn</code> for creating <code>AnsiWriter</code> instances</li>
        <li>Fix broken links in <code>README</code></li>
        <li>Add more explanation to Rustdoc</li>
        </ul>
    </td>
</tr>
<tr>
    <td>v0.1.0</td>
    <td>09-Jun 2024</td>
    <td>Initial release</td>
    </td>
</tr>
</tbody>
</table>

## Licence
MIT
