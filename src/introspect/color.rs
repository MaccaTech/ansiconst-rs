use crate::{Ansi, Color, Coloree, Toggle};
use super::{Attr, AttrFlags};
use std::fmt;

impl Attr<Color> {
    #[inline]
    pub(crate) const fn new_color(color: Color, toggle: Toggle, coloree: Coloree) -> Self {
        let mut flags = AttrFlags::empty();
        match toggle {
            Toggle::Reset => flags = flags.union(AttrFlags::Reset),
            Toggle::Set   => (),
        }
        match coloree {
            Coloree::Background => flags = flags.union(AttrFlags::Bg),
            Coloree::Text       => (),
        }
        Self { value: color, flags }
    }

    /// Creates an instance with this attribute's [`Color`] value as the foreground color.
    #[inline]
    pub const fn fg(&self) -> Self {
        Self { value: self.value, flags: self.flags.difference(AttrFlags::Bg) }
    }

    /// Creates an instance with this attribute's [`Color`] value as the background color.
    #[inline]
    pub const fn bg(&self) -> Self {
        Self { value: self.value, flags: self.flags.union(AttrFlags::Bg) }
    }

    /// True if this [`Color`] attribute is the background color.
    #[inline]
    pub const fn is_bg(&self) -> bool { self.flags.intersects(AttrFlags::Bg) }

    /// Used by the `styled_*!` macros to coerce a style argument to an [`Ansi`] instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi {
        let ansi = Ansi::from_color(self.value, self.get_toggle(), self.get_coloree());
        if self.is_important() { ansi.important() } else { ansi }
    }
}

impl PartialEq for Attr<Color> {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
        && (self.flags.intersects(AttrFlags::Reset)
            || self.value == other.value)
    }
}

impl Eq for Attr<Color>  {}

impl fmt::Display for Attr<Color> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.ansi(), f) }
}

impl fmt::Debug for Attr<Color> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_reset()     { write!(f, "Color::reset()")?; }
        else                   { write!(f, "{:?}", self.value())?; }
        if self.is_bg()        { write!(f, ".bg()")?; }
        if self.is_important() { write!(f, ".important()")?; }
        Ok(())
    }
}
