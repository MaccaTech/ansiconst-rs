mod colour;
mod effect;
mod attr;
pub(crate) use colour::Colours;
pub(crate) use effect::Effects;
pub use colour::Colour;
pub use effect::Effect;
pub use attr::Attrs;
use std::fmt;

/// Represents an arbitrary combination of ANSI [`Effect`]s and
/// foreground/background [`Colour`]s.
///
/// Additionally, provides a mechanism for preventing any/all of these attributes from
/// being changed in the `Ansi` that results from combining two `Ansi` instances.
/// See [`protect_attrs()`](Self::protect_attrs())
///
/// Note: this struct is designed to be *immutable* and *const*
#[derive(PartialEq, Eq, Clone, Copy, fmt::Debug)]
pub struct Ansi {
    effect:  Effects,
    colour:  Colours,
    protect: Attrs,
}

impl Ansi {
    /// Gets the set of [`Attrs`] of this instance that are `specified`.
    #[inline]
    pub const fn attrs(&self) -> Attrs {
        self.effect.attrs().union(self.colour.attrs())
    }

    /// Gets the set of [`Attrs`] of this instance that are [`protected`](Self::protect_attrs()).
    #[inline]
    pub const fn protected_attrs(&self) -> Attrs {
        self.protect
    }

    /// True if this instance is `NoAnsi` - see [`no_ansi()`][Self::no_ansi()]
    #[inline]
    pub const fn is_no_ansi(&self) -> bool {
        self.is_unspecified() && self.is_only()
    }

