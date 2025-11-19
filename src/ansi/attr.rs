use super::{color, effect};
use std::fmt;

/// An optimised struct that packs the following information into a u16,
/// by making the most efficient use of available bits:
///
/// 1. `no_ansi` flag
/// 2. `important` attrs
#[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
pub(super) struct Flags {
    bits: u16,
}

impl Flags {
    /// The below is an ugly way of ensuring **at compile time** that
    /// this const value does not conflict with the bits used by
    /// `effect::Attrs` and `color::Attrs`. I.e. there is at least
    /// one spare bit available.
    ///
    /// If there are no spare bits, compilation will fail with the following error:
    /// "attempt to shift left by `8_u32`, which would overflow"
    const NO_ANSI: u16 = Self::from_important(Attrs::new(
        effect::Attrs::empty(),
        color::Attrs::from_bits_retain(1 << color::Attrs::all().bits().count_ones())),
    ).bits;

    #[inline]
    pub(super) const fn empty() -> Self { Self { bits: 0 } }
    #[inline]
    pub(super) const fn no_ansi() -> Self { Self { bits: Self::NO_ANSI } }
    #[inline]
    pub(super) const fn is_no_ansi(&self) -> bool { self.bits == Self::NO_ANSI }
    #[inline]
    pub(super) const fn important(&self) -> Attrs {
        if self.is_no_ansi() { return Attrs::empty(); }

        Attrs {
            effect: effect::Attrs::from_bits_truncate((self.bits >> (8 * 0)) as u8),
            color:  color::Attrs::from_bits_truncate ((self.bits >> (8 * 1)) as u8),
        }
    }
    #[inline]
    pub(super) const fn from_important(attrs: Attrs) -> Self {
        Self {
            bits: (attrs.effect.bits() as u16) << (8 * 0)
                | (attrs.color .bits() as u16) << (8 * 1)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
pub(super) struct Attrs {
    pub(super) effect: effect::Attrs,
    pub(super) color:  color::Attrs,
}

impl Attrs {
    #[inline]
    pub(super) const fn empty() -> Self {
        Self { effect: effect::Attrs::empty(), color: color::Attrs::empty() }
    }
    #[inline]
    pub(super) const fn new(effect: effect::Attrs, color: color::Attrs) -> Self {
        Self { effect, color }
    }
    #[inline]
    pub(super) const fn union(&self, other: Attrs) -> Self {
        Self {
            effect: self.effect.union(other.effect),
            color: self.color.union(other.color),
        }
    }
    #[inline]
    pub(super) const fn difference(&self, other: Attrs) -> Self {
        Self {
            effect: self.effect.difference(other.effect),
            color: self.color.difference(other.color),
        }
    }
    #[inline]
    pub(super) const fn intersection(&self, other: Attrs) -> Self {
        Self {
            effect: self.effect.intersection(other.effect),
            color: self.color.intersection(other.color),
        }
    }
}
