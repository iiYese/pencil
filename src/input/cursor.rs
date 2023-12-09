use super::*;

#[derive(Relation)]
pub struct Focus;

#[derive(Component, Clone, Copy)]
pub struct Cursor {
    // pos in screen/window
    pub(crate) absolute: Vec2,
    // pos in ui rect
    pub(crate) relative: Vec2,
}

#[derive(Component)]
pub struct MouseMotionDriver;
