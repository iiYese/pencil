use crate::Units;
use bevy::prelude::*;
use pencil_case_macros::*;

#[derive(Clone, Component, Hereditary)]
pub struct Rounding(pub Units);

#[derive(Component)]
pub struct Main(Color);

#[derive(Component)]
pub struct Alt(Color);

#[derive(Component)]
pub struct Accent(Color);
