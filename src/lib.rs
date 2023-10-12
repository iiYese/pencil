#![allow(clippy::type_complexity)]
#![allow(clippy::new_ret_no_self)]

pub mod inheritance;
pub mod input;
pub mod layout;
pub mod style;

use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Units {
    /// Logical coordinates
    Exact(f32),
    /// Ratio of available space in proportion to siblings
    Ratio(f32),
}

#[derive(SystemSet, Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum UiSet {
    Propogate,
    Layout,
    Input,
    Draw,
}

pub struct PencilCase;

impl Plugin for PencilCase {
    fn build(&self, app: &mut App) {}
}

pub mod prelude {
    pub use aery::prelude::*;
    pub use pencil_case_macros::*;
}

// TODO
