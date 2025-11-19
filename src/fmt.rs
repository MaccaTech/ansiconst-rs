mod display;
mod string;
use display::StyledDisplay;
use string::ToStyledString;
pub use string::StyledString;
use crate::Ansi;
use std::fmt;
use std::ops::Deref;

/// Associates a [`Display`](std::fmt::Display) *target* with an [`Ansi`] *style*,
/// such that formatting produces the result of formatting the *target*
/// with the *style's* ANSI codes wrapped around it.
///
/// Instances of `Styled<T>`:
///
/// - are usually created using the [`styled!`](crate::styled!) or
/// [`styled_format_args!`](crate::styled_format_args!) macros.
/// - can be converted to a plain [`String`] with [`to_string`](std::string::ToString::to_string),
/// or by using the [`format!`] macro.
///
/// *Note: this library provides a similarly-named [`StyledString`], which offers the same
/// functionality as `Styled<T>` but wraps a plain [`String`] internally. While this offers
/// more convenience for certain use cases, there is additional runtime overhead.*
///
/// ## Discussion
///
/// At first glance, it may seem this struct is not strictly necessary.
/// Without relying on `Styled<T>`, an [`Ansi`] instance can be converted
/// into a simple ANSI code in the form of a `&'static str` using the
/// [`ansi_code`](crate::ansi_code) macro. For example:
///
/// ```
/// use ansiconst::ansi_code;
///
/// const RED:     &str = ansi_code!(Red);
/// const NOT_RED: &str = ansi_code!(Color::reset());
///
/// assert_eq!(RED,     "\x1B[31m");
/// assert_eq!(NOT_RED, "\x1B[39m");
///
/// println!("{RED}Hello{NOT_RED}");
/// ```
///
/// However, there are two key advantages of using a `Styled<T>` struct over using
/// ANSI styles in their `&'static str` form:
///
/// ## 1. Automatic Nesting
///
/// Nesting of ANSI styles is *automatically handled*.
/// Specifically, during formatting of nested `Styled<T>` instances, the parent's
/// ANSI style is restored when the child finishes formatting (i.e. by writing
/// into the output any ANSI codes necessary to restore the parent's style).
///
/// By contrast, if a programmer were to use ANSI codes in the form of simple
/// `&'static str`s, that individual would have to manage the nesting transitions
/// manually, which would be unwieldy and potentially error-prone.
///
/// ##### Examples
///
/// ```
/// use ansiconst::{ansi_code, paintln, styled, Styled};
///
/// // -------------------------------
/// // Approach #1: using &'static str
/// // -------------------------------
///
/// const RED:            &str = ansi_code!(Red);                        // "\x1B[31m"
/// const NOT_RED:        &str = ansi_code!(Color::reset());             // "\x1B[39m"
/// const GREEN_BOLD:     &str = ansi_code!(Green, Bold);                // "\x1B[1;32m"
/// const NOT_GREEN_BOLD: &str = ansi_code!(Color::reset(), Bold.not()); // "\x1B[22;39m"
///
/// let inner = format!("{GREEN_BOLD}this is green bold{NOT_GREEN_BOLD}");
/// // Notice how {RED} has to be re-printed after {inner},
/// // because NOT_GREEN_BOLD resets the foreground color.
/// println!("{RED}This is red {inner}{RED} this is red again{NOT_RED}");
///
/// // ----------------------------
/// // Approach #2: using Styled<T>
/// // ----------------------------
///
/// const INNER: Styled<&str> = styled!(Green, Bold, "this is green bold");
/// // Notice how the outer Red is automatically restored after {INNER}
/// paintln!(Red, "This is red {INNER} this is red again");
/// ```
///
/// ## 2. Disabling ANSI
///
/// Nested ANSI styles can be *disabled entirely*, or else have specific ANSI attributes
/// suppressed. This is achieved by using any of this crate's printing/writing/styling macros
/// to style output with an [`Ansi`] style that has [`important`](Ansi::important())
/// attributes set. Under the hood, the macros create an outer [`Styled<Arguments`>]
/// containing an [`Ansi`] with `important` attributes that override those
/// same attributes in any nested `Styled` whose attributes are not `important`.
///
/// ##### Examples
///
/// ```
/// use ansiconst::{Styled, styled, styled_format_args, paintln};
///
/// const RED_MSG: Styled<&str> = styled!(Red, "Hello world!");
///
/// // Prints "\x1B[31mHello world!\x1B[39m\n"
/// println!("{}", RED_MSG);
///
/// // Prints "Hello world!\n"
/// // I.e. ANSI codes are disabled
/// println!("{}", styled_format_args!(Ansi::no_ansi(), "{}", RED_MSG));
///
/// // Prints "Hello world!\n"
/// // I.e. ANSI codes are disabled
/// paintln!(Ansi::no_ansi(), "{}", RED_MSG);
///
/// // Prints "\x1B[34mHello world!\x1B[39m\n"
/// // I.e. in Blue, because outer Blue is important so nested Red is ignored
/// paintln!(Blue.important(), "{}", RED_MSG);
///
/// // --------------------------------------------------
/// // Check the above is correct by capturing the output
/// // --------------------------------------------------
///
/// // Red
/// assert_eq!("\x1B[31mHello world!\x1B[39m",
///            format!("{}", RED_MSG));
///
/// // Plain
/// assert_eq!("Hello world!",
///            styled_format_args!(Ansi::no_ansi(), "{}", RED_MSG).to_string());
///
/// // Blue
/// assert_eq!("\x1B[34mHello world!\x1B[39m",
///            styled_format_args!(Blue.important(), "{}", RED_MSG).to_string());
/// ```
///
/// ## How It Works
///
/// When a `Styled<T>` is formatted, it has no knowledge of whether or
/// not its nested target also contains `Styled<XYZ>` instances that
/// may want to write ANSI codes to the output. And yet, in order to calculate
/// the ANSI codes necessary to transition between parent and child styles,
/// there needs to be some kind of communication of the last-formatted style
/// between the nesting levels.
///
/// Ideally, Rust's standard library would allow specifying a custom [`Formatter`](std::fmt::Formatter)
/// with the ability to hold arbitrary state. This way, it would be possible
/// for a parent `Styled<T>` to pass the current [`Ansi`] style to its children
/// during formatting, so that they could determine the ANSI codes required
/// to handle the transtion from parent style → child style → parent style.
///
/// Since [`Formatter`](std::fmt::Formatter) does not provide this functionality currently,
/// the desired automatic handling of ANSI code transitions for nested `Styled<T>`s
/// is achieved using [`thread_local!`] to pass [`Ansi`] instances between
/// parents and children.
///
/// However, any associated overhead is likely minimal because the only value being
/// stored in [`thread_local!`] is a single [`Ansi`], which is relatively small.
///
/// # Examples
///
/// ```
/// use ansiconst::styled_format_args;
///
/// assert_eq!(
///     styled_format_args!(Red, "Red {} Red",
///         styled_format_args!(Green, "Green {} Green",
///             styled_format_args!(Blue, "Blue")
///         )
///     ).to_string(),
///     "\x1B[31mRed \x1B[32mGreen \x1B[34mBlue\x1B[32m Green\x1B[31m Red\x1B[39m"
/// );
///
/// assert_eq!(
///     styled_format_args!(Italic, "Italic {} Italic",
///         styled_format_args!(Faint, "Both {} Both",
///             styled_format_args!(Italic.not(), "Faint-only")
///         )
///     ).to_string(),
///     "\x1B[3mItalic \x1B[2mBoth \x1B[23mFaint-only\x1B[3m Both\x1B[22m Italic\x1B[23m"
/// );
/// ```
pub struct Styled<T: fmt::Display> { ansi: Ansi, target: T }

