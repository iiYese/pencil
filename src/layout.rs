use crate::Units;
use bevy::prelude::*;
use pencil_case_macros::*;

// General
pub struct Rect {
    pos: Vec2,
    len: Vec2,
    z: f32,
}

/// What direction to layout children that line up to a parent
pub enum Line {
    Vertical,
    Horizontal,
}

/// How to automatically fit a child inline within its parent.
/// Parent must have a [`Line`] direction.
/// - Fit(Units::Exact(_)): Takes up that many logical units in the parents line direction. The
/// other direction is fit to be within the space after [`Inset`].
/// - Fit(Units::Ratio(_)): Takes up the available space as a ratio to siblings that also fit inline.
#[derive(Clone, Component, Hereditary)]
pub struct Fit(pub Units);

#[derive(Clone, Component, Hereditary)]
pub struct Inset {
    line_start: Units,
    line_end: Units,
    perp_start: Units,
    perp_end: Units,
}

#[derive(Clone, Component, Hereditary)]
pub struct Spacing(pub Units);

// Scroll Areas
pub struct View {}

pub struct Stencil {}
