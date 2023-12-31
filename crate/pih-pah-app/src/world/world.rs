use crate::actor::physics_bundle::PhysicsBundle;
use crate::actor::ActorPlugins;
use crate::character::CharacterPlugins;
use crate::component::{AxisName, ComponentPlugins, DespawnReason, NoclipDuration, Respawn};
use crate::lobby::{Inputs, LobbyPlugins, LobbyState, PlayerInputs};
use crate::map::{MapPlugins, SpawnPoint};
use crate::settings::SettingsPlugins;
use crate::sound::SoundPlugins;
use crate::ui::GameMenuActionState;
use crate::ui::{DebugFrameState, DebugMenuEvent, DebugState};
use crate::ui::{MouseGrabState, UiPlugins, UiState};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_xpbd_3d::components::Mass;
use bevy_xpbd_3d::prelude::{Collider, PhysicsLayer, RigidBody};
use serde::{Deserialize, Serialize};

/// Enum representing collision layers for physics interactions.
#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    /// Actors with this layer cannot collide with each other.
    ActorNoclip,
    /// The default collision layer.
    Default,
}

/// A component representing a promised GLTF scene.
///
/// This component is used to temporarily hold a GLTF scene while additional components are added to it.
/// Once processing is complete, it should be removed from the entity.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use pih_pah_app::world::PromisedScene;
///
/// fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
///     let scene = asset_server.load("my_scene.glb#Scene0");
///
///     // Create an entity with the PromisedScene component.
///     commands.spawn((
///         SceneBundle { scene, ..default() },
///         PromisedScene,
///     ));
/// }
///```
#[derive(Component)]
pub struct PromisedScene;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LinkId {
    Scene(String),
    Projectile(usize),
}

#[derive(Resource, Default, Reflect, Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut)]
pub struct ProjectileIdSeq(usize);

impl ProjectileIdSeq {
    /// Returns the next projectile ID. A new ID is generated each time this method is called.
    pub fn shift(&mut self) -> LinkId {
        self.0 += 1;
        LinkId::Projectile(self.0)
    }
}

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectileIdSeq>()
            .register_type::<ProjectileIdSeq>()
            .add_systems(Update, input)
            .add_plugins((
                SettingsPlugins,
                SoundPlugins,
                MapPlugins,
                UiPlugins,
                LobbyPlugins,
                ActorPlugins,
                ComponentPlugins,
                CharacterPlugins,
            ))
            .add_systems(
                Update,
                process_scene.run_if(
                    not(in_state(LobbyState::None)).and_then(not(in_state(LobbyState::Client))),
                ),
            )
            .add_systems(
                Update,
                process_scene_simplified.run_if(in_state(LobbyState::Client)),
            );
    }
}

#[derive(Component)]
pub struct Me;

/// Processes the input keys and manages them from a resource or event deep in the program.
#[allow(clippy::too_many_arguments)]
pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state_debug_frame: ResMut<NextState<DebugFrameState>>,
    debug_frame_state: Res<State<DebugFrameState>>,
    mut next_state_debug: ResMut<NextState<DebugState>>,
    mut debug_menu_togl: EventWriter<DebugMenuEvent>,
    debug_state: Res<State<DebugState>>,
    ui_state: Res<State<UiState>>,
    mut next_state_game_menu_action: ResMut<NextState<GameMenuActionState>>,
    game_menu_action: Res<State<GameMenuActionState>>,
    mut player_input_query: Query<&mut PlayerInputs, With<Me>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut next_state_mouse_grab: ResMut<NextState<MouseGrabState>>,
    mouse_grab_state: Res<State<MouseGrabState>>,
    buttons: Res<Input<MouseButton>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && *ui_state.get() == UiState::GameMenu {
        next_state_game_menu_action.set(game_menu_action.get().clone().toggle());
        next_state_mouse_grab.set(mouse_grab_state.get().clone().toggle());
    }

    if keyboard_input.just_pressed(KeyCode::F8) {
        next_state_debug_frame.set(debug_frame_state.get().clone().toggle());
    }

    if keyboard_input.just_pressed(KeyCode::F9) {
        next_state_debug.set(debug_state.get().clone().toggle());
    }

    if keyboard_input.just_pressed(KeyCode::F10) {
        debug_menu_togl.send(DebugMenuEvent);
    }

    if *game_menu_action.get() == GameMenuActionState::Disable {
        if let Ok(mut player_input) = player_input_query.get_single_mut() {
            let mut turn_horizontal = 0.;
            let mut turn_vertical = 0.;
            for ev in motion_evr.read() {
                turn_horizontal = -ev.delta.x;
                turn_vertical = -ev.delta.y;
            }

            let input = Inputs {
                left: keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left),
                right: keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right),
                up: keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up),
                down: keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down),
                special: keyboard_input.pressed(KeyCode::F),
                jump: keyboard_input.pressed(KeyCode::Space),
                sprint: keyboard_input.pressed(KeyCode::ControlLeft),
                turn_horizontal,
                turn_vertical,
                fire: buttons
                    .get_pressed()
                    .any(|button| *button == MouseButton::Left),
            };

            player_input.insert_inputs(input);
        }
    }
}