impl<T: fmt::Display> Styled<T> {
    /// Creates an instance with the given [`Ansi`] style and target
    #[inline]
    pub const fn new(ansi: Ansi, target: T) -> Styled<T> { Self { ansi, target } }
    /// Creates an instance with an unspecified [`Ansi`] style and the given target
    #[inline]
    pub const fn unstyled(target: T) -> Styled<T> { Self { ansi: Ansi::empty(), target } }
    /// Gets the [`Ansi`] style
    #[inline]
    pub const fn ansi(&self) -> Ansi { self.ansi }
    /// Gets the target
    #[inline]
    pub const fn target(&self) -> &T { &self.target }
    /// Converts from `Styled<T>` (or `&Styled<T>`) to `Styled<&T::Target>`.
    ///
    /// Leaves the original `Styled` in-place, creating a new one with a reference to the
    /// original one's target, additionally coercing the target via [`Deref`].
    #[inline]
    pub fn as_deref(&self) -> Styled<&<T as Deref>::Target>
    where
        T: Deref,
        T::Target: fmt::Display,
    {
        Styled::new(self.ansi, self.target.deref())
    }

    /// Formats this instance to a [`StyledString`].
    #[inline]
    pub fn to_styled_string(&self) -> StyledString {
        StyledString::from(self)
    }
}

impl<T: fmt::Display> fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match ToStyledString::fmt_styled_begin(f, self.ansi)? {
            Some(to_styled_string) => {
                StyledDisplay::ToStyledString.fmt_styled(f, self)?;
                to_styled_string.fmt_styled_end(f)
            },
            None => StyledDisplay::Default.fmt_styled(f, self),
        }
    }
}
