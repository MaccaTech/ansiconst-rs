//! Obtain the properties of an [`Ansi`].

mod color;
mod effect;
#[cfg(doc)]
use crate::Ansi;
use crate::{Color, Coloree, Effect, Toggle};
use bitflags::bitflags;
use std::fmt;

pub(crate) mod private {
    pub trait Seal : PartialEq + Eq + Clone + Copy {}
}

#[doc(hidden)]
pub trait Value : private::Seal {}
impl private::Seal for Color {}
impl private::Seal for Effect {}
impl Value for Color {}
impl Value for Effect {}

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
    struct AttrFlags: u8 {
        const Reset     = 1 << 0;
        const Important = 1 << 1;
        const Bg        = 1 << 2;
    }
}

/// Exposes the parameters of a [`Color`] or [`Effect`] attribute of [`Ansi`].
///
/// An attribute may have the following parameters:
///
/// 1. [`value`](Attr::value) - either a [`Color`] or an [`Effect`].
/// 2. [`is_reset`](Attr::is_reset) - determines if it is an explicit [`Color`] or [`Effect`]
/// or else the corresponding `reset` code.
/// 3. [`is_important`](Attr::is_important) - determines if it takes precedence
/// when the associated [`Ansi`] is combined with another instance.
/// 4. [`is_bg`](Attr::is_bg) - determines if it is the background [`Color`]
#[derive(Clone, Copy)]
pub struct Attr<V: Value> {
    value: V,
    flags: AttrFlags,
}

impl<V: Value> Attr<V> {
    /// Creates an instance using the specified value.
    #[inline]
    pub(super) const fn new(value: V) -> Self { Self { value, flags: AttrFlags::empty() } }

    /// Gets this attribute's value (i.e. a [`Color`] or an [`Effect`])
    #[inline]
    pub const fn value(&self) -> V { let value = self.value; value }

    /// True if this instance represents the `reset` code of the contained
    /// [`Color`] or [`Effect`] value.
    #[inline]
    pub const fn is_reset(&self) -> bool { self.flags.contains(AttrFlags::Reset) }

    /// Creates an instance representing the `reset` code of the contained
    /// [`Color`] or [`Effect`] value.
    #[inline]
    pub const fn reset(&self) -> Self {
        Self { value: self.value, flags: self.flags.union(AttrFlags::Reset) }
    }

    /// Toggles between a [`Color`] or [`Effect`] value and its corresponding
    /// [`reset`](Self::reset) value.
    #[inline]
    pub const fn not(&self) -> Self {
        let reset = if self.flags.intersects(AttrFlags::Reset) { AttrFlags::empty() } else { AttrFlags::Reset };
        let flags = self.flags.difference(AttrFlags::Reset).union(reset);
        Self { value: self.value, flags }
    }

    /// True if this attribute is set to [`important`](Ansi::important).
    #[inline]
    pub const fn is_important(&self) -> bool { self.flags.intersects(AttrFlags::Important) }

    /// Creates an instance of this attribute set to [`important`](Ansi::important).
    #[inline]
    pub const fn important(&self) -> Self {
        Self { value: self.value, flags: self.flags.union(AttrFlags::Important) }
    }

    /// Creates an instance of this attribute set to [`unimportant`](Ansi::unimportant).
    #[inline]
    pub const fn unimportant(&self) -> Self {
        Self { value: self.value, flags: self.flags.difference(AttrFlags::Important) }
    }

    /// Creates an instance of this attribute with [`important`](Ansi::important)
    /// set according to the `is_important` parameter.
    #[inline]
    pub const fn with_important(&self, is_important: bool) -> Self {
        if is_important { self.important() } else { self.unimportant() }
    }

    #[inline]
    pub(crate) const fn get_toggle(&self) -> Toggle {
        if self.flags.intersects(AttrFlags::Reset) { Toggle::Reset } else { Toggle::Set }
    }

    #[inline]
    const fn get_coloree(&self) -> Coloree {
        if self.flags.intersects(AttrFlags::Bg) { Coloree::Background } else { Coloree::Text }
    }
}
