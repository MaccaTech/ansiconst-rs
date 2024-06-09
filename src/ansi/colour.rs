use super::{Ansi, Attrs};
use std::fmt::Debug;

/// Represents the colour codes that are used to set foreground
/// and background colours on ANSI terminals.
///
/// To obtain a background colour, call [`.bg()`](Colour::bg()) on a colour.
///
/// Note: this enum is designed to be *immutable* and *const*
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Colour {
    /// No specific colour
    Unspecified,
    /// Reset colour to default
    Reset,
    /// Colour #0 (foreground code `30`, background code `40`).
    Black,
    /// Colour #1 (foreground code `31`, background code `41`).
    Red,
    /// Colour #2 (foreground code `32`, background code `42`).
    Green,
    /// Colour #3 (foreground code `33`, background code `43`).
    Yellow,
    /// Colour #4 (foreground code `34`, background code `44`).
    Blue,
    /// Colour #5 (foreground code `35`, background code `45`).
    Purple,
    /// Colour #6 (foreground code `36`, background code `46`).
    Cyan,
    /// Colour #7 (foreground code `37`, background code `47`).
    White,
    /// Colour #8 (foreground code `30`, background code `40`).
    BrightBlack,
    /// Colour #9 (foreground code `91`, background code `101`).
    BrightRed,
    /// Colour #10 (foreground code `92`, background code `102`).
    BrightGreen,
    /// Colour #11 (foreground code `93`, background code `103`).
    BrightYellow,
    /// Colour #12 (foreground code `94`, background code `104`).
    BrightBlue,
    /// Colour #13 (foreground code `95`, background code `105`).
    BrightPurple,
    /// Colour #14 (foreground code `96`, background code `106`).
    BrightCyan,
    /// Colour #15 (foreground code `97`, background code `107`).
    BrightWhite,
    /// An 8-bit colour, for use in 256-colour terminals.
    ///
    /// *Note: only available with `feature=ansi256` or `feature=rgb`*
    #[cfg(any(feature="ansi256", doc))]
    Ansi256(u8),
    /// A 24-bit RGB color, as specified by ISO-8613-3.
    ///
    /// *Note: only available with `feature=rgb`*
    #[cfg(any(feature="rgb", doc))]
    Rgb(u8, u8, u8),
}

impl Colour {
    /// True if this instance is unspecified - see [`Ansi::unspecified()`]
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        match self {
            Self::Unspecified => true,
            _                 => false,
        }
    }

    /// True if this instance is reset - see [`Ansi::reset()`]
    #[inline]
    pub const fn is_reset(&self) -> bool {
        match self {
            Self::Reset       => true,
            _                 => false,
        }
    }

    /// Used for resetting ANSI styles - see [`Ansi::not()`].
    ///
    /// Returns `Reset` if this is an actual colour, else `Unspecified`.
    ///
    /// ```
    /// use ansiconst::Colour::{Blue, Reset, Unspecified};
    ///
    /// assert_eq!( Blue.not(),       Reset);
    /// assert_eq!(Reset.not(), Unspecified);
    /// ```
    #[inline]
    pub const fn not(&self) -> Colour {
        match *self {
            Self::Unspecified => Self::Unspecified,
            Self::Reset       => Self::Unspecified,
            _                 => Self::Reset,
        }
    }

    /// Creates an [`Ansi`] style with this colour used as the foreground colour.
    #[inline]
    pub const fn fg(&self) -> Ansi {
        Ansi::from_colour(Colours::from_fg(*self))
    }

    /// Creates an [`Ansi`] style with this `Colour` used as the background colour.
    #[inline]
    pub const fn bg(&self) -> Ansi {
        Ansi::from_colour(Colours::from_bg(*self))
    }

    /// Creates an [`Ansi`] style with this `Colour` used as the foreground colour,
    /// and with [`Attrs::all()`] protected.
    #[inline]
    pub const fn only(&self) -> Ansi {
        Ansi::from_colour(Colours::from_fg(*self)).only()
    }

    /// Creates an [`Ansi`] style with this `Colour` used as the foreground colour,
    /// and with [`Attrs::Foreground`] protected.
    #[inline]
    pub const fn protect(&self) -> Ansi {
        Ansi::from_colour(Colours::from_fg(*self)).protect()
    }

    /// Used by the `styled_*!` macros to coerce a style argument to an [`Ansi`] instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi {
        Ansi::from_colour(Colours::from_fg(*self))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Colours { fg: Colour, bg: Colour }

impl Colours {
    #[inline]
    pub(crate) const fn is_unspecified(&self) -> bool { self.fg.is_unspecified() && self.bg.is_unspecified() }
    #[inline]
    pub(crate) const fn is_reset(&self) -> bool { self.fg.is_reset() && self.bg.is_reset() }
    #[inline]
    pub(crate) const fn unspecified() -> Self { Self::new(Colour::Unspecified, Colour::Unspecified) }
    #[inline]
    pub(crate) const fn reset() -> Self { Self::new(Colour::Reset, Colour::Reset) }
    #[inline]
    pub(crate) const fn new(fg: Colour, bg: Colour) -> Self { Self { fg, bg } }
    #[inline]
    pub(crate) const fn from_fg(fg: Colour) -> Self { Self { fg, bg: Colour::Unspecified } }
    #[inline]
    pub(crate) const fn from_bg(bg: Colour) -> Self { Self { fg: Colour::Unspecified, bg } }
    #[inline]
    pub(crate) const fn fg(&self) -> Colour { self.fg }
    #[inline]
    pub(crate) const fn bg(&self) -> Colour { self.bg }
    #[inline]
    pub(crate) const fn add(&self, other: Self) -> Self {
        Self {
            fg: if other.fg.is_unspecified() { self.fg } else { other.fg },
            bg: if other.bg.is_unspecified() { self.bg } else { other.bg },
        }
    }
    #[inline]
    pub(crate) const fn remove(&self, other: Self) -> Self {
        Self {
            fg: if other.fg.is_unspecified() { self.fg } else { Colour::Unspecified },
            bg: if other.bg.is_unspecified() { self.bg } else { Colour::Unspecified },
        }
    }
    #[inline]
    pub(crate) fn transition(&self, to_other: Self) -> Self {
        Self {
            fg: if to_other.fg.is_unspecified() { self.fg.not() } else if self.fg == to_other.fg { Colour::Unspecified } else { to_other.fg },
            bg: if to_other.bg.is_unspecified() { self.bg.not() } else if self.bg == to_other.bg { Colour::Unspecified } else { to_other.bg },
        }
    }
    #[inline]
    pub(crate) const fn not(&self) -> Self {
        Self {
            fg: self.fg.not(),
            bg: self.bg.not(),
        }
    }
    #[inline]
    pub(crate) const fn filter(&self, attrs: Attrs) -> Self {
        Self {
            fg: if attrs.intersects(Attrs::Foreground) { self.fg } else { Colour::Unspecified },
            bg: if attrs.intersects(Attrs::Background) { self.bg } else { Colour::Unspecified },
        }
    }
    #[inline]
    pub(crate) const fn attrs(&self) -> Attrs {
        let fg = if self.fg.is_unspecified() { Attrs::empty() } else { Attrs::Foreground };
        let bg = if self.bg.is_unspecified() { Attrs::empty() } else { Attrs::Background };
        fg.union(bg)
    }
}

impl From<&Colour> for Colours {
    fn from(fg: &Colour) -> Self { Self::from_fg(*fg) }
}
impl From<Colour> for Colours {
    fn from(fg: Colour) -> Self { Self::from_fg(fg) }
}
