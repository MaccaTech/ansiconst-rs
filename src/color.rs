use crate::{Ansi, Toggle};
use crate::introspect::Attr;
use std::fmt;

/// Represents the control sequences, named Select Graphic Rendition (SGR),
/// that are used to set foreground and background colors on ANSI terminals.
///
/// To obtain a background color, call [`.bg()`](Color::bg) on a color,
/// or on [`reset`](Color::reset).
///
/// In addition to the standard set of 16 colors, [`extended colors`](Color::num)
/// and [`RGB colors`](Color::rgb) are also available.
///
/// Note: this enum is designed to be *immutable* and *const*
#[derive(Eq, Clone, Copy, fmt::Debug)]
#[non_exhaustive]
pub enum Color {
    /// Color #0 (SGR parameters: foreground `30`, background `40`).
    Black,
    /// Color #1 (SGR parameters: foreground `31`, background `41`).
    Red,
    /// Color #2 (SGR parameters: foreground `32`, background `42`).
    Green,
    /// Color #3 (SGR parameters: foreground `33`, background `43`).
    Yellow,
    /// Color #4 (SGR parameters: foreground `34`, background `44`).
    Blue,
    /// Color #5 (SGR parameters: foreground `35`, background `45`).
    Purple,
    /// Color #6 (SGR parameters: foreground `36`, background `46`).
    Cyan,
    /// Color #7 (SGR parameters: foreground `37`, background `47`).
    White,
    /// Color #8 (SGR parameters: foreground `30`, background `40`).
    BrightBlack,
    /// Color #9 (SGR parameters: foreground `91`, background `101`).
    BrightRed,
    /// Color #10 (SGR parameters: foreground `92`, background `102`).
    BrightGreen,
    /// Color #11 (SGR parameters: foreground `93`, background `103`).
    BrightYellow,
    /// Color #12 (SGR parameters: foreground `94`, background `104`).
    BrightBlue,
    /// Color #13 (SGR parameters: foreground `95`, background `105`).
    BrightPurple,
    /// Color #14 (SGR parameters: foreground `96`, background `106`).
    BrightCyan,
    /// Color #15 (SGR parameters: foreground `97`, background `107`).
    BrightWhite,
    /// See [`Color::num()`]
    #[doc(hidden)]
    #[cfg(feature="color256")]
    ColorNum(u8),
    /// See [`Color::rgb()`]
    #[doc(hidden)]
    #[cfg(feature="rgb")]
    Rgb(u8, u8, u8),
}

impl Color {
    /// Gets a [`ColorReset`] representing the `reset` color code.
    #[inline]
    pub const fn reset() -> ColorReset { ColorReset }

    /// An 8-bit color, for use in 256-color terminals.
    ///
    /// *Note: only available with `feature="color256"` or `feature="rgb"`*
    #[cfg(any(feature="color256", doc))]
    #[inline]
    pub const fn num(i: u8) -> Color { Self::ColorNum(i) }

    /// A 24-bit RGB color, as specified by ISO-8613-3.
    ///
    /// *Note: only available with `feature="rgb"`*
    #[cfg(any(feature="rgb", doc))]
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color { Self::Rgb(r,g,b) }

    /// Creates an [`Attr`] with `self` used as the foreground color.
    #[inline]
    pub const fn attr(&self) -> Attr<Color> { Attr::new(*self) }

    /// Creates an [`Ansi`] style with `self` used as the background color.
    #[inline]
    pub const fn bg(&self) -> Ansi {
        Ansi::from_color(*self, Toggle::Set, Coloree::Background)
    }

    /// Creates an [`Ansi`] style with `self` used as the foreground color
    /// set to [`only`](Ansi::only()).
    #[inline]
    pub const fn only(&self) -> Ansi { self.ansi().only() }

    /// Creates an [`Ansi`] style with `self` used as the foreground color
    /// set to [`important`](Ansi::important()).
    #[inline]
    pub const fn important(&self) -> Ansi { self.ansi().important() }

