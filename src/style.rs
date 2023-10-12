use crate::Val;
use bevy::prelude::*;
use pencil_case_macros::*;

#[derive(Clone, Component, Hereditary)]
pub enum Rounding {
    Exact(f32),
    (f32),
}

#[derive(Component)]
struct Main(Color);

#[derive(Component)]
struct Alt(Color);

#[derive(Component)]
struct Accent(Color);
