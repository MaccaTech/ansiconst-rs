use crate::ansi::{Ansi, Colour, Effect, Effects};
use std::fmt;

#[inline]
fn write_ansi<W: fmt::Write>(w: &mut W, ansi: Ansi) -> fmt::Result {
    if ansi.is_unspecified() {
        // Do nothing
    } else if ansi.is_reset() {
        write!(w, "0")?;
    } else {
        write_ef(w, ansi.effect())?;
        write_fg(w, ansi.colour().fg())?;
        write_bg(w, ansi.colour().bg())?;
    }
    Ok(())
}

#[inline]
fn write_ef<W: fmt::Write>(w: &mut W, ef: Effects) -> fmt::Result {
    // Note: do resets first, because bold & faint share the same reset code
    if ef.has_effect(Effect::NotBold     )
    || ef.has_effect(Effect::NotFaint    ) { write!(w, "22")?; }
    if ef.has_effect(Effect::NotItalic   ) { write!(w, "23")?; }
    if ef.has_effect(Effect::NotUnderline) { write!(w, "24")?; }
    if ef.has_effect(Effect::NotBlink    ) { write!(w, "25")?; }
    if ef.has_effect(Effect::NotReverse  ) { write!(w, "27")?; }
    if ef.has_effect(Effect::NotHidden   ) { write!(w, "28")?; }
    if ef.has_effect(Effect::NotStrike   ) { write!(w, "29")?; }
    if ef.has_effect(Effect::Bold        ) { write!(w,  "1")?; }
    if ef.has_effect(Effect::Faint       ) { write!(w,  "2")?; }
    if ef.has_effect(Effect::Italic      ) { write!(w,  "3")?; }
    if ef.has_effect(Effect::Underline   ) { write!(w,  "4")?; }
    if ef.has_effect(Effect::Blink       ) { write!(w,  "5")?; }
    if ef.has_effect(Effect::Reverse     ) { write!(w,  "7")?; }
    if ef.has_effect(Effect::Hidden      ) { write!(w,  "8")?; }
    if ef.has_effect(Effect::Strike      ) { write!(w,  "9")?; }
    Ok(())
}

#[inline]
fn write_fg<W: fmt::Write>(w: &mut W, fg: Colour) -> fmt::Result {
    match fg {
        Colour::Unspecified        => (),
        Colour::Reset              => { write!(w,  "39")?; },
        #[cfg(feature="ansi256")]
        Colour::Ansi256(num)       => { write!(w,  "38;5;{}", num)?; },
        #[cfg(feature="rgb")]
        Colour::Rgb(r,g,b)         => { write!(w,  "38;2;{};{};{}", r, g, b)?; },
        Colour::Black              => { write!(w,  "30")?; },
        Colour::Red                => { write!(w,  "31")?; },
        Colour::Green              => { write!(w,  "32")?; },
        Colour::Yellow             => { write!(w,  "33")?; },
        Colour::Blue               => { write!(w,  "34")?; },
        Colour::Purple             => { write!(w,  "35")?; },
        Colour::Cyan               => { write!(w,  "36")?; },
        Colour::White              => { write!(w,  "37")?; },
        Colour::BrightBlack        => { write!(w,  "90")?; },
        Colour::BrightRed          => { write!(w,  "91")?; },
        Colour::BrightGreen        => { write!(w,  "92")?; },
        Colour::BrightYellow       => { write!(w,  "93")?; },
        Colour::BrightBlue         => { write!(w,  "94")?; },
        Colour::BrightPurple       => { write!(w,  "95")?; },
        Colour::BrightCyan         => { write!(w,  "96")?; },
        Colour::BrightWhite        => { write!(w,  "97")?; },
    }
    Ok(())
}

#[inline]
fn write_bg<W: fmt::Write>(w: &mut W, bg: Colour) -> fmt::Result {
    match bg {
        Colour::Unspecified        => (),
        Colour::Reset              => { write!(w,  "49")?; },
        #[cfg(feature="ansi256")]
        Colour::Ansi256(num)       => { write!(w,  "48;5;{}", num)?; },
        #[cfg(feature="rgb")]
        Colour::Rgb(r,g,b)         => { write!(w,  "48;2;{};{};{}", r, g, b)?; },
        Colour::Black              => { write!(w,  "40")?; },
        Colour::Red                => { write!(w,  "41")?; },
        Colour::Green              => { write!(w,  "42")?; },
        Colour::Yellow             => { write!(w,  "43")?; },
        Colour::Blue               => { write!(w,  "44")?; },
        Colour::Purple             => { write!(w,  "45")?; },
        Colour::Cyan               => { write!(w,  "46")?; },
        Colour::White              => { write!(w,  "47")?; },
        Colour::BrightBlack        => { write!(w, "100")?; },
        Colour::BrightRed          => { write!(w, "101")?; },
        Colour::BrightGreen        => { write!(w, "102")?; },
        Colour::BrightYellow       => { write!(w, "103")?; },
        Colour::BrightBlue         => { write!(w, "104")?; },
        Colour::BrightPurple       => { write!(w, "105")?; },
        Colour::BrightCyan         => { write!(w, "106")?; },
        Colour::BrightWhite        => { write!(w, "107")?; },
    }
    Ok(())
}

pub(crate) struct Formatter<'a,'f> where 'f: 'a {
    f: &'a mut fmt::Formatter<'f>,
    has_written_anything: bool
}

impl<'a,'f> Formatter<'a,'f> where 'f: 'a {
    #[inline]
    pub(crate) fn fmt_ansi(f: &'a mut fmt::Formatter<'f>, ansi: Ansi) -> fmt::Result {
        if !ansi.is_unspecified() {
            let mut w = Self::new(f);
            write_ansi(&mut w, ansi)?;
            w.write_terminator()?;
        }
        Ok(())
    }
    #[inline]
    fn new(f: &'a mut fmt::Formatter<'f>) -> Self { Formatter { f, has_written_anything: false } }
    #[inline]
    fn write_separator(&mut self) -> fmt::Result {
        if !self.has_written_anything {
            self.has_written_anything = true;
            write!(self.f, "\x1B[")
        } else {
            write!(self.f, ";")
        }
    }
    #[inline]
    fn write_terminator(&mut self) -> fmt::Result {
        if self.has_written_anything {
            self.has_written_anything = false;
            write!(self.f, "m")?;
        }
        Ok(())
    }
}

impl fmt::Write for Formatter<'_,'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_separator()?;
        self.f.write_str(s)
    }
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> fmt::Result {
        self.write_separator()?;
        self.f.write_fmt(fmt)
    }
}
