use crate::{tile::Tile, Ui};
use aery::{prelude::*, relation::RelationId, tuple_traits::RelationEntries};
use bevy::{
    ecs::system::SystemId,
    prelude::*,
    utils::{HashMap, HashSet},
};
use std::{collections::VecDeque, marker::PhantomData};

mod cursor;
mod keyboard;
mod mouse;

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

#[derive(Component)]
struct SourceMatcher<T: Source>(PhantomData<T>);

#[derive(Component)]
struct StateMatcher<T: State>(PhantomData<T>);

#[derive(Component, Deref, DerefMut)]
struct ShouldRun(bool);

#[derive(Component, Deref)]
struct SysId(SystemId);

#[derive(Relation)]
struct Matcher;

//  cmds.spawn((Text("ClickMe"), Button, Counter(0)))
//      .handle::<Press, Lmb>(|me: EntityMut| { *me.get_mut::<Counter>().unwrap() += 1; })
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
            // TODO:
            //  - Use `In` + `Out` params
            let matcher_ent = {
                let mut q = world.query_filtered::<
                    Entity,
                    (With<StateMatcher<Stt>>, With<SourceMatcher<Src>>, With<SysId>)
                >();

                if let Ok(matcher_ent) = q.get_single(world) {
                    matcher_ent
                } else {
                    let matcher_ent = world
                        .spawn((
                            ShouldRun(false),
                            StateMatcher::<Stt>(PhantomData),
                            SourceMatcher::<Src>(PhantomData)
                        ))
                        .id();

                    let matcher_sys_id = world.register_system(move |
                        mut should_run: Query<&mut ShouldRun>,
                        sources: Query<Relations<Bindings>, With<Src>>,
                        states: Query<(), With<Stt>>,
                    | {
                        let Ok(mut should_run) = should_run.get_mut(matcher_ent) else { return };
                        **should_run = sources
                            .iter()
                            .flat_map(|edges| edges
                                .hosts(RelationId::of::<Bindings>())
                                .first()
                                .copied()
                            )
                            .any(|e| states.get(e).is_ok());
                    });

                    world.entity_mut(matcher_ent).insert(SysId(matcher_sys_id));

                    matcher_ent
                }
            };

            let handle_sys_id = world.register_system(sys);

            world
                .spawn(SysId(handle_sys_id))
                .set::<HandleOf>(id)
                .set::<Matcher>(matcher_ent);
        });

        self
    }
}

fn clean_dangling() {}

fn eval_matchers() {}

#[rustfmt::skip]
fn capture(
    mut cursors: Query<(&mut CursorTarget, &mut CursorPos)>,
    tree: Query<((Entity, &Tile), Relations<Ui>)>,
    roots: Query<Entity, Root<Ui>>,
) {
    for (mut target, mut pos) in cursors.iter_mut() {
        pos.relative = pos.absolute;
        **target = Entity::PLACEHOLDER;

        // TODO:
        //  - Don't ignore z
        //  - Trigger hover
        if let Some(((entity, rect), _)) = roots
            .iter()
            .flat_map(|entity| tree.get(entity))
            .find(|((_, rect), _)| rect.contains(pos.relative))
        {
            **target = entity;
            pos.relative -= rect.pos;
        }

        loop {
            let Ok(((_, _), edges)) = tree.get(**target) else {
                break
            };

            let Some (((entity, rect), _)) = edges
                .hosts(RelationId::of::<Ui>())
                .iter()
                .flat_map(|e| tree.get(*e))
                .find(|((_, rect), _)| rect.contains(pos.relative))
            else {
                break
            };

            **target = entity;
            pos.relative -= rect.pos;
        }
    }
}

fn bubble(
    world: &mut World,
    targets: &mut QueryState<&CursorTarget>,
    run_checks: &mut QueryState<&ShouldRun>,
    handles: &mut QueryState<(&SysId, Relations<Matcher>), Leaf<Matcher>>,
    tree: &mut QueryState<Relations<(Ui, Option<HandleOf>)>>,
) {
    for mut target in targets
        .iter(world)
        .map(|target| **target)
        .collect::<Vec<_>>()
        .into_iter()
    {
        todo!()
    }
}
