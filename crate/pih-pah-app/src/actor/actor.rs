use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query},
    },
    hierarchy::DespawnRecursiveExt,
};

#[cfg(feature = "temp-container")]
use {
    bevy::{
        app::Startup,
        core::Name,
        render::view::{InheritedVisibility, ViewVisibility, Visibility},
        transform::components::{GlobalTransform, Transform},
    },
    std::any::type_name,
};

use super::TracePlugins;

#[derive(Default, Component)]
pub struct Actor;

#[derive(Default, Component)]
pub struct TempContainer;

#[derive(Event)]
pub struct UnloadActorsEvent;

pub struct ActorPlugins;

impl Plugin for ActorPlugins {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "temp-container")]
        app.add_systems(Startup, setup);
        app.add_event::<UnloadActorsEvent>()
            .add_plugins(TracePlugins)
            .add_systems(Update, unload_actors);
    }
}

// TODO on state it will be faster
fn unload_actors(
    mut commands: Commands,
    actor_query: Query<Entity, With<Actor>>,
    mut event: EventReader<UnloadActorsEvent>,
) {
    for _ in event.read() {
        log::info!("UnloadActorsEvent");
        for entity in actor_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[cfg(feature = "temp-container")]
fn setup(mut commands: Commands) {
    let full_name = type_name::<TempContainer>();
    let short_name = full_name.split("::").last().unwrap_or(full_name);

    commands.spawn((
        TempContainer,
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Name::new(short_name),
    ));
}
