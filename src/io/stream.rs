use crate::{styled_write, Ansi};
use std::io::{self, IsTerminal};
use std::fmt;
use std::cell::Cell;

use super::{AnsiPreference, AnsiWrite};

static mut ANSIOUT: Cell<Option<Ansi>> = Cell::new(None);
static mut ANSIERR: Cell<Option<Ansi>> = Cell::new(None);

/// A `Writer` that writes styled output to an inner [`StdoutLock`](std::io::StdoutLock) using
/// a configurable default [`Ansi`] instance.
///
/// Like its inner [`StdoutLock`](std::io::StdoutLock), each instance of this `Writer` is
/// a handle to a *shared* global construct. The configurable default [`Ansi`] is therefore
/// shared amongst all `Ansiout` instances. Threadsafe access is ensured thanks to the
/// inner lock's mutex.
///
/// Created by the [`ansiout`] method.
///
/// **Note**: only calls to this `Writer`'s [`write_fmt()`](io::Write::write_fmt()) method
/// will have the default ANSI styling applied. Calls to any other [`Write`](io::Write)
/// methods are unaffected.
pub struct Ansiout(io::StdoutLock<'static>);
/// A `Writer` that writes styled output to an inner [`StderrLock`](std::io::StderrLock) using
/// a configurable default [`Ansi`] instance.
///
/// Like its inner [`StderrLock`](std::io::StderrLock), each instance of this `Writer` is
/// a handle to a *shared* global construct. The configurable default [`Ansi`] is therefore
/// shared amongst all `Ansierr` instances. Threadsafe access is ensured thanks to the
/// inner lock's mutex.
///
/// Created by the [`ansierr`] method.
///
/// **Note**: only calls to this `Writer`'s [`write_fmt()`](io::Write::write_fmt()) method
/// will have the default ANSI styling applied. Calls to any other [`Write`](io::Write)
/// methods are unaffected.
pub struct Ansierr(io::StderrLock<'static>);

/// Creates an [`Ansiout`] that wraps the result of locking [`stdout()`](io::stdout())
///
pub fn ansiout() -> Ansiout { Ansiout(io::stdout().lock()) }
/// Creates an [`Ansierr`] that wraps the result of locking [`stderr()`](io::stderr())
pub fn ansierr() -> Ansierr { Ansierr(io::stderr().lock()) }

 impl Ansiout {
    // Needed so that this crate's paint*! macros work without having std::io::Write in scope
    #[inline]
    #[doc(hidden)]
    pub fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        io::Write::write_fmt(self, fmt)
    }
}

impl Ansierr {
    // Needed so that this crate's paint*! macros work without having std::io::Write in scope
    #[inline]
    #[doc(hidden)]
    pub fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        io::Write::write_fmt(self, fmt)
    }
}

impl AnsiWrite for Ansiout {
    fn ansi(&self) -> Ansi {
        // Note: actually safe, because we're holding a StdoutLock
        unsafe {
            match ANSIOUT.get() {
                None => {
                    let ansi: Ansi = self.preferred_ansi();
                    ANSIOUT.set(Some(ansi));
                    ansi
                },
                Some(ansi) => ansi,
            }
        }
    }

    fn set_ansi(&mut self, ansi: Ansi) {
        // Note: actually safe, because we're holding a StdoutLock
        unsafe { ANSIOUT.set(Some(ansi)); }
    }
}

impl AnsiWrite for Ansierr {
    fn ansi(&self) -> Ansi {
        // Note: actually safe, because we're holding a StderrLock
        unsafe {
            match ANSIERR.get() {
                None => {
                    let ansi: Ansi = self.preferred_ansi();
                    ANSIERR.set(Some(ansi));
                    ansi
                },
                Some(ansi) => ansi,
            }
        }
    }

    fn set_ansi(&mut self, ansi: Ansi) {
        // Note: actually safe, because we're holding a StderrLock
        unsafe { ANSIERR.set(Some(ansi)); }
    }
}

impl AnsiPreference for Ansiout {
    fn is_ansi_preferred(&self) -> bool { self.0.is_terminal() }
}
impl AnsiPreference for Ansierr {
    fn is_ansi_preferred(&self) -> bool { self.0.is_terminal() }
}

impl io::Write for Ansiout {
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        if ! self.ansi().is_empty() {
            styled_write!(self.0, self.ansi(), "{}", fmt)
        } else {
            self.0.write_fmt(fmt)
        }
    }
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.0.write(buf) }
    fn flush(&mut self) -> io::Result<()> { self.0.flush() }
}
impl io::Write for Ansierr {
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        if ! self.ansi().is_empty() {
            styled_write!(self.0, self.ansi(), "{}", fmt)
        } else {
            self.0.write_fmt(fmt)
        }
    }
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.0.write(buf) }
    fn flush(&mut self) -> io::Result<()> { self.0.flush() }
}
