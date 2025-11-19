use crate::{Color, Coloree, Toggle, ToggleColor};
use crate::write::{compile_time, run_time};
use crate::introspect::Attr;
use bitflags::bitflags;
use std::fmt;

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
    pub(super) struct Attrs: u8 {
        const Fg = 1 << 0;
        const Bg = 1 << 1;
    }
}

impl Attrs {
    #[inline]
    pub(super) const fn contains_coloree(&self, coloree: Coloree) -> bool {
        self.contains(Self::from_coloree(coloree))
    }

    #[inline]
    const fn from_coloree(coloree: Coloree) -> Self {
        match coloree {
            Coloree::Text       => Self::Fg,
            Coloree::Background => Self::Bg,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(super) struct Colors { fg: Option<ToggleColor>, bg: Option<ToggleColor> }

impl Colors {
    #[inline]
    pub(super) const fn from_color(color: Color, toggle: Toggle, coloree: Coloree) -> Self {
        let is_color = match toggle {
            Toggle::Reset => ToggleColor::Reset,
            Toggle::Set   => ToggleColor::Set(color),
        };
        Self::new(is_color, coloree)
    }

    #[inline]
    const fn new(color: ToggleColor, coloree: Coloree) -> Self {
        match coloree {
            Coloree::Text       => Self { fg: Some(color), bg: None        },
            Coloree::Background => Self { fg: None,        bg: Some(color) },
        }
    }
    #[inline]
    pub(super) const fn is_empty(&self) -> bool { self.fg.is_none() && self.bg.is_none() }
    #[inline]
    pub(super) const fn is_reset(&self) -> bool {
        match (self.fg, self.bg) {
            (Some(ToggleColor::Reset), Some(ToggleColor::Reset)) => true,
            _ => false,
        }
    }
    #[inline]
    pub(super) const fn empty() -> Self { Self { fg: None, bg: None } }
    #[inline]
    pub(super) const fn reset() -> Self { Self { fg: Some(ToggleColor::Reset), bg: Some(ToggleColor::Reset) } }
    #[inline]
    pub(super) const fn get_color(&self, coloree: Coloree) -> Option<Attr<Color>> {
        let is_color = match coloree {
            Coloree::Text       => self.fg,
            Coloree::Background => self.bg,
        };
        match is_color {
            None => None,
            Some(ToggleColor::Reset)  => Some(Attr::new_color(Color::Black, Toggle::Reset, coloree)),
            Some(ToggleColor::Set(c)) => Some(Attr::new_color(c, Toggle::Set, coloree)),
        }
    }
    #[inline]
    const fn not_fg(&self) -> Option<ToggleColor> {
        match self.fg {
            None | Some(ToggleColor::Reset) => None,
            _ => Some(ToggleColor::Reset),
        }
    }
    #[inline]
    const fn not_bg(&self) -> Option<ToggleColor> {
        match self.bg {
            None | Some(ToggleColor::Reset) => None,
            _ => Some(ToggleColor::Reset),
        }
    }
    #[inline]
    pub(super) const fn add(&self, other: Self) -> Self {
        Self {
            fg: if other.fg.is_none() { self.fg } else { other.fg },
            bg: if other.bg.is_none() { self.bg } else { other.bg },
        }
    }
    #[inline]
    pub(super) fn transition(&self, to_other: Self) -> Self {
        Self {
            fg: if to_other.fg.is_none() { self.not_fg() } else if self.fg == to_other.fg { None } else { to_other.fg },
            bg: if to_other.bg.is_none() { self.not_bg() } else if self.bg == to_other.bg { None } else { to_other.bg },
        }
    }
    #[inline]
    pub(super) const fn not(&self) -> Self {
        Self {
            fg: self.not_fg(),
            bg: self.not_bg(),
        }
    }
    #[inline]
    pub(super) const fn only(&self) -> Self {
        Self {
            fg: if self.fg.is_none() { Some(ToggleColor::Reset) } else { self.fg },
            bg: if self.bg.is_none() { Some(ToggleColor::Reset) } else { self.bg },
        }
    }
    #[inline]
    pub(super) const fn remove(&self, attrs: Attrs) -> Self {
        Self {
            fg: if attrs.intersects(Attrs::Fg) { None } else { self.fg },
            bg: if attrs.intersects(Attrs::Bg) { None } else { self.bg },
        }
    }
    #[inline]
    pub(super) const fn attrs(&self) -> Attrs {
        let fg = if self.fg.is_none() { Attrs::empty() } else { Attrs::Fg };
        let bg = if self.bg.is_none() { Attrs::empty() } else { Attrs::Bg };
        fg.union(bg)
    }

    #[inline]
    pub(super) fn write(&self, w: &mut run_time::Formatter<'_,'_>, toggle: Toggle) -> fmt::Result {
        match (toggle, self.fg) {
            (Toggle::Reset, Some(ToggleColor::Reset))  => w.write_color(Coloree::Text, ToggleColor::Reset)?,
            (Toggle::Set,   Some(ToggleColor::Set(c))) => w.write_color(Coloree::Text, ToggleColor::Set(c))?,
            _ => (),
        }
        match (toggle, self.bg) {
            (Toggle::Reset, Some(ToggleColor::Reset))  => w.write_color(Coloree::Background, ToggleColor::Reset)?,
            (Toggle::Set,   Some(ToggleColor::Set(c))) => w.write_color(Coloree::Background, ToggleColor::Set(c))?,
            _ => (),
        }
        Ok(())
    }
    #[inline]
    pub(super) const fn write_const(&self, mut w: compile_time::Writer, toggle: Toggle) -> compile_time::Writer {
        w = match (toggle, self.fg) {
            (Toggle::Reset, Some(ToggleColor::Reset))  => w.write_color(Coloree::Text, ToggleColor::Reset),
            (Toggle::Set,   Some(ToggleColor::Set(c))) => w.write_color(Coloree::Text, ToggleColor::Set(c)),
            _ => w,
        };
        w = match (toggle, self.bg) {
            (Toggle::Reset, Some(ToggleColor::Reset))  => w.write_color(Coloree::Background, ToggleColor::Reset),
            (Toggle::Set,   Some(ToggleColor::Set(c))) => w.write_color(Coloree::Background, ToggleColor::Set(c)),
            _ => w,
        };
        w
    }
}

impl From<&Color> for Colors {
    fn from(fg: &Color) -> Self { Self::from_color(*fg, Toggle::Set, Coloree::Text) }
}
impl From<Color> for Colors {
    fn from(fg: Color) -> Self { Self::from_color(fg, Toggle::Set, Coloree::Text) }
}
