use super::*;

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct CursorTarget(Entity);

#[derive(Component, Clone, Copy)]
pub struct CursorPos {
    // pos in screen/window
    pub(crate) absolute: Vec2,
    // pos in ui rect
    pub(crate) relative: Vec2,
}

#[derive(Component)]
pub struct MouseMotionDriver;
