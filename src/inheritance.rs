use aery::prelude::*;
use bevy::prelude::*;

use std::marker::PhantomData;

#[derive(Component)]
pub struct InheritAll;

/// Component that can be inherited  by entities in a hierarchy
pub trait Hereditary: Component + Clone {}

#[derive(Component)]
pub struct Inherit<T: Hereditary>(PhantomData<T>);

#[derive(Component)]
pub struct Reject<T: Hereditary>(PhantomData<T>);

impl<T: Hereditary> Reject<T> {
    pub fn and_replace(val: T) -> impl Bundle {
        (Self(PhantomData), val)
    }
}

#[derive(Component)]
pub struct Hidden<T: Hereditary>(PhantomData<T>);

impl<T: Hereditary> Hidden<T> {
    pub fn new(val: T) -> impl Bundle {
        (Self(PhantomData), val)
    }
}

pub fn propogate<C: Hereditary, R: Relation>(
    mut commands: Commands,
    roots: Query<Entity, Root<R>>,
    tree: Query<(Entity, Relations<R>)>,
    providers: Query<&C, Without<Hidden<C>>>,
    receivers: Query<
        Entity,
        (
            Or<(With<Inherit<C>>, With<InheritAll>)>,
            (Without<Reject<C>>, Without<Hidden<C>>),
        ),
    >,
) {
    tree.traverse::<R>(roots.iter())
        .track(&providers)
        .for_each(|provider, receiver, _| {
            if receivers.contains(*receiver) {
                commands.entity(*receiver).insert(provider.clone());
            }
        })
}