    /// True if this instance is `Unspecified` - see [`unspecified()`](Self::unspecified())
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        self.effect.is_unspecified() && self.colour.is_unspecified()
    }

    /// True if this instance is `Unprotected` - see [`unprotect()`](Self::unprotect())
    #[inline]
    pub const fn is_unprotected(&self) -> bool {
        self.protect.is_empty()
    }

    /// True if this instance [`is_unspecified`](Self::is_unspecified)
    /// and [`is_unprotected`](Self::is_unprotected).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.is_unspecified() && self.is_unprotected()
    }

    /// True if this instance is `Reset` - see [`reset()`](Self::reset())
    #[inline]
    pub const fn is_reset(&self) -> bool {
        self.effect.is_reset() && self.colour.is_reset()
    }

    /// True if this instance is `Only` - see [`only()`][Self::only()]
    #[inline]
    pub const fn is_only(&self) -> bool {
        self.protect.is_all()
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Colour`]s are `Unspecified`,
    /// which means they do not represent any specific ANSI codes and so render
    /// an empty string when formatted.
    ///
    /// The resulting `Ansi`'s attributes are [`all protected`](Self::only()).
    ///
    /// This is used primarily for disabling ANSI codes from being rendered entirely.
    /// See [`Styled<T>`](crate::Styled) for details.
    #[inline]
    pub const fn no_ansi() -> Ansi {
        Self { effect: Effects::unspecified(), colour: Colours::unspecified(), protect: Attrs::all() }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Colour`]s are `Unspecified`,
    /// which means they do not represent any specific ANSI codes and so render
    /// an empty string when formatted.
    ///
    /// The resulting `Ansi`'s attributes are [`unprotected`](Self::unprotect_attrs()).
    #[inline]
    pub const fn unspecified() -> Ansi {
        Self { effect: Effects::unspecified(), colour: Colours::unspecified(), protect: Attrs::empty() }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Colour`]s are `Reset`,
    /// which means they would render ANSI *reset* codes for all attributes
    /// when each formatted individually.
    ///
    /// For brevity, the returned instance simply renders the universal
    /// ANSI reset `"\x1B[0m"` when formatted.
    ///
    /// The resulting `Ansi`'s attributes are [`unprotected`](Self::unprotect_attrs()).
    #[inline]
    pub const fn reset() -> Ansi {
        Self { effect: Effects::reset(), colour: Colours::reset(), protect: Attrs::empty() }
    }

    /// Creates an `Ansi` instance by adding another `Ansi`'s [`Effect`]s and [`Colour`]s to `self`'s.
    ///
    /// In the absence of [`protected attributes`](Self::protect_attrs()) in either `self`
    /// or `other`, the resulting `Ansi` is the union of `self`'s and `other`'s attributes,
    /// with `other`'s attributes replacing `self`'s in the event of overlap.
    ///
    /// In the event of `protected` attributes in either `self` or `other`,
    /// these attributes are preserved in the result, except where the same attributes are
    /// `protected` in both instances, in which case `self`'s take precedence.
    ///
    /// The resulting `Ansi`'s `protected` attributes are the union of those of both instances.
    #[inline]
    pub const fn add(&self, other: Ansi) -> Ansi {
        let filter_self  = other.protect.difference(self.protect).complement();
        let filter_other = self.protect.complement();
        Self {
            effect:  self.effect.filter(filter_self).add(other.effect.filter(filter_other)),
            colour:  self.colour.filter(filter_self).add(other.colour.filter(filter_other)),
            protect: self.protect.union(other.protect),
        }
    }

    /// Creates an `Ansi` instance by removing another `Ansi`'s [`Effect`]s and [`Colour`]s
    /// from `self`'s.
    ///
    /// In the absence of [`protected attributes`](Self::protect_attrs()) in `self`,
    /// the resulting `Ansi` is comprised of `self`'s attributes excluding any
    /// attributes that exist in `other`.
    ///
    /// In the event of `protected` attributes in `self`, these attributes are
    /// preserved in the result.
    ///
    /// The resulting `Ansi`'s `protected` attributes are those of `self`.
    #[inline]
    pub const fn remove(&self, other: Ansi) -> Ansi {
        Self {
            effect:  self.effect.remove(other.effect.filter(self.protect.complement())),
            colour:  self.colour.remove(other.colour.filter(self.protect.complement())),
            protect: self.protect,
        }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Colour`]s will, when formatted,
    /// render the minimum ANSI codes necessary to transition from this instance's
    /// ANSI style to that of another instance.
    ///
    /// The resulting `Ansi`'s attributes are [`unprotected`](Self::unprotect_attrs()).
    #[inline]
    pub fn transition(&self, to_other: Ansi) -> Ansi {
        Self {
            effect:  self.effect.transition(to_other.effect),
            colour:  self.colour.transition(to_other.colour),
            protect: Attrs::empty(),
        }
    }

    /// Creates an `Ansi` instance whose [`Effect`]s and [`Colour`]s, will, when formatted,
    /// render the ANSI codes necessary to reset this instance's ANSI style.
    ///
    /// For example, [`Effect::Bold`] becomes [`Effect::NotBold`] and
    /// [`Colour::Red`] becomes [`Colour::Reset`].
    ///
    /// The resulting `Ansi`'s [`protected attributes`](Self::protect_attrs())
    /// are those of `self`.
    #[inline]
    pub const fn not(&self) -> Ansi {
        Self {
            effect:  self.effect.not(),
            colour:  self.colour.not(),
            protect: self.protect,
        }
    }

    /// Creates an `Ansi` instance by including only the [`Effect`]s and [`Colour`]s of `self`
    /// that are selected by the given [`Attrs`].
    ///
    /// The resulting `Ansi`'s [`protected attributes`](Self::protect_attrs)
    /// are the intersection of `self`'s with those of the `attrs` parameter.
    #[inline]
    pub const fn filter(&self, attrs: Attrs) -> Ansi {
        Self {
            effect:  self.effect.filter(attrs),
            colour:  self.colour.filter(attrs),
            protect: self.protect.intersection(attrs),
        }
    }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Colour`]s
    /// but with [`protection`](Self::protect_attrs()) enabled for all [`Attrs`],
    /// including the `Unspecified` ones.
    ///
    /// See [`protect_attrs()`](Self::protect_attrs) for further details and examples.
    #[inline]
    pub const fn only(&self) -> Ansi { self.protect_attrs(Attrs::all()) }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Colour`]s,
    /// but with [`protection`](Self::protect_attrs()) enabled for any [`Attrs`] that are `specified`.
    ///
    /// See [`protect_attrs()`](Self::protect_attrs) for further details and examples.
    #[inline]
    pub const fn protect(&self) -> Ansi { self.protect_attrs(self.attrs()) }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Colour`]s,
    /// but with [`protection`](Self::unprotect_attrs()) disabled for all [`Attrs`],
    /// including the `Unspecified` ones.
    #[inline]
    pub const fn unprotect(&self) -> Ansi { self.unprotect_attrs(Attrs::all()) }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Colour`]s,
    /// but with protection enabled for the given [`Attrs`].
    ///
    /// Protected [`Attrs`] are not changed in the `Ansi` that results from
    /// combining this instance with another `Ansi` instance.
    ///
    /// This is particularly useful when dealing with formatting nested `Ansi` instances.
    /// We may want to prevent all ANSI codes from being displayed, or we may want
    /// to, for example, prevent nested `Ansi` instances from changing the colour.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ansiconst::{*, Effect::{Bold, Italic}, Colour::{Blue, Red}};
    ///
    /// const BLUE_BOLD:  Ansi = ansi!(Blue, Bold  ).protect_attrs(Attrs::Foreground);
    /// const RED_ITALIC: Ansi = ansi!(Red,  Italic).protect_attrs(Attrs::effects());
    ///
    /// assert_eq!(
    ///     styled_format_args!(BLUE_BOLD, "This is blue/bold, and {}",
    ///         styled_format_args!(RED_ITALIC, "this is blue/italic")
    ///     ).to_string(),
    ///     "\x1B[1;34mThis is blue/bold, and \x1B[22;3mthis is blue/italic\x1B[23;1m\x1B[22;39m"
    /// );
    /// ```
    #[inline]
    pub const fn protect_attrs(&self, attrs: Attrs) -> Ansi {
        Self {
            effect:  self.effect,
            colour:  self.colour,
            protect: self.protect.union(attrs),
        }
    }

    /// Creates an `Ansi` instance using this instance's [`Effect`]s and [`Colour`]s,
    /// but with protection disabled for the specified [`Attrs`].
    ///
    /// Unprotected [`Attrs`] may be changed in the `Ansi` that results from
    /// combining this instance with another `Ansi` instance.
    ///
    /// See [`protect_attrs()`](Self::protect_attrs) for further details and examples.
    #[inline]
    pub const fn unprotect_attrs(&self, attrs: Attrs) -> Ansi {
        Self {
            effect:  self.effect,
            colour:  self.colour,
            protect: self.protect.difference(attrs),
        }
    }

    /// Used by the `styled_*!` macros to coerce a style argument to an `Ansi` instance.
    #[inline]
    pub const fn ansi(&self) -> Ansi { *self }

    #[inline]
    pub(super) const fn from_effect(effect: Effects) -> Ansi {
        Self { effect, colour: Colours::unspecified(), protect: Attrs::empty() }
    }
    #[inline]
    pub(super) const fn from_colour(colour: Colours) -> Ansi {
        Self { colour, effect: Effects::unspecified(), protect: Attrs::empty() }
    }
    #[inline]
    pub(super) const fn effect(&self) -> Effects { self.effect }
    #[inline]
    pub(super) const fn colour(&self) -> Colours { self.colour }
}

impl From<Effect> for Ansi {
    fn from(value: Effect) -> Ansi { Ansi::from_effect(value.into()) }
}
impl From<Effects> for Ansi {
    fn from(value: Effects) -> Ansi { Ansi::from_effect(value) }
}
impl From<Colour> for Ansi {
    fn from(value: Colour) -> Ansi { Ansi::from_colour(value.into()) }
}
impl From<Colours> for Ansi {
    fn from(value: Colours) -> Ansi { Ansi::from_colour(value) }
}
