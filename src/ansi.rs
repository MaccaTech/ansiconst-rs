mod color;
mod effect;
mod attr;
use color::Colors;
use effect::Effects;
use crate::{Color, Coloree, Effect};
use crate::introspect::Attr;
use crate::write::{compile_time, run_time};
use attr::{Flags, Attrs};
use std::fmt;

/// Represents an arbitrary combination of ANSI [`Effect`]s and
/// foreground/background [`Color`]s.
///
/// Additionally, provides a mechanism for preventing any/all of these attributes from
/// being changed in the `Ansi` that results from combining two `Ansi` instances.
/// See [`important`](Self::important), [`only`](Self::only) and [`no_ansi`](Self::no_ansi).
///
/// Note: this struct is designed to be *immutable* and *const*
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Ansi {
    effects: Effects,
    colors:  Colors,
    flags:   Flags,
}

impl Ansi {
    /// True if this instance is [`no_ansi`][Self::no_ansi()]
    #[inline]
    pub const fn is_no_ansi(&self) -> bool { self.flags.is_no_ansi() }

    /// True if this instance is [`empty`](Self::empty())
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.effects.is_empty() && self.colors.is_empty() && !self.flags.is_no_ansi()
    }

    /// True if this instance is [`reset`](Self::reset())
    #[inline]
    pub const fn is_reset(&self) -> bool {
        self.effects.is_reset() && self.colors.is_reset()
    }

    /// True if this instance is [`only`][Self::only()]
    #[inline]
    pub const fn is_only(&self) -> bool {
        self.effects.attrs().is_all() && self.colors.attrs().is_all()
    }

    /// Creates an `Ansi` that disables ANSI codes from being rendered entirely.
    /// See [`Styled<T>`](crate::Styled) for details.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ansiconst::styled_format_args;
    ///
    /// assert_eq!(
    ///     styled_format_args!(Ansi::no_ansi(), "This is plain, and {}",
    ///         styled_format_args!(Green, Underline, "this is plain too")
    ///     ).to_string(),
    ///     "This is plain, and this is plain too"
    /// );
    /// ```
    #[inline]
    pub const fn no_ansi() -> Ansi {
        Self { effects: Effects::empty(), colors: Colors::empty(), flags: Flags::no_ansi() }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Color`]s are `empty`,
    /// which means they do not represent any specific ANSI codes and so render
    /// an empty string when formatted.
    #[inline]
    pub const fn empty() -> Ansi {
        Self { effects: Effects::empty(), colors: Colors::empty(), flags: Flags::empty() }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Color`]s are `reset`,
    /// which means they would render ANSI *reset* codes for all attributes
    /// when each formatted individually.
    ///
    /// For brevity, the returned instance simply renders the universal
    /// ANSI reset `"\x1B[0m"` when formatted.
    #[inline]
    pub const fn reset() -> Ansi {
        Self { effects: Effects::reset(), colors: Colors::reset(), flags: Flags::empty() }
    }

    /// Creates an `Ansi` instance by adding another `Ansi`'s [`Effect`]s and [`Color`]s to `self`'s.
    ///
    /// In contrast to method [`then`](Self::then), with this method the [`important`](Self::important)
    /// modifier is **ignored**.
    ///
    /// In the case of overlapping attributes, `other`'s attributes replace `self`'s,
    /// including the presence/absence of the [`important`](Self::important()) modifier
    /// on those attributes.
    ///
    /// The resulting `Ansi`'s `important` attributes are those of `other` plus any `important`
    /// attributes of `self` that do not overlap `other`.
    ///
    /// If either `self` or `other` is [`no_ansi`](Self::no_ansi), then this is returned.
    #[inline]
    pub const fn add(&self, other: Ansi) -> Ansi {
        if self.is_no_ansi()  { return *self; }
        if other.is_no_ansi() { return other; }

        let important = self.important_attrs()
            .difference(other.attrs())
            .union(other.important_attrs());
        Self {
            effects: self.effects.add(other.effects),
            colors:  self.colors.add(other.colors),
            flags:   Flags::from_important(important),
        }
    }

    /// Creates an `Ansi` instance by adding another `Ansi`'s [`Effect`]s and [`Color`]s to `self`'s.
    ///
    /// In contrast to method [`add`](Self::add), with this method the [`important`](Self::important)
    /// modifier is **respected**.
    ///
    /// In the case of overlapping attributes, if `self`'s attributes are
    /// [`important`](Self::important) and `other`'s are not, then `self`'s take precedence;
    /// in all other cases, `other`'s take precedence.
    ///
    /// The resulting `Ansi`'s `important` attributes are the union of those of both instances.
    ///
    /// If either `self` or `other` is [`no_ansi`](Self::no_ansi), then this is returned.
    #[inline]
    pub const fn then(&self, other: Ansi) -> Ansi {
        if self.is_no_ansi()  { return *self; }
        if other.is_no_ansi() { return other; }

        let a = self.remove(self.important_attrs().intersection(other.important_attrs()));
        let b = other.remove(self.important_attrs().intersection(other.unimportant_attrs()));
        a.add(b)
    }

    #[inline]
    const fn remove(&self, attrs: Attrs) -> Ansi {
        if self.is_no_ansi()  { return *self; }

        Self {
            effects: self.effects.remove(attrs.effect),
            colors:  self.colors.remove(attrs.color),
            flags:   Flags::from_important(self.important_attrs().difference(attrs)),
        }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Color`]s will, when formatted,
    /// render the minimum ANSI codes necessary to transition from this instance's
    /// ANSI style to that of another instance.
    ///
    /// The resulting `Ansi`'s attributes are [`unimportant`](Self::unimportant()).
    #[inline]
    pub fn transition(&self, to_other: Ansi) -> Ansi {
        if self.is_no_ansi()     { return Self::empty();            }
        if to_other.is_no_ansi() { return self.not().unimportant(); }
        if to_other.is_reset()
            && !self.is_reset()  { return Self::reset();            }
        Self {
            effects: self.effects.transition(to_other.effects),
            colors:  self.colors.transition(to_other.colors),
            flags:   Flags::empty(),
        }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Color`]s, will, when formatted,
    /// render the ANSI codes necessary to reset this instance's ANSI style.
    ///
    /// ##### Example
    ///
    /// | Ansi                        | => | Ansi.not()                  |
    /// |-----------------------------|----|-----------------------------|
    /// | [`Bold`](Effect::Bold)      | => | [`Bold.not()`](Effect::not) |
    /// | [`Red`](Color::Red)         | => | [`Color::reset()`]          |
    /// | [`Bold.not()`](Effect::not) | => | `empty`                     |
    /// | [`Color::reset()`]          | => | `empty`                     |
    ///
    /// The resulting `Ansi`'s [`important`](Self::important) attributes
    /// are those of `self` that remain.
    ///
    /// If `self` is [`no_ansi`](Self::no_ansi), then this is returned.
    #[inline]
    pub const fn not(&self) -> Ansi {
        if self.is_no_ansi()  { return *self; }

        let effects = self.effects.not();
        let colors = self.colors.not();
        let attrs = Attrs::new(effects.attrs(), colors.attrs());
        let important = self.important_attrs().intersection(attrs);
        Self { effects, colors, flags: Flags::from_important(important) }
    }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Color`]s,
    /// and with all other attributes set to [`reset`](Self::reset()).
    ///
    /// This is used primarily for ensuring nested ANSI styles do not blend with
    /// parent styles when formatted. See [`Styled<T>`](crate::Styled) for details.
    ///
    /// If `self` is [`no_ansi`](Self::no_ansi), then this is returned.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ansiconst::styled_format_args;
    ///
    /// assert_eq!(
    ///     styled_format_args!(Blue, "This is blue only, and {}",
    ///         styled_format_args!(Italic, "this is blue/italic")
    ///     ).to_string(),
    ///     "\x1B[34mThis is blue only, and \x1B[3mthis is blue/italic\x1B[23m\x1B[39m"
    /// );
    ///
    /// assert_eq!(
    ///     styled_format_args!(Blue, "This is blue only, and {}",
    ///         styled_format_args!(Italic.only(), "this is italic only")
    ///     ).to_string(),
    ///     "\x1B[34mThis is blue only, and \x1B[39;3mthis is italic only\x1B[23;34m\x1B[39m"
    /// );
    /// ```
    #[inline]
    pub const fn only(&self) -> Ansi {
        if self.is_no_ansi()  { return *self; }

        Self {
            effects: self.effects.only(),
            colors:  self.colors.only(),
            flags:   self.flags,
        }
    }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Color`]s,
    /// but with `important` enabled (i.e. for the non-empty attributes).
    ///
    /// Similar to HTML's CSS, `important` attributes take precedence over
    /// normal attributes when combining two `Ansi` instances.
    ///
    /// This is particularly useful when dealing with formatting nested `Ansi` instances.
    /// We may want to, for example, prevent nested `Ansi` instances from changing the color.
    ///
    /// If `self` is [`no_ansi`](Self::no_ansi), then this is returned.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ansiconst::{Ansi, ansi, styled_format_args};
    ///
    /// const BLUE:       Ansi = ansi!(Blue.important());
    /// const RED_ITALIC: Ansi = ansi!(Red, Italic);
    ///
    /// assert_eq!(
    ///     styled_format_args!(BLUE, "This is blue, and {}",
    ///         styled_format_args!(RED_ITALIC, "this is blue/italic")
    ///     ).to_string(),
    ///     "\x1B[34mThis is blue, and \x1B[3mthis is blue/italic\x1B[23m\x1B[39m"
    /// );
    /// ```
    #[inline]
    pub const fn important(&self) -> Ansi {
        if self.is_no_ansi()  { return *self; }

        Self {
            effects: self.effects,
            colors:  self.colors,
            flags:   Flags::from_important(self.attrs()),
        }
    }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Color`]s,
    /// but with [`important`](Self::important) disabled for all attributes.
    #[inline]
    pub const fn unimportant(&self) -> Ansi {
        if self.is_no_ansi()  { return *self; }

        Self {
            effects: self.effects,
            colors:  self.colors,
            flags:   Flags::empty(),
        }
    }

    /// Used by the `styled_*!` macros to coerce a style argument to an `Ansi` instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi { *self }

    /// Gets the specified [`Effect`] attribute, or `None` if not set.
    #[inline]
    pub const fn get_effect(&self, effect: Effect) -> Option<Attr<Effect>> {
        match self.effects.get_effect(effect) {
            None => None,
            Some(attr) => Some(
                if self.is_important_effect(effect) { attr.important() } else { attr }
            ),
        }
    }

    /// Gets specified [`Coloree`] attribute, or `None` if not set.
    #[inline]
    pub const fn get_color(&self, coloree: Coloree) -> Option<Attr<Color>> {
        match self.colors.get_color(coloree) {
            None => None,
            Some(attr) => Some(
                if self.is_important_color(coloree) { attr.important() } else { attr }
            ),
        }
    }

    /// True if the specified [`Effect`] is set to `important`.
    #[inline]
    const fn is_important_effect(&self, effect: Effect) -> bool {
        self.important_attrs().effect.contains_effect(effect)
    }
    /// True if the specified color is set to `important`.
    #[inline]
    const fn is_important_color(&self, coloree: Coloree) -> bool {
        self.important_attrs().color.contains_coloree(coloree)
    }

    #[inline]
    const fn attrs(&self) -> Attrs {
        Attrs::new(self.effects.attrs(), self.colors.attrs())
    }
    #[inline]
    const fn important_attrs(&self) -> Attrs {
        self.flags.important()
    }
    #[inline]
    const fn unimportant_attrs(&self) -> Attrs {
        self.attrs().difference(self.flags.important())
    }

    #[inline]
    const fn from_effects(effects: Effects) -> Ansi {
        Self { effects, colors: Colors::empty(), flags: Flags::empty() }
    }
    #[inline]
    const fn from_colors(colors: Colors) -> Ansi {
        Self { colors, effects: Effects::empty(), flags: Flags::empty() }
    }
    #[inline]
    pub(super) const fn from_color(color: Color, toggle: Toggle, coloree: Coloree) -> Ansi {
        Self::from_colors(Colors::from_color(color, toggle, coloree))
    }
    #[inline]
    pub(super) const fn from_effect(effect: Effect, toggle: Toggle) -> Ansi {
        Self::from_effects(Effects::from_effect(effect, toggle))
    }

    #[inline]
    pub(crate) fn fmt_no_alternate(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        run_time::Formatter::fmt_ansi(f, *self)
    }

    #[inline]
    pub(crate) fn write(&self, w: &mut run_time::Formatter<'_,'_>) -> fmt::Result {
        if self.is_empty() {
            // Do nothing
        } else if self.is_reset() {
            w.write_reset()?;
        } else {
            if self.is_only() {
                w.write_reset()?; // Reset all
            } else {
                self.effects.write(w, Toggle::Reset)?;
                self.colors .write(w, Toggle::Reset)?;
            }
            self.effects.write(w, Toggle::Set)?;
            self.colors .write(w, Toggle::Set)?;
        }
        Ok(())
    }

    #[inline]
    pub(crate) const fn write_const(&self, mut w: compile_time::Writer) -> compile_time::Writer {
        if self.is_empty() {
            // Do nothing
        } else if self.is_reset() {
            w = w.write_reset();
        } else {
            if self.is_only() {
                w = w.write_reset(); // Reset all
            } else {
                w = self.effects.write_const(w, Toggle::Reset);
                w = self.colors .write_const(w, Toggle::Reset);
            }
            w = self.effects.write_const(w, Toggle::Set);
            w = self.colors .write_const(w, Toggle::Set);
        }
        w
    }
}

impl From<Effect> for Ansi {
    fn from(value: Effect) -> Ansi { Ansi::from_effects(value.into()) }
}
impl From<Effects> for Ansi {
    fn from(value: Effects) -> Ansi { Ansi::from_effects(value) }
}
impl From<Color> for Ansi {
    fn from(value: Color) -> Ansi { Ansi::from_colors(value.into()) }
}
impl From<Colors> for Ansi {
    fn from(value: Colors) -> Ansi { Ansi::from_colors(value) }
}

impl fmt::Display for Ansi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        run_time::Formatter::fmt_ansi(f, if f.alternate() { self.not() } else { *self })
    }
}

impl fmt::Debug for Ansi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_no_ansi() { return write!(f, "no_ansi"); }
        write!(f, "[")?;
        let mut sep = "";
        for effect in Effect::all() {
            if let Some(attr) = self.get_effect(*effect) {
                write!(f, "{sep}{:?}", attr)?;
                sep = ", ";
            }
        }
        for coloree in Coloree::all() {
            if let Some(attr) = self.get_color(*coloree) {
                write!(f, "{sep}{:?}", attr)?;
                sep = ", ";
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

/// Represents either an explicit [`Color`] or [`Effect`], or the corresponding
/// `reset` code.
#[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
pub(crate) enum Toggle {
    // An explicit [`Color`] or [`Effect`]
    Set,
    /// The `reset` code for the [`Color`] or [`Effect`]
    Reset
}

/// Represents either an explicit [`Color`] or the `reset` color code
#[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
pub(crate) enum ToggleColor {
    /// An explicit [`Color`]
    Set(Color),
    /// The `reset` color code (SGR parameters: foreground 39, background 49)
    Reset,
}
