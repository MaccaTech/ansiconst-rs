use crate::{Ansi, Toggle};
use crate::introspect::Attr;
use std::fmt;

/// Represents the control sequences, named Select Graphic Rendition (SGR),
/// that are used to enable various effects (e.g. italic) on ANSI terminals.
///
/// `Effect`s can be combined arbitrarily.
///
/// Note: this enum is designed to be *immutable* and *const*
#[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
#[non_exhaustive]
pub enum Effect {
    /// Effect with SGR parameter `1`
    Bold,
    /// Effect with SGR parameter `2`
    Faint,
    /// Effect with SGR parameter `3`
    Italic,
    /// Effect with SGR parameter `4`
    Underline,
    /// Effect with SGR parameter `5`
    Blink,
    /// Effect with SGR parameter `7`
    Reverse,
    /// Effect with SGR parameter `8`
    Hidden,
    /// Effect with SGR parameter `9`
    Strike,
}

impl Effect {
    const VARIANTS: &'static[Effect] = &[
        Self::Bold,
        Self::Faint,
        Self::Italic,
        Self::Underline,
        Self::Blink,
        Self::Reverse,
        Self::Hidden,
        Self::Strike,
    ];

    /// Get all `Effect`s, which facilitates iterating.
    pub const fn all() -> &'static[Effect] { Self::VARIANTS }

    /// Creates an [`Attr`] with this `Effect`
    #[inline]
    pub const fn attr(&self) -> Attr<Effect> { Attr::new_effect(*self, Toggle::Set) }

    /// Creates an [`Ansi`] style with the corresponding `reset` code for this `Effect`,
    /// as follows:
    ///
    /// | ANSI effect                      | SGR parameter |
    /// |----------------------------------|--------------:|
    /// | [`Bold`](Effect::Bold)           |          `22` |
    /// | [`Faint`](Effect::Faint)         |          `22` |
    /// | [`Italic`](Effect::Italic)       |          `23` |
    /// | [`Underline`](Effect::Underline) |          `24` |
    /// | [`Blink`](Effect::Blink)         |          `25` |
    /// | [`Reverse`](Effect::Reverse)     |          `27` |
    /// | [`Hidden`](Effect::Hidden)       |          `28` |
    /// | [`Strike`](Effect::Strike)       |          `29` |
    ///
    /// *Note: `Bold.not()` and `Faint.not()` have the same parameter*
    #[inline]
    pub const fn not(&self) -> Ansi { Ansi::from_effect(*self, Toggle::Reset) }

    /// Creates an [`Ansi`] style with this `Effect` set to [`only`](Ansi::only()).
    #[inline]
    pub const fn only(&self) -> Ansi { self.ansi().only() }

    /// Creates an [`Ansi`] style with this `Effect` set to [`important`](Ansi::important()).
    #[inline]
    pub const fn important(&self) -> Ansi { self.ansi().important() }

    /// Used by the `styled_*!` macros to coerce a style argument to an [`Ansi`] instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi { Ansi::from_effect(*self, Toggle::Set) }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.ansi(), f) }
}
