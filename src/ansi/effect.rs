use super::{Ansi, Attrs};
use bitflags::bitflags;
use std::fmt::Debug;

/// Represents the control sequences, named Select Graphic Rendition (SGR),
/// that are used to enable various effects (e.g. italic) on ANSI terminals.
///
/// `Effect`s can be combined arbitrarily.
///
/// Note: this enum is designed to be *immutable* and *const*
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Effect {
    /// No specific effect
    Unspecified,
    /// Effect with SGR attribute code `1`
    Bold,
    /// Reset with SGR attribute code `22` (note: same as `NotFaint`)
    NotBold,
    /// Effect with SGR attribute code `2`
    Faint,
    /// Reset with SGR attribute code `22` (note: same as `NotBold`)
    NotFaint,
    /// Effect with SGR attribute code `3`
    Italic,
    /// Reset with SGR attribute code `23`
    NotItalic,
    /// Effect with SGR attribute code `4`
    Underline,
    /// Reset with SGR attribute code `24`
    NotUnderline,
    /// Effect with SGR attribute code `5`
    Blink,
    /// Reset with SGR attribute code `25`
    NotBlink,
    /// Effect with SGR attribute code `7`
    Reverse,
    /// Reset with SGR attribute code `27`
    NotReverse,
    /// Effect with SGR attribute code `8`
    Hidden,
    /// Reset with SGR attribute code `28`
    NotHidden,
    /// Effect with SGR attribute code `9`
    Strike,
    /// Reset with SGR attribute code `29`
    NotStrike,
}

impl Effect {
    /// True if this instance is unspecified - see [`Ansi::unspecified()`]
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        match self {
            Self::Unspecified  => true,
            _                  => false,
        }
    }

    /// True if this instance is reset - see [`Ansi::reset()`]
    #[inline]
    pub const fn is_reset(&self) -> bool {
        match *self {
            Self::NotBold      => true,
            Self::NotFaint     => true,
            Self::NotItalic    => true,
            Self::NotUnderline => true,
            Self::NotBlink     => true,
            Self::NotReverse   => true,
            Self::NotHidden    => true,
            Self::NotStrike    => true,
            _                  => false,
        }
    }

    /// Used for resetting ANSI styles - see [`Ansi::not()`].
    ///
    /// Returns the `Not*` equivalent of this effect if this is an actual effect,
    /// else `Unspecified`.
    ///
    /// ```
    /// use ansiconst::Effect::{Bold, NotBold, Unspecified};
    ///
    /// assert_eq!(   Bold.not(),     NotBold);
    /// assert_eq!(NotBold.not(), Unspecified);
    /// ```
    #[inline]
    pub const fn not(&self) -> Effect {
        match *self {
            Self::Bold         => Self::NotBold,
            Self::Faint        => Self::NotFaint,
            Self::Italic       => Self::NotItalic,
            Self::Underline    => Self::NotUnderline,
            Self::Blink        => Self::NotBlink,
            Self::Reverse      => Self::NotReverse,
            Self::Hidden       => Self::NotHidden,
            Self::Strike       => Self::NotStrike,
            _                  => Self::Unspecified,
        }
    }

    /// Creates an [`Ansi`] style with this `Effect` and with [`Attrs::all()`] protected.
    #[inline]
    pub const fn only(&self) -> Ansi { self.ansi().only() }

    /// Used by the `styled_*!` macros to coerce a style argument to an [`Ansi`] instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi {
        Ansi::from_effect(self.as_effects())
    }

    #[inline]
    pub(crate) const fn as_effects(&self) -> Effects {
        if !self.is_reset() {
            Effects { y: self.to_bits(), n: Bits::empty() }
        } else {
            Effects { n: self.to_bits(), y: Bits::empty() }
        }
    }
    #[inline]
    const fn to_bits(&self) -> Bits {
        match *self {
            Self::Unspecified  => Bits::empty(),
            Self::Bold         => Bits::Bold,
            Self::NotBold      => Bits::Bold,
            Self::Faint        => Bits::Faint,
            Self::NotFaint     => Bits::Faint,
            Self::Italic       => Bits::Italic,
            Self::NotItalic    => Bits::Italic,
            Self::Underline    => Bits::Underline,
            Self::NotUnderline => Bits::Underline,
            Self::Blink        => Bits::Blink,
            Self::NotBlink     => Bits::Blink,
            Self::Reverse      => Bits::Reverse,
            Self::NotReverse   => Bits::Reverse,
            Self::Hidden       => Bits::Hidden,
            Self::NotHidden    => Bits::Hidden,
            Self::Strike       => Bits::Strike,
            Self::NotStrike    => Bits::Strike,
        }
    }
}

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    struct Bits: u8 {
        const Bold      = 1 << 0;
        const Faint     = 1 << 1;
        const Italic    = 1 << 2;
        const Underline = 1 << 3;
        const Blink     = 1 << 4;
        const Reverse   = 1 << 5;
        const Hidden    = 1 << 6;
        const Strike    = 1 << 7;
    }
}