    /// Used by the `styled_*!` macros to coerce a style argument to an [`Ansi`] instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi {
        Ansi::from_color(*self, Toggle::Set, Coloree::Text)
    }

    #[inline]
    const fn _get_num_opt_unless_rgb(&self) -> Option<u8> {
        match self {
            Self::Black        => Some( 0),
            Self::Red          => Some( 1),
            Self::Green        => Some( 2),
            Self::Yellow       => Some( 3),
            Self::Blue         => Some( 4),
            Self::Purple       => Some( 5),
            Self::Cyan         => Some( 6),
            Self::White        => Some( 7),
            Self::BrightBlack  => Some( 8),
            Self::BrightRed    => Some( 9),
            Self::BrightGreen  => Some(10),
            Self::BrightYellow => Some(11),
            Self::BrightBlue   => Some(12),
            Self::BrightPurple => Some(13),
            Self::BrightCyan   => Some(14),
            Self::BrightWhite  => Some(15),
            #[cfg(feature="color256")]
            Self::ColorNum(n)  => Some(*n),
            #[cfg(feature="rgb")]
            Self::Rgb(_,_,_)   => None,
        }
    }

    /// Gets this instance's 8-bit color number.
    ///
    /// *Note: **not** available with `feature="rgb"`*
    #[cfg(any(not(feature="rgb"), doc))]
    #[inline]
    pub const fn get_num(&self) -> u8 {
        self._get_num_opt_unless_rgb().unwrap()
    }

    /// Gets this instance's 8-bit color number, or `None` if this is an RGB color
    /// outside the range of 8-bit colors.
    ///
    /// *Note: **only** available with `feature="rgb"`*
    #[cfg(any(feature="rgb", doc))]
    #[inline]
    pub const fn get_num_opt(&self) -> Option<u8> {
        match self {
            Self::Rgb(r, g, b) => Self::num_opt_from_rgb(*r, *g, *b),
            _ => self._get_num_opt_unless_rgb(),
        }
    }

    #[cfg(feature="rgb")]
    const fn num_opt_from_rgb(r: u8, g: u8, b: u8) -> Option<u8> {
        // 4-bit
        match (r, g, b) {
            (192, 192, 192) => return Some(7),
            (128, 128, 128) => return Some(8),
            (0 | 128, 0 | 128, 0 | 128) => return Some(
                0
                + if r == 0 { 0 } else { 1 }
                + if g == 0 { 0 } else { 2 }
                + if b == 0 { 0 } else { 4 }
            ),
            (0 | 255, 0 | 255, 0 | 255) => return Some(
                8
                + if r == 0 { 0 } else { 1 }
                + if g == 0 { 0 } else { 2 }
                + if b == 0 { 0 } else { 4 }
            ),
            _ => ()
        }

        // 8-bit colour
        match (r, g, b) {
            (0 | 95 | 135 | 175 | 215 | 255,
             0 | 95 | 135 | 175 | 215 | 255,
             0 | 95 | 135 | 175 | 215 | 255) => return Some(
                16
                + if r == 0 { 0 } else { ((r - 95) / 40 + 1) * 36 }
                + if g == 0 { 0 } else { ((g - 95) / 40 + 1) *  6 }
                + if b == 0 { 0 } else { ((b - 95) / 40 + 1) *  1 }
             ),
             _ => (),
        }

        // 8-bit grayscale
        if r == g
            && g == b
            && r >= 8
            && (r - 8) % 10 == 0 {
            let num = (r - 8) / 10;
            return match num {
                12 => Some(8),
                24 => None,
                _  => Some(num + 232),
            };
        }

        // Other
        None
    }

    const fn rgb_from_num(index: u8) -> (u8,u8,u8) {
        match index {
            0..=15 => { // 4-bit
                let level = match index {
                    9.. => 255,
                    7 => 192,
                    _ => 128,
                };
                let r = if index == 8 { 128 }
                else if (index & 1) != 0 { level }
                else { 0 };
                let g = if index == 8 { 128 }
                else if (index & 2) != 0 { level }
                else { 0 };
                let b = if index == 8 { 128 }
                else if (index & 4) != 0 { level }
                else { 0 };
                (r,g,b)
            },
            16..=231 => { // 8-bit colour
                let index = index - 16;
                let r_index = (index / 36) % 6;
                let g_index = (index / 6) % 6;
                let b_index = index % 6;
                let r = if r_index == 0 { 0 } else { 95 + 40 * (r_index-1) };
                let g = if g_index == 0 { 0 } else { 95 + 40 * (g_index-1) };
                let b = if b_index == 0 { 0 } else { 95 + 40 * (b_index-1) };
                (r,g,b)
            }
            _ => { // 8-bit grayscale
                let index = index - 232;
                let level = index * 10 + 8;
                (level, level, level)
            }
        }
    }

    /// Gets this instance's 24-bit RGB color components.
    #[cfg(any(not(feature="rgb"), doc))]
    #[inline]
    pub const fn get_rgb(&self) -> (u8,u8,u8) {
        Self::rgb_from_num(self.get_num())
    }

    #[doc(hidden)]
    #[cfg(feature="rgb")]
    #[inline]
    pub const fn get_rgb(&self) -> (u8,u8,u8) {
        match self {
            Self::Rgb(r,g,b) => (*r,*g,*b),
            _ => {
                let num = self._get_num_opt_unless_rgb().unwrap();
                Self::rgb_from_num(num)
            },
        }
    }
}

#[cfg(not(feature="rgb"))]
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.get_num() == other.get_num()
    }
}

#[cfg(feature="rgb")]
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (self._get_num_opt_unless_rgb(), other._get_num_opt_unless_rgb()) {
            (Some(a), Some(b)) => a == b,
            _ => {
                let a = self.get_rgb();
                let b = other.get_rgb();
                a.0 == b.0 && a.1 == b.1 && a.2 == b.2
            },
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.ansi(), f) }
}

/// Represents the `reset` [`Color`] code (SGR parameters: foreground 39, background 49)
pub struct ColorReset;

impl ColorReset {
    /// Creates an [`Ansi`] style with `reset` used as the background [`Color`].
    #[inline]
    pub const fn bg(&self) -> Ansi {
        Ansi::from_color(Color::Black, Toggle::Reset, Coloree::Background)
    }

    /// Creates an [`Ansi`] style with `reset` used as the foreground [`Color`]
    /// set to [`only`](Ansi::only()).
    #[inline]
    pub const fn only(&self) -> Ansi { self.ansi().only() }

    /// Creates an [`Ansi`] style with `reset` used as the foreground [`Color`]
    /// set to [`important`](Ansi::important()).
    #[inline]
    pub const fn important(&self) -> Ansi { self.ansi().important() }

    /// Used by the `styled_*!` macros to coerce a style argument to an [`Ansi`] instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi {
        Ansi::from_color(Color::Black, Toggle::Reset, Coloree::Text)
    }
}

impl fmt::Display for ColorReset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.ansi(), f) }
}

/// Represents the parts of an ANSI terminal that can have a color applied.
///
/// Note: this enum is designed to be *immutable* and *const*
#[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
#[non_exhaustive]
pub enum Coloree {
    /// The terminal foreground (i.e. the text)
    Text,
    /// The terminal background
    Background,
}

impl Coloree {
    const VARIANTS: &'static[Coloree] = &[Self::Text, Self::Background];

    /// Get all `Coloree`s, which facilitates iterating.
    pub const fn all() -> &'static[Coloree] { Self::VARIANTS }
}
