use crate::{Ansi, Color::*, Coloree::{self, *}, Effect::{self, *}, Toggle, ToggleColor};
use std::fmt;

enum State { Clean, Dirty }

impl State {
    #[inline]
    fn write_str<W: fmt::Write>(&self, w: &mut W, s: &str) -> Result<Self, fmt::Error> {
        self._write_separator(w)?;
        w.write_str(s)?;
        Ok(Self::Dirty)
    }
    #[cfg(feature="color256")]
    #[inline]
    fn write_fmt<W: fmt::Write>(&self, w: &mut W, fmt: fmt::Arguments<'_>) -> Result<Self, fmt::Error> {
        self._write_separator(w)?;
        w.write_fmt(fmt)?;
        Ok(Self::Dirty)
    }

    #[inline]
    fn _write_separator<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Clean => w.write_str("\x1B["),
            Self::Dirty => w.write_str(";"),
        }
    }
    #[inline]
    fn write_terminator<W: fmt::Write>(&self, w: &mut W) -> Result<Self, fmt::Error> {
        match self {
            Self::Clean => (),
            Self::Dirty => w.write_str("m")?,
        }
        Ok(Self::Clean)
    }
}

pub(crate) struct Formatter<'a,'f> where 'f: 'a {
    f: &'a mut fmt::Formatter<'f>,
    state: State,
}

impl<'a,'f> Formatter<'a,'f> where 'f: 'a {
    #[inline]
    pub(crate) fn fmt_ansi(f: &'a mut fmt::Formatter<'f>, ansi: Ansi) -> fmt::Result {
        if !ansi.is_empty() {
            let mut w = Self::new(f);
            ansi.write(&mut w)?;
            w.write_terminator()?;
        }
        Ok(())
    }
    #[inline]
    fn new(f: &'a mut fmt::Formatter<'f>) -> Self {
        Self { f, state: State::Clean }
    }
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.state = self.state.write_str(self.f, s)?;
        Ok(())
    }
    #[cfg(feature="color256")]
    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> fmt::Result {
        self.state = self.state.write_fmt(self.f, fmt)?;
        Ok(())
    }
    #[inline]
    fn write_terminator(&mut self) -> fmt::Result {
        self.state = self.state.write_terminator(self.f)?;
        Ok(())
    }

    pub(crate) fn write_reset(&mut self) -> fmt::Result { self.write_str("0") }

    pub(crate) fn write_color(&mut self, color: Coloree, value: ToggleColor) -> fmt::Result {
        match (color, value) {
            (Text,       ToggleColor::Reset) => { self.write_str("39") },
            (Background, ToggleColor::Reset) => { self.write_str("49") },
            (Text,       ToggleColor::Set(c)) => match c {
                #[cfg(feature="color256")]
                ColorNum(n)  => { self.write_fmt(format_args!("38;5;{}", n)) },
                #[cfg(feature="rgb")]
                Rgb(r,g,b)   => { self.write_fmt(format_args!("38;2;{};{};{}", r, g, b)) },
                Black        => { self.write_str( "30") },
                Red          => { self.write_str( "31") },
                Green        => { self.write_str( "32") },
                Yellow       => { self.write_str( "33") },
                Blue         => { self.write_str( "34") },
                Purple       => { self.write_str( "35") },
                Cyan         => { self.write_str( "36") },
                White        => { self.write_str( "37") },
                BrightBlack  => { self.write_str( "90") },
                BrightRed    => { self.write_str( "91") },
                BrightGreen  => { self.write_str( "92") },
                BrightYellow => { self.write_str( "93") },
                BrightBlue   => { self.write_str( "94") },
                BrightPurple => { self.write_str( "95") },
                BrightCyan   => { self.write_str( "96") },
                BrightWhite  => { self.write_str( "97") },
            },
            (Background, ToggleColor::Set(c)) => match c {
                #[cfg(feature="color256")]
                ColorNum(n)  => { self.write_fmt(format_args!("48;5;{}", n)) },
                #[cfg(feature="rgb")]
                Rgb(r,g,b)   => { self.write_fmt(format_args!("48;2;{};{};{}", r, g, b)) },
                Black        => { self.write_str( "40") },
                Red          => { self.write_str( "41") },
                Green        => { self.write_str( "42") },
                Yellow       => { self.write_str( "43") },
                Blue         => { self.write_str( "44") },
                Purple       => { self.write_str( "45") },
                Cyan         => { self.write_str( "46") },
                White        => { self.write_str( "47") },
                BrightBlack  => { self.write_str("100") },
                BrightRed    => { self.write_str("101") },
                BrightGreen  => { self.write_str("102") },
                BrightYellow => { self.write_str("103") },
                BrightBlue   => { self.write_str("104") },
                BrightPurple => { self.write_str("105") },
                BrightCyan   => { self.write_str("106") },
                BrightWhite  => { self.write_str("107") },
            },
        }
    }

    pub(crate) fn write_effect(&mut self, effect: Effect, value: Toggle) -> fmt::Result {
        match (effect, value) {
            (Bold,       Toggle::Reset) => { self.write_str("22") },
            (Faint,      Toggle::Reset) => { self.write_str("22") },
            (Italic,     Toggle::Reset) => { self.write_str("23") },
            (Underline,  Toggle::Reset) => { self.write_str("24") },
            (Blink,      Toggle::Reset) => { self.write_str("25") },
            (Reverse,    Toggle::Reset) => { self.write_str("27") },
            (Hidden,     Toggle::Reset) => { self.write_str("28") },
            (Strike,     Toggle::Reset) => { self.write_str("29") },
            (Bold,         Toggle::Set) => { self.write_str( "1") },
            (Faint,        Toggle::Set) => { self.write_str( "2") },
            (Italic,       Toggle::Set) => { self.write_str( "3") },
            (Underline,    Toggle::Set) => { self.write_str( "4") },
            (Blink,        Toggle::Set) => { self.write_str( "5") },
            (Reverse,      Toggle::Set) => { self.write_str( "7") },
            (Hidden,       Toggle::Set) => { self.write_str( "8") },
            (Strike,       Toggle::Set) => { self.write_str( "9") },
        }
    }
}
