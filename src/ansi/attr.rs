use bitflags::bitflags;
use std::fmt;

bitflags! {
    /// A bitmask used to select an arbitrary combination of [`Ansi`](crate::Ansi) attributes.
    ///
    /// See [`Ansi::filter()`](crate::Ansi::filter()) and [`Ansi::protect_attrs()`](crate::Ansi::protect_attrs()).
    #[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
    pub struct Attrs: u16 {
        /// Matches ANSI effects [`Bold`](crate::Effect::Bold) and [`NotBold`](crate::Effect::NotBold)
        const Bold       = 1 << 0;
        /// Matches ANSI effects [`Faint`](crate::Effect::Faint) and [`NotFaint`](crate::Effect::NotFaint)
        const Faint      = 1 << 1;
        /// Matches ANSI effects [`Italic`](crate::Effect::Italic) and [`NotItalic`](crate::Effect::NotItalic)
        const Italic     = 1 << 2;
        /// Matches ANSI effects [`Underline`](crate::Effect::Underline) and [`NotUnderline`](crate::Effect::NotUnderline)
        const Underline  = 1 << 3;
        /// Matches ANSI effects [`Blink`](crate::Effect::Blink) and [`NotBlink`](crate::Effect::NotBlink)
        const Blink      = 1 << 4;
        /// Matches ANSI effects [`Reverse`](crate::Effect::Reverse) and [`NotReverse`](crate::Effect::NotReverse)
        const Reverse    = 1 << 5;
        /// Matches ANSI effects [`Hidden`](crate::Effect::Hidden) and [`NotHidden`](crate::Effect::NotHidden)
        const Hidden     = 1 << 6;
        /// Matches ANSI effects [`Strike`](crate::Effect::Strike) and [`NotStrike`](crate::Effect::NotStrike)
        const Strike     = 1 << 7;
        /// Matches ANSI *foreground* [`Colour`](crate::Colour)
        const Foreground = 1 << 8;
        /// Matches ANSI *background* [`Colour`](crate::Colour)
        const Background = 1 << 9;
    }
}

impl Attrs {
    /// Gets the `Attr` corresponding to both foreground and background [`Colour`](crate::Colour)s
    #[inline]
    pub const fn colours() -> Self { Self::Foreground.union(Self::Background) }
    /// Gets the `Attr` corresponding to all [`Effect`](crate::Effect)s
    #[inline]
    pub const fn effects() -> Self { Self::Bold.union(Self::Faint).union(Self::Italic).union(Self::Underline).union(Self::Blink).union(Self::Reverse).union(Self::Hidden).union(Self::Strike) }
}
