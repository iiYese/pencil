use aery::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct TimeOuts {
    pub click: f32,
    pub drag: f32,
}

// Cursor -Driver-> One shot system
#[derive(Relation)]
pub struct Driver;

#[derive(Component, Clone, Copy)]
pub struct Cursor {
    pub pos: Vec2,
    pub rel: Vec2,
}

#[derive(Component)]
pub struct Snapshot {
    pub time: f32,
}

#[derive(Relation)]
pub struct CursorTarget;

#[derive(Component)]
pub struct MouseMotionDriver;

#[derive(Relation)]
pub struct Hover;

#[derive(Relation)]
pub struct Press;

#[derive(Relation)]
pub struct Drag;

#[derive(Relation)]
pub struct Release;

// Click: Press + Release on the same entity
