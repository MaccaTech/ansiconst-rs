use crate::{Ansi, Color::*, Coloree::{self, *}, Effect::{self, *}, Toggle, ToggleColor};

#[doc(hidden)]
pub struct Buffer<T> {
    pub array: T,
    pub len: usize,
}

impl Buffer<[u8;25]> {
    #[doc(hidden)]
    pub const fn from_ansi(ansi: Ansi) -> Self {
        ansi.write_const(Writer::new()).take()
    }
}

pub(crate) struct Writer { buf: Buffer<[u8;25]> }

impl Writer {
    const fn new() -> Self { Self { buf: Buffer { array: [0u8;25], len: 0 } } }

    const fn write(mut self, value: u8) -> Self {
        self.buf.array[self.buf.len] = value;
        self.buf.len += 1;
        self
    }

    const fn take(self) -> Buffer<[u8;25]> { self.buf }

    pub(crate) const fn write_reset(self) -> Self { self.write(0) }

    pub(crate) const fn write_color(self, color: Coloree, value: ToggleColor) -> Self {
        match (color, value) {
            (Text,       ToggleColor::Reset) => { self.write(39) },
            (Background, ToggleColor::Reset) => { self.write(49) },
            (Text,       ToggleColor::Set(c)) => match c {
                #[cfg(feature="color256")]
                ColorNum(n)  => { self.write(38).write(5).write(n) },
                #[cfg(feature="rgb")]
                Rgb(r,g,b)   => { self.write(38).write(2).write(r).write(g).write(b) },
                Black        => { self.write( 30) },
                Red          => { self.write( 31) },
                Green        => { self.write( 32) },
                Yellow       => { self.write( 33) },
                Blue         => { self.write( 34) },
                Purple       => { self.write( 35) },
                Cyan         => { self.write( 36) },
                White        => { self.write( 37) },
                BrightBlack  => { self.write( 90) },
                BrightRed    => { self.write( 91) },
                BrightGreen  => { self.write( 92) },
                BrightYellow => { self.write( 93) },
                BrightBlue   => { self.write( 94) },
                BrightPurple => { self.write( 95) },
                BrightCyan   => { self.write( 96) },
                BrightWhite  => { self.write( 97) },
            },
            (Background, ToggleColor::Set(c)) => match c {
                #[cfg(feature="color256")]
                ColorNum(n)  => { self.write(48).write(5).write(n) },
                #[cfg(feature="rgb")]
                Rgb(r,g,b)   => { self.write(48).write(2).write(r).write(g).write(b) },
                Black        => { self.write( 40) },
                Red          => { self.write( 41) },
                Green        => { self.write( 42) },
                Yellow       => { self.write( 43) },
                Blue         => { self.write( 44) },
                Purple       => { self.write( 45) },
                Cyan         => { self.write( 46) },
                White        => { self.write( 47) },
                BrightBlack  => { self.write(100) },
                BrightRed    => { self.write(101) },
                BrightGreen  => { self.write(102) },
                BrightYellow => { self.write(103) },
                BrightBlue   => { self.write(104) },
                BrightPurple => { self.write(105) },
                BrightCyan   => { self.write(106) },
                BrightWhite  => { self.write(107) },
            },
        }
    }

    pub(crate) const fn write_effect(self, effect: Effect, value: Toggle) -> Self {
        match (effect, value) {
            (Bold,       Toggle::Reset) => { self.write(22) },
            (Faint,      Toggle::Reset) => { self.write(22) },
            (Italic,     Toggle::Reset) => { self.write(23) },
            (Underline,  Toggle::Reset) => { self.write(24) },
            (Blink,      Toggle::Reset) => { self.write(25) },
            (Reverse,    Toggle::Reset) => { self.write(27) },
            (Hidden,     Toggle::Reset) => { self.write(28) },
            (Strike,     Toggle::Reset) => { self.write(29) },
            (Bold,         Toggle::Set) => { self.write( 1) },
            (Faint,        Toggle::Set) => { self.write( 2) },
            (Italic,       Toggle::Set) => { self.write( 3) },
            (Underline,    Toggle::Set) => { self.write( 4) },
            (Blink,        Toggle::Set) => { self.write( 5) },
            (Reverse,      Toggle::Set) => { self.write( 7) },
            (Hidden,       Toggle::Set) => { self.write( 8) },
            (Strike,       Toggle::Set) => { self.write( 9) },
        }
    }
}
