use ansiconst::{Ansi, Color, Coloree::{self, *}, Effect::{self, *}};
use ansiconst::introspect::Attr;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Expect {
    NoAnsi,
    Ansi(ExpectAttrs),
}

impl Expect {
    fn empty() -> Self { Self::Ansi(ExpectAttrs::empty()) }

    pub const fn from_ansi(ansi: Ansi) -> Self {
        if ansi.is_no_ansi() { Self::NoAnsi }
        else                 { Self::Ansi(ExpectAttrs::from_ansi(ansi)) }
    }

    fn ansi(&self) -> Ansi {
        match self {
            Self::NoAnsi => Ansi::no_ansi(),
            Self::Ansi(attrs) => attrs.ansi(),
        }
    }

    pub fn is_no_ansi(&self) -> bool { self == &Self::NoAnsi }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::NoAnsi => false,
            Self::Ansi(attrs) => attrs.is_empty(),
        }
    }
    pub fn is_reset(&self) -> bool {
        match self {
            Self::NoAnsi => false,
            Self::Ansi(attrs) => attrs.is_reset(),
        }
    }

    pub fn important(&self) -> Self {
        match self {
            Self::NoAnsi => *self,
            Self::Ansi(attrs) => Self::Ansi(attrs.important()),
        }
    }

    pub fn not(&self) -> Self {
        match self {
            Self::NoAnsi => *self,
            Self::Ansi(attrs) => Self::Ansi(attrs.not()),
        }
    }

    pub fn add(&self, other: Self) -> Self {
        match (self, other) {
            (Self::NoAnsi, _) => *self,
            (_, Self::NoAnsi) => other,
            (Self::Ansi(a), Self::Ansi(b)) => Self::Ansi(a.add(b)),
        }
    }

    pub fn then(&self, other: Self) -> Self {
        match (self, other) {
            (Self::NoAnsi, _) => *self,
            (_, Self::NoAnsi) => other,
            (Self::Ansi(a), Self::Ansi(b)) => Self::Ansi(a.then(b)),
        }
    }

    pub fn transition(&self, to_other: Self) -> Self {
        match (self, to_other) {
            (Self::NoAnsi, _) => Self::empty(),
            (Self::Ansi(a), Self::NoAnsi) => Self::Ansi(a.not().unimportant()),
            (Self::Ansi(a), Self::Ansi(b)) => Self::Ansi(a.transition(b)),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ExpectAttrs {
    bold:      ExpectAttr,
    faint:     ExpectAttr,
    italic:    ExpectAttr,
    underline: ExpectAttr,
    blink:     ExpectAttr,
    reverse:   ExpectAttr,
    hidden:    ExpectAttr,
    strike:    ExpectAttr,
    fg:        ExpectAttr,
    bg:        ExpectAttr,
}

impl ExpectAttrs {
    const fn empty() -> Self {
        Self {
            bold:      ExpectAttr::None,
            faint:     ExpectAttr::None,
            italic:    ExpectAttr::None,
            underline: ExpectAttr::None,
            blink:     ExpectAttr::None,
            reverse:   ExpectAttr::None,
            hidden:    ExpectAttr::None,
            strike:    ExpectAttr::None,
            fg:        ExpectAttr::None,
            bg:        ExpectAttr::None,
        }
    }

    const fn from_ansi(ansi: Ansi) -> Self {
        Self {
            bold:      ExpectAttr::from_effect(ansi, Bold),
            faint:     ExpectAttr::from_effect(ansi, Faint),
            italic:    ExpectAttr::from_effect(ansi, Italic),
            underline: ExpectAttr::from_effect(ansi, Underline),
            blink:     ExpectAttr::from_effect(ansi, Blink),
            reverse:   ExpectAttr::from_effect(ansi, Reverse),
            hidden:    ExpectAttr::from_effect(ansi, Hidden),
            strike:    ExpectAttr::from_effect(ansi, Strike),
            fg:        ExpectAttr::from_color(ansi, Text),
            bg:        ExpectAttr::from_color(ansi, Background),
        }
    }

    fn ansi(&self) -> Ansi {
        let mut ansi: Ansi = Ansi::empty();
        if let ExpectAttr::Effect(attr) = self.bold      { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Effect(attr) = self.faint     { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Effect(attr) = self.italic    { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Effect(attr) = self.underline { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Effect(attr) = self.blink     { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Effect(attr) = self.reverse   { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Effect(attr) = self.hidden    { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Effect(attr) = self.strike    { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Color (attr) = self.fg        { ansi = ansi.add(attr.ansi()); }
        if let ExpectAttr::Color (attr) = self.bg        { ansi = ansi.add(attr.ansi()); }
        ansi
    }

    fn and(&self, f: impl Fn(ExpectAttr) -> bool) -> bool {
        f(self.bold)
        && f(self.faint)
        && f(self.italic)
        && f(self.underline)
        && f(self.blink)
        && f(self.reverse)
        && f(self.hidden)
        && f(self.strike)
        && f(self.fg)
        && f(self.bg)
    }

    fn map(&self, f: impl Fn(ExpectAttr) -> ExpectAttr) -> Self {
        Self {
            bold:      f(self.bold),
            faint:     f(self.faint),
            italic:    f(self.italic),
            underline: f(self.underline),
            blink:     f(self.blink),
            reverse:   f(self.reverse),
            hidden:    f(self.hidden),
            strike:    f(self.strike),
            fg:        f(self.fg),
            bg:        f(self.bg),
        }
    }

    fn merge(&self, other: Self, f: impl Fn(ExpectAttr, ExpectAttr) -> ExpectAttr) -> Self {
        Self {
            bold:      f(self.bold,      other.bold),
            faint:     f(self.faint,     other.faint),
            italic:    f(self.italic,    other.italic),
            underline: f(self.underline, other.underline),
            blink:     f(self.blink,     other.blink),
            reverse:   f(self.reverse,   other.reverse),
            hidden:    f(self.hidden,    other.hidden),
            strike:    f(self.strike,    other.strike),
            fg:        f(self.fg,        other.fg),
            bg:        f(self.bg,        other.bg),
        }
    }

    fn with_overlaps(&self) -> Self {
        let mut result = *self;
        if self.bold.is_reset() && self.faint.is_none() {
            result.faint = ExpectAttr::Effect(Faint.attr().not())
        } else if self.faint.is_reset() && self.bold.is_none() {
            result.bold = ExpectAttr::Effect(Bold.attr().not())
        }
        result
    }

    fn set_important(&self, is_important: bool) -> Self {
        self.map(|attr| attr.important(is_important))
    }

    fn   important(&self) -> Self { self.set_important(true)  }
    fn unimportant(&self) -> Self { self.set_important(false) }

    fn not(&self) -> Self {
        self.map(|attr| attr.not())
    }

    fn add(&self, other: Self) -> Self {
        self.merge(other, |a, b| {
            if !b.is_none() { b } else { a }
        })
    }

    fn then(&self, other: Self) -> Self {
        self.merge(other, |a, b| {
            if !b.is_none() && (b.is_important() || !a.is_important()) { b } else { a }
        })
    }

    fn transition(&self, to_other: Self) -> Self {
        if to_other.is_reset()
            && !self.is_reset() { return to_other.unimportant(); }

        let a = self.unimportant().with_overlaps();
        let b = to_other.unimportant();
        let mut result = a.merge(b, |a, b| {
            if a == b { ExpectAttr::None }
            else if !b.is_none() { b }
            else { a.not() }
        });

        // Handle Bold / Faint sharing same reset code
        let (is_reset_bold, is_reset_faint) = (
            result.bold.is_reset_opt(), result.faint.is_reset_opt()
        );
        match (is_reset_bold, is_reset_faint) {
            (Some(true), Some(true)) => {
                if b.bold.is_none() && !b.faint.is_none() {
                    result.bold = ExpectAttr::None;
                } else {
                    result.faint = ExpectAttr::None;
                }
            },
            (Some(true), None) => {
                if a.faint.is_set() && b.faint.is_set() {
                    result.faint = b.faint
                }
            },
            (None, Some(true)) => {
                if a.bold.is_set() && b.bold.is_set() {
                    result.bold = b.bold
                }
            },
            _ => (),
        };

        result
    }

    fn is_empty(&self) -> bool { self.and(|attr| attr.is_none()) }
    fn is_reset(&self) -> bool { self.and(|attr| attr.is_reset()) }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ExpectAttr {
    None,
    Color(Attr<Color>),
    Effect(Attr<Effect>),
}

impl ExpectAttr {
    fn not(&self) -> Self {
        match self {
            Self::Color(attr) => {
                if !attr.is_reset() {
                    Self::Color(attr.not())
                } else {
                    Self::None
                }
            },
            Self::Effect(attr) => {
                if !attr.is_reset() {
                    Self::Effect(attr.not())
                } else {
                    Self::None
                }
            },
            _ => Self::None,
        }
    }

    fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _          => false
        }
    }

    fn is_reset(&self) -> bool {
        self.is_reset_opt().unwrap_or(false)
    }

    fn is_reset_opt(&self) -> Option<bool> {
        match self {
            Self::Color (attr) => Some(attr.is_reset()),
            Self::Effect(attr) => Some(attr.is_reset()),
            Self::None         => None,
        }
    }

    fn is_set(&self) -> bool { !self.is_none() && !self.is_reset() }

    fn important(&self, is_important: bool) -> Self {
        match self {
            Self::Color (attr) => Self::Color (attr.with_important(is_important)),
            Self::Effect(attr) => Self::Effect(attr.with_important(is_important)),
            Self::None         => *self,
        }
    }

    fn is_important(&self) -> bool {
        match *self {
            Self::Color (attr) => attr.is_important(),
            Self::Effect(attr) => attr.is_important(),
            Self::None         => false,
        }
    }

    const fn from_effect(ansi: Ansi, effect: Effect) -> Self {
        match ansi.get_effect(effect) {
            Some(attr) => Self::Effect(attr),
            None       => Self::None,
        }
    }

    const fn from_color(ansi: Ansi, color: Coloree) -> Self {
        match ansi.get_color(color) {
            Some(attr) => Self::Color(attr),
            None       => Self::None,
        }
    }
}

impl fmt::Display for Expect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.ansi(), f) }

}
impl fmt::Debug for Expect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&self.ansi(), f) }
}

impl fmt::Debug for ExpectAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Color (attr) => write!(f, "{:?}", attr),
            Self::Effect(attr) => write!(f, "{:?}", attr),
            Self::None         => Ok(())
        }
    }
}