/// Processes a promised GLTF scene, adding components as needed and removing the [`PromisedScene`] component from the entity.
///
/// This function recursively traverses the scene hierarchy, processes each entity based on its name and attributes, and adds relevant components.
/// After processing, the [`PromisedScene`] component is removed from the entity.
///
fn process_scene(
    mut commands: Commands,
    scene_query: Query<(Entity, &Children), With<PromisedScene>>,
    parent_query: Query<&Children>,
    name_query: Query<&Name>,
    mesh_handle_query: Query<&Handle<Mesh>>,
    transform_query: Query<&Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, children) in scene_query.iter() {
        for child in children {
            process_scene_child(
                &mut commands,
                *child,
                &parent_query,
                &name_query,
                &mesh_handle_query,
                &transform_query,
                &mut meshes,
            );
        }
        commands.entity(entity).remove::<PromisedScene>();
    }
}

/// Processes a child entity within the GLTF scene hierarchy.
///
/// This function processes a child entity based on its name and attributes, adding relevant components as needed.
/// It is called recursively for each child entity.
fn process_scene_child(
    commands: &mut Commands,
    entity: Entity,
    parent_query: &Query<&Children>,
    name_query: &Query<&Name>,
    mesh_handle_query: &Query<&Handle<Mesh>>,
    transform_query: &Query<&Transform>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    if let Ok(name) = name_query.get(entity) {
        if name.find('[').is_some() {
            let name = name.split('.').next().unwrap_or(name);
            let params = name
                .split('[')
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap()
                .split(';');
            for param in params {
                let mut split = param.split(':');
                let name = split.next().unwrap();
                if let Some(val) = split.next() {
                    if name == "c" {
                        let collider_handler = mesh_handle_query.get(entity).unwrap();
                        if let Some(mesh) = meshes.get(collider_handler) {
                            let collider = Collider::trimesh_from_mesh(mesh).unwrap();
                            commands.entity(entity).insert(collider);

                            if val == "d" {
                                commands
                                    .entity(entity)
                                    .insert(PhysicsBundle::from_rigid_body(RigidBody::Dynamic));
                            }
                            if val == "s" {
                                commands
                                    .entity(entity)
                                    .insert(PhysicsBundle::from_rigid_body(RigidBody::Static));
                            }
                        }
                    } else if name == "id" {
                        commands
                            .entity(entity)
                            .insert(LinkId::Scene(val.to_string()));
                    } else if name == "m" {
                        commands.entity(entity).insert(Mass(val.parse().unwrap()));
                    }
                } else if name == "r" {
                    let transform = transform_query.get(entity).unwrap();
                    commands.entity(entity).insert(Respawn::new(
                        (
                            DespawnReason::More(200., AxisName::Y),
                            DespawnReason::Less(-10., AxisName::Y),
                            DespawnReason::More(100., AxisName::X),
                            DespawnReason::Less(-100., AxisName::X),
                            DespawnReason::More(100., AxisName::Z),
                            DespawnReason::Less(-100., AxisName::Z),
                        ),
                        SpawnPoint::new(transform.translation),
                        NoclipDuration::Timer(10.),
                    ));
                }
            }
        }
    }
    if let Ok(children) = parent_query.get(entity) {
        for child in children {
            process_scene_child(
                commands,
                *child,
                parent_query,
                name_query,
                mesh_handle_query,
                transform_query,
                meshes,
            );
        }
    }
}

/// Processes a promised GLTF scene for a client, adding components as needed and removing the [`PromisedScene`] component from the entity.
///
/// This function recursively traverses the scene hierarchy and processes each entity based on its name and attributes, adding relevant components.
/// After processing, the [`PromisedScene`] component is removed from the entity.
fn process_scene_simplified(
    mut commands: Commands,
    scene_query: Query<(Entity, &Children), With<PromisedScene>>,
    parent_query: Query<&Children>,
    name_query: Query<&Name>,
) {
    for (entity, children) in scene_query.iter() {
        for child in children {
            process_scene_child_simplified(&mut commands, *child, &parent_query, &name_query);
        }
        commands.entity(entity).remove::<PromisedScene>();
    }
}

/// Processes a child entity within the GLTF scene hierarchy for a client.
///
/// This function processes a child entity based on its name and attributes, adding relevant components as needed.
/// It is called recursively for each child entity.
fn process_scene_child_simplified(
    commands: &mut Commands,
    entity: Entity,
    parent_query: &Query<&Children>,
    name_query: &Query<&Name>,
) {
    if let Ok(name) = name_query.get(entity) {
        if name.find('[').is_some() {
            let name = name.split('.').next().unwrap_or(name);
            let params = name
                .split('[')
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap()
                .split(';');
            for param in params {
                let mut split = param.split(':');
                let name = split.next().unwrap();
                if let Some(val) = split.next() {
                    if name == "id" {
                        commands
                            .entity(entity)
                            .insert(LinkId::Scene(val.to_string()));
                    }
                }
            }
        }
    }
    if let Ok(children) = parent_query.get(entity) {
        for child in children {
            process_scene_child_simplified(commands, *child, parent_query, name_query);
        }
    }
}
