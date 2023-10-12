use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Cursor(pub Vec3);

#[derive(Component)]
pub struct MouseInput;
