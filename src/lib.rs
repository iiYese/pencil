#![allow(clippy::type_complexity)]
#![allow(clippy::new_ret_no_self)]

pub mod inheritance;
pub mod input;
pub mod layout;
pub mod style;
pub mod widgets;

use aery::prelude::*;
use bevy::prelude::*;

/// Trait to construct a bundle without asset API
pub trait Placeholder: Component {
    type Out: Bundle;
    fn build(world: &mut World) -> Self::Out;
}

#[derive(Relation)]
pub struct Ui;

#[derive(Clone, Copy)]
pub enum Units {
    Exact(f32),
    Ratio(f32),
}

impl Units {
    pub fn exact(&self) -> Option<f32> {
        match self {
            Self::Exact(val) => Some(*val),
            _ => None,
        }
    }

    pub fn ratio(&self) -> Option<f32> {
        match self {
            Self::Ratio(val) => Some(*val),
            _ => None,
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::Exact(0.)
    }
}

#[derive(SystemSet, Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum UiSet {
    Inherit,
    Layout,
    Input,
    Draw,
}

pub struct PencilCase;

impl Plugin for PencilCase {
    fn build(&self, app: &mut App) {
        use crate::{
            inheritance::inherit,
            layout::{Fit, Inset, Spacing},
            style::Rounding,
            UiSet::*,
        };

        app.configure_sets(PreUpdate, (Inherit, Layout, Input).chain())
            .configure_set(Update, Draw)
            .add_systems(
                PreUpdate,
                (
                    inherit::<Ui, Fit>,
                    inherit::<Ui, Inset>,
                    inherit::<Ui, Spacing>,
                    inherit::<Ui, Rounding>,
                )
                    .in_set(Inherit),
            )
            .add_systems(PreUpdate, apply_deferred.after(Inherit))
            .add_systems(PreUpdate, apply_deferred.after(Input));
    }
}

pub mod prelude {
    pub use crate::inheritance::{inherit, Hidden, Inherit, InheritAny, Reject};
    pub use aery::prelude::*;
    pub use pencil_case_macros::*;
}

// TODO
