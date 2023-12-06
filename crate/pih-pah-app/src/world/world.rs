use crate::character::CharacterPlugins;
use crate::component::{ComponentPlugins, Respawn};
use crate::load::LoadPlugins;
use crate::lobby::{LobbyPlugins, LobbyState, PlayerInput};
use crate::province::ProvincePlugins;
use crate::settings::SettingsPlugins;
use crate::sound::SoundPlugins;
use crate::ui::{GameMenuActionState, MouseGrabState};
use crate::ui::UiPlugins;
use bevy::prelude::*;
use bevy_xpbd_3d::components::{CollisionLayers, Mass};
use bevy_xpbd_3d::prelude::{Collider, PhysicsLayer, RigidBody};
use serde::{Deserialize, Serialize};

#[derive(PhysicsLayer)]
pub enum MyLayers {
    /// Cannot touch each other
    ActorNoclip,
    Default,
}

#[derive(Component)]
pub struct PromisedScene;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkId(String);

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoadPlugins,
            SettingsPlugins,
            SoundPlugins,
            ProvincePlugins,
            UiPlugins,
            LobbyPlugins,
            CharacterPlugins,
            ComponentPlugins,
        ))
        .add_systems(Update, input)
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

fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state_game_menu_action: ResMut<NextState<GameMenuActionState>>,
    mut nex_state_mouse_grab: ResMut<NextState<MouseGrabState>>,
    game_menu_action: Res<State<GameMenuActionState>>,
    mouse_grab_state: Res<State<MouseGrabState>>,
    mut player_input_query: Query<&mut PlayerInput, With<Me>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state_game_menu_action.set(game_menu_action.get().clone().toggle());
        nex_state_mouse_grab.set(mouse_grab_state.get().clone().toggle());
    }

    if let Ok(mut player_input) = player_input_query.get_single_mut() {
        player_input.left =
            keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        player_input.right =
            keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        player_input.down =
            keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        player_input.turn_left = keyboard_input.pressed(KeyCode::Q);
        player_input.turn_right = keyboard_input.pressed(KeyCode::E);
        player_input.jump = keyboard_input.just_pressed(KeyCode::Space);
        player_input.sprint = keyboard_input.pressed(KeyCode::ControlLeft);
    }
}

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
                        commands.entity(entity).insert(LinkId(val.to_string()));
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
                            commands
                                .entity(entity)
                                .insert(collider)
                                .insert(CollisionLayers::new(
                                    [MyLayers::Default],
                                    [MyLayers::Default, MyLayers::ActorNoclip],
                                ));

                            if val == "d" {
                                let mut commands_entity = commands.entity(entity);
                                commands_entity.insert(RigidBody::Dynamic);
                            }
                            if val == "s" {
                                let mut commands_entity = commands.entity(entity);
                                commands_entity.insert(RigidBody::Static);
                            }
                        }
                    } else if name == "id" {
                        commands.entity(entity).insert(LinkId(val.to_string()));
                    } else if name == "m" {
                        commands.entity(entity).insert(Mass(val.parse().unwrap()));
                    }
                } else if name == "r" {
                    let transform = transform_query.get(entity).unwrap();
                    commands
                        .entity(entity)
                        .insert(Respawn::from_vec3(transform.translation));
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
