use std::io;
use std::fmt;

use crate::{styled_write, Ansi};
use super::{AnsiPreference, AnsiWrite};

/// A `Writer` that writes styled output to an inner [`Write`](io::Write) using
/// a configurable default [`Ansi`] instance.
///
/// **Note**: only calls to this `Writer`'s [`write_fmt()`](io::Write::write_fmt()) method
/// will have the default ANSI styling applied. Calls to any other [`Write`](io::Write)
/// methods are unaffected.
pub struct AnsiWriter<W: io::Write + AnsiPreference> {
    ansi: Ansi,
    writer: W,
}

impl<W: io::Write + AnsiPreference> AnsiWrite for AnsiWriter<W> {
    fn ansi(&self) -> Ansi { self.ansi }
    fn set_ansi(&mut self, ansi: Ansi) { self.ansi = ansi }
}

impl<W: io::Write + AnsiPreference> AnsiPreference for AnsiWriter<W> {
    fn is_ansi_preferred(&self) -> bool { self.writer.is_ansi_preferred() }
}

impl<W: io::Write + AnsiPreference> io::Write for AnsiWriter<W> {
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        if ! self.ansi.is_empty() {
            styled_write!(self.writer, self.ansi, "{}", fmt)
        } else {
            self.writer.write_fmt(fmt)
        }
    }
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.writer.write(buf) }
    fn flush(&mut self) -> io::Result<()> { self.writer.flush() }
}

impl<T: io::IsTerminal> AnsiPreference for T {
    fn is_ansi_preferred(&self) -> bool { self.is_terminal() }
}
