use crate::{Effect, Toggle};
use crate::write::{compile_time, run_time};
use crate::introspect::Attr;
use bitflags::bitflags;
use std::fmt;

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
    pub(super) struct Attrs: u8 {
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

impl Attrs {
    #[inline]
    pub(super) const fn contains_effect(&self, effect: Effect) -> bool {
        self.contains(Self::from_effect(effect))
    }

    #[inline]
    const fn from_effect(effect: Effect) -> Self {
        match effect {
            Effect::Bold      => Self::Bold,
            Effect::Faint     => Self::Faint,
            Effect::Italic    => Self::Italic,
            Effect::Underline => Self::Underline,
            Effect::Blink     => Self::Blink,
            Effect::Reverse   => Self::Reverse,
            Effect::Hidden    => Self::Hidden,
            Effect::Strike    => Self::Strike,
        }
    }

    /// Includes other attributes that are also reset when self's `reset` ANSI codes are applied.
    #[inline]
    pub(super) const fn with_overlaps(&self) -> Self {
        if self.intersects(Attrs::Bold) {
            self.union(Attrs::Faint)
        } else if self.intersects(Attrs::Faint) {
            self.union(Attrs::Bold)
        } else {
            *self
        }
    }

    /// Excludes other attributes that are also reset when self's `reset` ANSI codes are applied.
    #[inline]
    pub(super) const fn no_overlaps(&self) -> Self {
        if self.contains(Attrs::Bold.union(Attrs::Faint)) {
            self.difference(Attrs::Faint)
        } else {
            *self
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(super) struct Effects { y: Attrs, n: Attrs }

impl Effects {
    #[inline]
    pub(super) const fn from_effect(effect: Effect, toggle: Toggle) -> Self {
        Self::new(Attrs::from_effect(effect), toggle)
    }

    #[inline]
    const fn new(attrs: Attrs, toggle: Toggle) -> Self {
        match toggle {
            Toggle::Set   => Self { y: attrs,          n: Attrs::empty() },
            Toggle::Reset => Self { y: Attrs::empty(), n: attrs          },
        }
    }
    #[inline]
    pub(super) const fn is_empty(&self) -> bool { self.y.is_empty() && self.n.is_empty() }
    #[inline]
    pub(super) const fn is_reset(&self) -> bool { self.y.is_empty() && self.n.is_all() }
    #[inline]
    pub(super) const fn empty() -> Self{ Effects { y: Attrs::empty(), n: Attrs::empty() } }
    #[inline]
    pub(super) const fn reset() -> Self{ Effects { y: Attrs::empty(), n: Attrs::all() } }
    #[inline]
    pub(super) const fn get_effect(&self, ef: Effect) -> Option<Attr<Effect>> {
        let attrs = Attrs::from_effect(ef);
        if self.y.contains(attrs) {
            Some(Attr::new_effect(ef, Toggle::Set))
        } else if self.n.contains(attrs) {
            Some(Attr::new_effect(ef, Toggle::Reset))
        } else {
            None
        }
    }
    #[inline]
    pub(super) const fn add(&self, other: Self) -> Self {
        let other_attrs = other.attrs();
        Self {
            y: self.y.difference(other_attrs).union(other.y),
            n: self.n.difference(other_attrs).union(other.n),
        }
    }
    #[inline]
    pub(super) const fn transition(&self, to_other: Self) -> Self {
        // 1. Include other's non-overlapping .n
        let other_new_n = to_other.n.difference(self.n.with_overlaps());
        // 2. Include resets for self's non-overlapping .y
        let self_kill_y = self.y.difference(to_other.y).difference(other_new_n.with_overlaps());
        // 3. Restore other's .y that were indirectly reset by #1 & #2
        let other_restore_y = to_other.y.intersection(self_kill_y.with_overlaps().union(other_new_n.with_overlaps()));
        // 4. Combine other's non-overlapping .y and #1, #2, #3
        let y = to_other.y.difference(self.y).union(other_restore_y);
        let n = other_new_n.union(self_kill_y).no_overlaps();
        #[cfg(test)]
        assert!(!y.intersects(n));
        Self { y, n }
    }
    #[inline]
    pub(super) const fn not(&self) -> Self {
        Self {
            y: Attrs::empty(),
            n: self.y,
        }
    }
    #[inline]
    pub(super) const fn only(&self) -> Self {
        Self {
            y: self.y,
            n: self.n.union(self.y.complement()),
        }
    }
    #[inline]
    pub(super) const fn remove(&self, attrs: Attrs) -> Self {
        Self {
            y: self.y.difference(attrs),
            n: self.n.difference(attrs),
        }
    }
    #[inline]
    pub(super) const fn attrs(&self) -> Attrs {
        self.y.union(self.n)
    }

    #[inline]
    pub(super) fn write(&self, w: &mut run_time::Formatter<'_,'_>, toggle: Toggle) -> fmt::Result {
        let attrs = match toggle {
            Toggle::Set   => self.y,
            Toggle::Reset => self.n,
        };
        if attrs.contains(Attrs::Bold     ) { w.write_effect(Effect::Bold,      toggle)?; }
        if attrs.contains(Attrs::Faint    ) { w.write_effect(Effect::Faint,     toggle)?; }
        if attrs.contains(Attrs::Italic   ) { w.write_effect(Effect::Italic,    toggle)?; }
        if attrs.contains(Attrs::Underline) { w.write_effect(Effect::Underline, toggle)?; }
        if attrs.contains(Attrs::Blink    ) { w.write_effect(Effect::Blink,     toggle)?; }
        if attrs.contains(Attrs::Reverse  ) { w.write_effect(Effect::Reverse,   toggle)?; }
        if attrs.contains(Attrs::Hidden   ) { w.write_effect(Effect::Hidden,    toggle)?; }
        if attrs.contains(Attrs::Strike   ) { w.write_effect(Effect::Strike,    toggle)?; }
        Ok(())
    }
    #[inline]
    pub(super) const fn write_const(&self, mut w: compile_time::Writer, toggle: Toggle) -> compile_time::Writer {
        let attrs = match toggle {
            Toggle::Set   => self.y,
            Toggle::Reset => self.n,
        };
        if attrs.contains(Attrs::Bold     ) { w = w.write_effect(Effect::Bold,      toggle); }
        if attrs.contains(Attrs::Faint    ) { w = w.write_effect(Effect::Faint,     toggle); }
        if attrs.contains(Attrs::Italic   ) { w = w.write_effect(Effect::Italic,    toggle); }
        if attrs.contains(Attrs::Underline) { w = w.write_effect(Effect::Underline, toggle); }
        if attrs.contains(Attrs::Blink    ) { w = w.write_effect(Effect::Blink,     toggle); }
        if attrs.contains(Attrs::Reverse  ) { w = w.write_effect(Effect::Reverse,   toggle); }
        if attrs.contains(Attrs::Hidden   ) { w = w.write_effect(Effect::Hidden,    toggle); }
        if attrs.contains(Attrs::Strike   ) { w = w.write_effect(Effect::Strike,    toggle); }
        w
    }
}

impl From<&Effect> for Effects {
    fn from(ef: &Effect) -> Self { Self::from_effect(*ef, Toggle::Set) }
}
impl From<Effect> for Effects {
    fn from(ef: Effect) -> Self { Self::from_effect(ef, Toggle::Set) }
}
