use crate::ansi::{Ansi, Colour, Effect, Effects};

const fn write_ansi(mut w: ConstWriter, ansi: Ansi) -> ConstWriter {
    if ansi.is_unspecified() {
        // Do nothing
    } else if ansi.is_reset() {
        w = w.write(0);
    } else {
        w = write_ef(w, ansi.effect());
        w = write_fg(w, ansi.colour().fg());
        w = write_bg(w, ansi.colour().bg());
    }
    w
}

const fn write_ef(mut w: ConstWriter, ef: Effects) -> ConstWriter {
    // Note: do resets first, because bold & faint share the same reset code
    if ef.has_effect(Effect::NotBold     )
    || ef.has_effect(Effect::NotFaint    ) { w = w.write(22); }
    if ef.has_effect(Effect::NotItalic   ) { w = w.write(23); }
    if ef.has_effect(Effect::NotUnderline) { w = w.write(24); }
    if ef.has_effect(Effect::NotBlink    ) { w = w.write(25); }
    if ef.has_effect(Effect::NotReverse  ) { w = w.write(27); }
    if ef.has_effect(Effect::NotHidden   ) { w = w.write(28); }
    if ef.has_effect(Effect::NotStrike   ) { w = w.write(29); }
    if ef.has_effect(Effect::Bold        ) { w = w.write( 1); }
    if ef.has_effect(Effect::Faint       ) { w = w.write( 2); }
    if ef.has_effect(Effect::Italic      ) { w = w.write( 3); }
    if ef.has_effect(Effect::Underline   ) { w = w.write( 4); }
    if ef.has_effect(Effect::Blink       ) { w = w.write( 5); }
    if ef.has_effect(Effect::Reverse     ) { w = w.write( 7); }
    if ef.has_effect(Effect::Hidden      ) { w = w.write( 8); }
    if ef.has_effect(Effect::Strike      ) { w = w.write( 9); }
    w
}

const fn write_fg(mut w: ConstWriter, fg: Colour) -> ConstWriter {
    match fg {
        Colour::Unspecified        => (),
        Colour::Reset              => { w = w.write( 39); },
        #[cfg(feature="ansi256")]
        Colour::Ansi256(num)       => { w = w.write( 38).write(5).write(num); },
        #[cfg(feature="rgb")]
        Colour::Rgb(r,g,b)         => { w = w.write( 38).write(2).write(r).write(g).write(b); },
        Colour::Black              => { w = w.write( 30); },
        Colour::Red                => { w = w.write( 31); },
        Colour::Green              => { w = w.write( 32); },
        Colour::Yellow             => { w = w.write( 33); },
        Colour::Blue               => { w = w.write( 34); },
        Colour::Purple             => { w = w.write( 35); },
        Colour::Cyan               => { w = w.write( 36); },
        Colour::White              => { w = w.write( 37); },
        Colour::BrightBlack        => { w = w.write( 90); },
        Colour::BrightRed          => { w = w.write( 91); },
        Colour::BrightGreen        => { w = w.write( 92); },
        Colour::BrightYellow       => { w = w.write( 93); },
        Colour::BrightBlue         => { w = w.write( 94); },
        Colour::BrightPurple       => { w = w.write( 95); },
        Colour::BrightCyan         => { w = w.write( 96); },
        Colour::BrightWhite        => { w = w.write( 97); },
    }
    w
}

const fn write_bg(mut w: ConstWriter, bg: Colour) -> ConstWriter {
    match bg {
        Colour::Unspecified        => (),
        Colour::Reset              => { w = w.write( 49); },
        #[cfg(feature="ansi256")]
        Colour::Ansi256(num)       => { w = w.write( 48).write(5).write(num); },
        #[cfg(feature="rgb")]
        Colour::Rgb(r,g,b)         => { w = w.write( 48).write(2).write(r).write(g).write(b); },
        Colour::Black              => { w = w.write( 40); },
        Colour::Red                => { w = w.write( 41); },
        Colour::Green              => { w = w.write( 42); },
        Colour::Yellow             => { w = w.write( 43); },
        Colour::Blue               => { w = w.write( 44); },
        Colour::Purple             => { w = w.write( 45); },
        Colour::Cyan               => { w = w.write( 46); },
        Colour::White              => { w = w.write( 47); },
        Colour::BrightBlack        => { w = w.write(100); },
        Colour::BrightRed          => { w = w.write(101); },
        Colour::BrightGreen        => { w = w.write(102); },
        Colour::BrightYellow       => { w = w.write(103); },
        Colour::BrightBlue         => { w = w.write(104); },
        Colour::BrightPurple       => { w = w.write(105); },
        Colour::BrightCyan         => { w = w.write(106); },
        Colour::BrightWhite        => { w = w.write(107); },
    }
    w
}

#[doc(hidden)]
pub struct Buffer<T> {
    pub array: T,
    pub len: usize,
}

impl Buffer<[u8;25]> {
    #[doc(hidden)]
    pub const fn from_ansi(ansi: Ansi) -> Self {
        write_ansi(ConstWriter::new(), ansi).take()
    }
}

struct ConstWriter { buf: Buffer<[u8;25]> }

impl ConstWriter {
    const fn new() -> Self { Self { buf: Buffer { array: [0u8;25], len: 0 } } }

    const fn write(mut self, value: u8) -> Self {
        self.buf.array[self.buf.len] = value;
        self.buf.len += 1;
        self
    }

    const fn take(self) -> Buffer<[u8;25]> { self.buf }
}
