use crate::{Ansi, Effect, Toggle};
use super::{Attr, AttrFlags};
use std::fmt;

impl Attr<Effect> {
    #[inline]
    pub(crate) const fn new_effect(effect: Effect, toggle: Toggle) -> Self {
        let flags = match toggle {
            Toggle::Reset => AttrFlags::Reset,
            Toggle::Set   => AttrFlags::empty(),
        };
        Self { value: effect, flags }
    }

    /// Used by the `styled_*!` macros to coerce a style argument to an [`Ansi`] instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi {
        let ansi = Ansi::from_effect(self.value, self.get_toggle());
        if self.is_important() { ansi.important() } else { ansi }
    }
}

impl PartialEq for Attr<Effect> {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
        && self.value == other.value
    }
}

impl Eq for Attr<Effect> {}

impl fmt::Display for Attr<Effect> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.ansi(), f) }
}

impl fmt::Debug for Attr<Effect> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value())?;
        if self.is_reset()     { write!(f, ".not()")?; }
        if self.is_important() { write!(f, ".important()")?; }
        Ok(())
    }
}
