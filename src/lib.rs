#![allow(clippy::type_complexity)]
#![allow(clippy::new_ret_no_self)]

pub mod inheritance;
pub mod input;
pub mod tile;
pub mod widgets;

use aery::prelude::*;
use bevy::prelude::*;

/// Trait to construct a bundle without asset API
pub trait Placeholder: Component {
    type Out: Bundle;
    fn build(self, world: &mut World) -> Self::Out;
}

/// TODO: Replace with hooks
pub fn build_placeholders<P: Placeholder>(mut cmds: Commands, query: Query<Entity, With<P>>) {
    for e in query.iter() {
        cmds.add(move |world: &mut World| {
            let built = world
                .entity_mut(e)
                .take::<P>()
                .expect("Placeholder component should exist on entity.")
                .build(world);

            world.entity_mut(e).insert(built);
        });
    }
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

    pub fn calculate(&self, denom: f32, available: f32) -> f32 {
        match *self {
            Self::Exact(val) => val,
            Self::Ratio(val) => (val / denom) * available,
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
    Placeholders,
    Inherit,
    Layout,
    CursorDrivers,
    VirtualInput,
    CaptureBubble,
    Draw,
}

pub struct PencilCase;

impl Plugin for PencilCase {
    fn build(&self, app: &mut App) {
        use crate::{
            inheritance::inherit,
            tile::{
                layout::{
                    fit_tiles, {Fit, Inset, Spacing},
                },
                style::Rounding,
            },
            UiSet::*,
        };

        app.configure_sets(
            PreUpdate,
            (
                Placeholders,
                Inherit,
                Layout,
                CursorDrivers,
                VirtualInput,
                CaptureBubble,
            )
                .chain(),
        )
        .configure_sets(Update, Draw)
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
        .add_systems(Update, fit_tiles.in_set(Layout))
        .add_systems(PreUpdate, apply_deferred.after(Placeholders))
        .add_systems(PreUpdate, apply_deferred.after(Inherit));
    }
}

pub mod prelude {
    pub use crate::inheritance::{inherit, Hidden, Inherit, InheritAll, Reject};
    pub use aery::prelude::*;
    pub use pencil_case_macros::*;
}
