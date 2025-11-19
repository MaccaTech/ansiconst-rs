use super::Styled;
use crate::Ansi;
use std::fmt;
use std::cell::Cell;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(super) enum StyledDisplay {
    Default,
    ToStyledString,
}

impl StyledDisplay {
    thread_local! {
        static ANSI: Cell<Ansi> = const { Cell::new(Ansi::empty()) };
    }

    #[inline]
    pub(super) fn ansi() -> Ansi { Self::ANSI.get() }

    pub(super) fn fmt_styled<T: fmt::Display>(&self, f: &mut fmt::Formatter<'_>, styled: &Styled<T>) -> fmt::Result {
        let old_ansi = Self::ANSI.get();
        let new_ansi = old_ansi.then(styled.ansi);
        // Uncomment for debugging:
        // println!("[DISPLAY]\n{:?}\n+\n{:?}\n=\n{:?}\n", old_ansi, self.ansi, new_ansi);
        if new_ansi == old_ansi {
            styled.target.fmt(f)?;
        } else {
            // Don't write ANSI codes if within to_styled_string()
            if *self == Self::Default {
                let old_to_new = old_ansi.transition(new_ansi);
                old_to_new.fmt_no_alternate(f)?;
            }

            Self::ANSI.set(new_ansi.only());
            styled.target.fmt(f)?;
            Self::ANSI.set(old_ansi);

            // Don't write ANSI codes if within to_styled_string()
            if *self == Self::Default {
                let new_to_old = new_ansi.transition(old_ansi);
                new_to_old.fmt_no_alternate(f)?;
            }
        }

        Ok(())
    }
}
