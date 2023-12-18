use std::time::Duration;

use crate::{
    component::{Despawn, DespawnReason, DespawnTimer},
    extend_commands,
};
use bevy::{
    asset::Assets,
    core::Name,
    ecs::{entity::Entity, world::World},
    pbr::{PbrBundle, StandardMaterial},
    prelude::default,
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
};
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Spawn a trasepoint at the actor position.
#[derive(Component)]
pub struct Trace {
    /// tracepoint lifetime
    pub duration: f32,
    /// period of tracepoint spawn
    pub intensity: TraceTimer,
    /// tracepoint color
    pub color: Color,
}

impl Trace {
    /// Creates a new `Trace` instance.
    ///
    /// # Arguments
    ///
    /// * `duration` - tracepoint lifetime
    /// * `intensity` - period of tracepoint spawn
    pub fn new(duration: f32, intensity: f32, color: Color) -> Self {
        Self {
            duration,
            intensity: TraceTimer::new(intensity),
            color,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct TraceTimer(Timer);

impl TraceTimer {
    /// Creates a new [`TraceTimer`] with the specified duration.
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }

    /// Updates the timer.
    pub fn update(&mut self, delta: Duration) -> &mut Self {
        self.0.tick(delta);
        self
    }

    /// Returns `true` if the timer has finished.
    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }
}

#[derive(Default, Component)]
pub struct Tracepoint;

pub struct TracePlugins;

impl Plugin for TracePlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_tracepoint);
    }
}

fn process_tracepoint(
    mut commands: Commands,
    mut trace_query: Query<(&GlobalTransform, &mut Trace)>,
    _temp_container_query: Query<Entity, With<super::TempContainer>>,
    time: Res<Time>,
) {
    for (global_transform, mut trace) in trace_query.iter_mut() {
        if trace.intensity.update(time.delta()).just_finished() {
            let _tracepoint = commands.spawn_tracepoint(global_transform.translation(), trace.duration, trace.color).id();
            #[cfg(feature = "temp-container")]
            if let Ok(temp_container) = _temp_container_query.get_single() {
                commands.entity(temp_container).add_child(_tracepoint);
            }
            else {
                warn!("TempContainer not found");
            }
        }
    }
}

extend_commands!(
  spawn_tracepoint(translation: Vec3, duration: f32, color: Color),
  |world: &mut World, entity_id: Entity, translation: Vec3, duration: f32, color: Color| {
    let mesh = world
        .resource_mut::<Assets<Mesh>>()
        .add(Mesh::try_from(shape::Cube {size: 0.2}).unwrap());
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: color,
            ..default()
        });

    world
      .entity_mut(entity_id)
      .insert((
        PbrBundle {
          mesh,
          material,
            transform: Transform::from_translation(translation),
          ..default()
        },
        Name::new("tracepoint"),
        Despawn::new(DespawnReason::After(DespawnTimer::new(duration))),
        Tracepoint,
      ));
  }
);
