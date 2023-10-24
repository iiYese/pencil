use aery::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct TimeOuts {
    pub click: f32,
    pub drag: f32,
}

#[derive(Component, Clone, Copy)]
pub struct Cursor {
    pub pos: Vec2,
    pub rel: Vec2,
}

#[derive(Relation)]
pub struct CursorTarget;

pub struct Hover;

pub struct Press;

pub struct Drag;

pub struct Release;

// Press + Release on the same entity
pub struct Click;
