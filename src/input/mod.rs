use crate::{tile::Tile, Ui};
use aery::{prelude::*, relation::RelationId, tuple_traits::RelationEntries};
use bevy::{
    ecs::system::SystemId,
    prelude::*,
    utils::{HashMap, HashSet},
};
use std::collections::VecDeque;

mod button;
mod cursor;
mod keyboard;
mod mouse;

use button::*;
use cursor::*;
use keyboard::*;
use mouse::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct VirtualInputState;

#[derive(Component, Deref)]
pub struct InputInfo(pub(crate) String);

#[derive(Relation)]
pub struct Bindings;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct VirtualInputSource;

#[derive(Relation)]
pub struct HandleOf;

pub trait Source: Component {}

pub trait State: Component {}

#[derive(Component, Deref, DerefMut)]
struct ShouldRun(bool);

#[derive(Component)]
struct MatcherSys(SystemId);

#[derive(Component)]
struct HandleSys(SystemId);

//  cmds.spawn((Text("ClickMe"), Button, Counter(0)))
//      .handle::<(With<Lmb>, With<Press>)>(|me: EntityMut| { *me.get_mut::<Counter>().unwrap() += 1; })
//      .handle::<Press, Rmb>(|me: EntityMut| { *me.get_mut::<Counter>().unwrap() = 0; });

pub trait Handle {
    fn handle<Stt, Src>(&mut self, sys: impl IntoSystem<(), (), ()> + 'static) -> &mut Self
    where
        Stt: State,
        Src: Source;
}

#[rustfmt::skip]
impl Handle for EntityWorldMut<'_> {
    fn handle<Stt, Src>(&mut self, sys: impl IntoSystem<(), (), ()> + 'static) -> &mut Self
    where
        Stt: State,
        Src: Source,
    {
        let id = self.id();
        self.world_scope(move |world| {
            // TODO: Use `In` + `Out` params
            let handle_sys_id = world.register_system(sys);
            let handle_ent = world.spawn((ShouldRun(false), HandleSys(handle_sys_id))).id();
            let matcher_sys_id = world.register_system(move |
                world: &mut World,
                sources: &mut QueryState<Relations<Bindings>, With<Src>>,
                states: &mut QueryState<(), With<Stt>>,
            | {
                let mut should_run = false;
                for edges in sources.iter(world) {
                    let [e] = edges.hosts(RelationId::of::<Bindings>()) else { continue };
                    if states.get(world, *e).is_ok() {
                        should_run = true;
                        break;
                    }
                }

                **world.entity_mut(handle_ent).get_mut::<ShouldRun>().unwrap() = should_run;
            });

            world
                .entity_mut(handle_ent)
                .insert(MatcherSys(matcher_sys_id))
                .set::<HandleOf>(id);
        });

        self
    }
}

#[rustfmt::skip]
fn capture_and_bubble(
    mut cmds: Commands,
    mut cursors: Query<(Entity, &mut Cursor)>,
    tree: Query<((Entity, &Tile), Relations<Ui>)>,
    roots: Query<Entity, Root<Ui>>,
) {
    for (cursor_entity, mut cursor) in cursors.iter_mut() {
        cursor.relative = cursor.absolute;
        let mut focus = Entity::PLACEHOLDER;

        // TODO: Don't ignore z
        if let Some(((entity, rect), _)) = roots
            .iter()
            .flat_map(|entity| tree.get(entity))
            .find(|((_, rect), _)| rect.contains(cursor.relative))
        {
            focus = entity;
            cursor.relative -= rect.pos;
        }

        loop {
            let Ok(((_, _), edges)) = tree.get(focus) else {
                break
            };

            let Some (((entity, rect), _)) = edges
                .hosts(RelationId::of::<Ui>())
                .iter()
                .flat_map(|e| tree.get(*e))
                .find(|((_, rect), _)| rect.contains(cursor.relative))
            else {
                break
            };

            focus = entity;
            cursor.relative -= rect.pos;
        }

        if focus != Entity::PLACEHOLDER {
            todo!()
        }
    }
}
