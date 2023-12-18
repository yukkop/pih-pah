use std::any::type_name;

use bevy::{app::{Plugin, App, Startup}, ecs::{component::Component, system::Commands}, transform::{components::{Transform, GlobalTransform}}, core::Name, render::view::{Visibility, InheritedVisibility, ViewVisibility}};

use super::{CharacterPlugins, TracePlugins};

#[derive(Default, Component)]
pub struct TempContainer;

pub struct ActorPlugins;

impl Plugin for ActorPlugins {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "temp-container")]
        app .add_systems(Startup, setup);
        app .add_plugins((
                CharacterPlugins,
                TracePlugins

            ));
    }
}

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
        Name::new(short_name)
    ));
}