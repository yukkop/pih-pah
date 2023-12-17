use std::time::Duration;

use bevy::{ecs::{world::World, entity::Entity}, pbr::{PbrBundle, StandardMaterial}, core::Name, render::{mesh::{Mesh, shape}, color::Color}, asset::Assets, prelude::default};
use bevy::{ecs::system::EntityCommands, prelude::*};
use crate::{extend_commands, component::{DespawnReason, Despawn, DespawnTimer}};

/// Spawn a trasepoint at the actor position.
#[derive(Component)]
pub struct Trace {
  /// tracepoint lifetime
  pub duration: f32,
  /// period of tracepoint spawn
  pub intensity: TraceTimer,
}

impl Trace {
  /// Creates a new `Trace` instance.
  ///
  /// # Arguments
  ///
  /// * `duration` - tracepoint lifetime
  /// * `intensity` - period of tracepoint spawn
  pub fn new(duration: f32, intensity: f32) -> Self {
    Self {
      duration,
      intensity: TraceTimer::new(intensity),
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
  mut query: Query<(&GlobalTransform, &mut Trace)>,
  time: Res<Time>,
) {
  for (global_transform, mut trace) in query.iter_mut() {
    if trace.intensity.update(time.delta()).just_finished() {
        commands.spawn_tracepoint(global_transform.translation(), trace.duration);
    }
  }
}

extend_commands!(
  spawn_tracepoint(translation: Vec3, duration: f32),
  |world: &mut World, entity_id: Entity, translation: Vec3, duration: f32| {
    let mesh = world
        .resource_mut::<Assets<Mesh>>()
        .add(Mesh::try_from(shape::Icosphere { radius: 0.1, subdivisions: 3 }).unwrap());
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::RED.into(),
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