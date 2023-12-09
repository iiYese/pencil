use bevy::prelude::*;

pub mod layout;
pub mod style;

#[derive(Component, Clone, Copy)]
pub struct Tile {
    /// Top left
    pub pos: Vec2,
    pub len: Vec2,
}
impl Tile {
    pub fn contains(&self, point: Vec2) -> bool {
        todo!()
    }
}