impl Bits {
    const fn to_not_bitmask(&self) -> Self {
        if self.intersects(Bits::Bold) {
            self.union(Bits::Faint)
        } else if self.intersects(Bits::Faint) {
            self.union(Bits::Bold)
        } else {
            *self
        }
    }
    const fn filter(&self, attrs: Attrs) -> Self {
        self.intersection(Self::from_attrs(attrs))
    }
    const fn from_attrs(attrs: Attrs) -> Self {
        let mut bits = Self::empty();
        if attrs.intersects(Attrs::Bold)      { bits = bits.union(Self::Bold); }
        if attrs.intersects(Attrs::Faint)     { bits = bits.union(Self::Faint); }
        if attrs.intersects(Attrs::Italic)    { bits = bits.union(Self::Italic); }
        if attrs.intersects(Attrs::Underline) { bits = bits.union(Self::Underline); }
        if attrs.intersects(Attrs::Blink)     { bits = bits.union(Self::Blink); }
        if attrs.intersects(Attrs::Reverse)   { bits = bits.union(Self::Reverse); }
        if attrs.intersects(Attrs::Hidden)    { bits = bits.union(Self::Hidden); }
        if attrs.intersects(Attrs::Strike)    { bits = bits.union(Self::Strike); }
        bits
    }
    const fn to_attrs(&self) -> Attrs {
        let mut attrs = Attrs::empty();
        if self.intersects(Self::Bold)        { attrs = attrs.union(Attrs::Bold); }
        if self.intersects(Self::Faint)       { attrs = attrs.union(Attrs::Faint); }
        if self.intersects(Self::Italic)      { attrs = attrs.union(Attrs::Italic); }
        if self.intersects(Self::Underline)   { attrs = attrs.union(Attrs::Underline); }
        if self.intersects(Self::Blink)       { attrs = attrs.union(Attrs::Blink); }
        if self.intersects(Self::Reverse)     { attrs = attrs.union(Attrs::Reverse); }
        if self.intersects(Self::Hidden)      { attrs = attrs.union(Attrs::Hidden); }
        if self.intersects(Self::Strike)      { attrs = attrs.union(Attrs::Strike); }
        attrs
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Effects { y: Bits, n: Bits }

impl Effects {
    #[inline]
    pub(crate) const fn is_unspecified(&self) -> bool { self.y.is_empty() && self.n.is_empty() }
    #[inline]
    pub(crate) const fn is_reset(&self) -> bool { self.y.is_empty() && self.n.is_all() }
    #[inline]
    pub(crate) const fn unspecified() -> Self{ Effects { y: Bits::empty(), n: Bits::empty() } }
    #[inline]
    pub(crate) const fn reset() -> Self{ Effects { y: Bits::empty(), n: Bits::all() } }
    #[cfg(test)]
    const fn all() -> Self{ Effects { y: Bits::all(), n: Bits::empty() } }
    #[inline]
    pub(crate) const fn has_effect(&self, ef: Effect) -> bool {
        if !ef.is_reset() {
            self.y.intersects(ef.to_bits())
        } else {
            self.n.intersects(ef.to_bits())
        }
    }
    #[inline]
    pub(crate) const fn add(&self, other: Self) -> Self {
        self.union(other)
    }
    #[inline]
    pub(crate) const fn remove(&self, other: Self) -> Self {
        self.difference(other)
    }
    #[inline]
    pub(crate) const fn transition(&self, to_other: Self) -> Self {
        let remove = self.difference(to_other).not();
        let add = to_other.difference(*self).union(to_other.intersection(remove));
        remove.union(add)
    }
    #[inline]
    pub(crate) const fn not(&self) -> Self {
        Self {
            y: Bits::empty(),
            n: self.y
        }
    }
    #[inline]
    pub(crate) const fn filter(&self, attrs: Attrs) -> Self {
        Self {
            y: self.y.filter(attrs),
            n: self.n.filter(attrs),
        }
    }
    #[inline]
    pub(crate) const fn attrs(&self) -> Attrs {
        self.y.to_attrs().union(self.n.to_attrs())
    }
    #[inline]
    const fn union(&self, other: Self) -> Self {
        Self {
            y: self.y.difference(other.n).union(other.y.difference(self.n)),
            n: self.n.difference(other.y).union(other.n.difference(self.y)),
        }
    }
    #[inline]
    const fn difference(&self, other: Self) -> Self {
        let other_bitmask = other.y.union(other.n.to_not_bitmask());
        Self {
            y: self.y.difference(other_bitmask),
            n: self.n.difference(other_bitmask),
        }
    }
    #[inline]
    const fn intersection(&self, other: Self) -> Self {
        let other_bitmask: Bits = other.y.union(other.n.to_not_bitmask());
        Self {
            y: self.y.intersection(other_bitmask),
            n: self.n.intersection(other_bitmask),
        }
    }
    #[cfg(test)]
    const fn intersects(&self, other: Self) -> bool {
        let other_bitmask = other.y.union(other.n.to_not_bitmask());
        self.y.intersects(other_bitmask) ||
        self.n.intersects(other_bitmask)
    }
    #[cfg(test)]
    fn iter(&self) -> Iter { Iter { y: self.y.iter(), n: self.n.iter() } }
}

impl From<&Effect> for Effects {
    fn from(ef: &Effect) -> Self { ef.as_effects() }
}
impl From<Effect> for Effects {
    fn from(ef: Effect) -> Self { ef.as_effects() }
}

#[cfg(test)]
struct Iter { y: bitflags::iter::Iter<Bits>, n: bitflags::iter::Iter<Bits> }

#[cfg(test)]
impl Iterator for Iter {
    type Item = Effects;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.y.next() {
            Some(Effects { y, n: Bits::empty() })
        } else if let Some(n) = self.n.next() {
            Some(Effects { y: Bits::empty(), n })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Effect::{Bold, Faint};

    fn check_same_effects(a: Effects, b: Effects) {
        assert_eq!(a.union(b),                     a);
        assert_eq!(a.union(b.not()),               Effects::unspecified());
        assert_eq!(a.difference(b),                Effects::unspecified());
        assert_eq!(a.difference(b.not()),          Effects::unspecified());
        assert_eq!(a.intersection(b),              a);
        assert_eq!(a.intersection(b.not()),        a);
        assert_eq!(a.intersects(b),                true);
        assert_eq!(a.intersects(b.not()),          true);
    }

    fn check_diff_effects(a: Effects, b: Effects) {
        let is_bold_faint_pair = (a.has_effect(Bold) || a.has_effect(Faint))
                                    && (b.has_effect(Bold) || b.has_effect(Faint));
        assert_eq!(a.difference(b),                a);
        assert_eq!(a.difference(b.not()),          if is_bold_faint_pair { Effects::unspecified() } else { a });
        assert_eq!(a.intersection(b),              Effects::unspecified());
        assert_eq!(a.intersection(b.not()),        if is_bold_faint_pair { a } else { Effects::unspecified() });
        assert_eq!(a.intersects(b),                false);
        assert_eq!(a.intersects(b.not()),          is_bold_faint_pair);

        let both = a.union(b);
        assert_eq!(both.union(b),                  both);
        assert_eq!(both.union(b.not()),            a);
        assert_eq!(b.union(both),                  both);
        assert_eq!(b.union(both.not()),            a.not());
        assert_eq!(both.difference(b),             a);
        assert_eq!(both.difference(b.not()),       if is_bold_faint_pair { Effects::unspecified() } else { a });
        assert_eq!(b.difference(both),             Effects::unspecified());
        assert_eq!(b.difference(both.not()),       Effects::unspecified());
        assert_eq!(both.intersection(b),           b);
        assert_eq!(both.intersection(b.not()),     if is_bold_faint_pair { both } else { b });
        assert_eq!(b.intersection(both),           b);
        assert_eq!(b.intersection(both.not()),     b);
        assert_eq!(both.intersects(b),             true);
        assert_eq!(both.intersects(b.not()),       true);
        assert_eq!(b.intersects(both),             true);
        assert_eq!(b.intersects(both.not()),       true);
    }

    #[test]
    fn test_effect() {
        for a in Effects::all().iter() {
            for b in Effects::all().iter() {
                println!("{:?} {:?}", a.y, b.y);
                if a == b {
                    check_same_effects(a, b);
                } else {
                    check_diff_effects(a, b);
                }
            }
        }
    }
}
