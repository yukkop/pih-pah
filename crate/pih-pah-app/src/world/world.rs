use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{Collider, RigidBody};
use crate::{province, ui};
use crate::character::CharacterPlugins;
use crate::lobby::{LobbyPlugins, PlayerInput};
use crate::province::ProvincePlugins;
use crate::sound::SoundPlugins;
use crate::ui::{UiAction, UiPlugins};
use crate::util::ResourceAction;

#[derive(Component)]
pub struct PromisedScene;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((SoundPlugins, ProvincePlugins, UiPlugins, LobbyPlugins, CharacterPlugins))
            .add_systems(Startup, setup)
            .add_systems(Update, (input, process_scene));
    }
}

fn setup(
    mut ui_menu_writer: EventWriter<ui::MenuEvent>,
    mut province_menu_writer: EventWriter<province::MenuEvent>,
) {
    ui_menu_writer.send(ui::MenuEvent(ResourceAction::Load));
    province_menu_writer.send(province::MenuEvent(ResourceAction::Load));
}

#[derive(Component)]
pub struct Me;

fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_game_menu_writer: EventWriter<ui::GameMenuEvent>,
    mut player_input_query: Query<&mut PlayerInput, With<Me>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        ui_game_menu_writer.send(ui::GameMenuEvent(UiAction::Toggle));
    }

    if let Ok(mut player_input) = player_input_query.get_single_mut() {
        player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        player_input.turn_left = keyboard_input.pressed(KeyCode::Q);
        player_input.turn_right = keyboard_input.pressed(KeyCode::E);
        player_input.jump = keyboard_input.just_pressed(KeyCode::Space);
        player_input.sprint = keyboard_input.pressed(KeyCode::ControlLeft);
    }
}

fn process_scene(
    mut commands: Commands,
    scene_query: Query<(Entity, &Children), With<PromisedScene>>,
    parent_query: Query<&Children>,
    name_query: Query<&Name>,
    mesh_handle_query: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, children) in scene_query.iter() {
        for child in children {
            process_scene_child(&mut commands, *child, &parent_query, &name_query, &mesh_handle_query, &mut meshes);
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
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    if let Ok(name) = name_query.get(entity) {
        if name.find("[").is_some() {
            let name = name.split('.').next().unwrap_or(name);
            let params = name.split('[').nth(1).unwrap()
                .split(']').next().unwrap().split(';');
            for param in params {
                let mut split = param.split(":");
                let name = split.next().unwrap();
                let val = split.next().unwrap();
                if name == "c" {
                    let collider_handler = mesh_handle_query.get(entity).unwrap();
                    if let Some(mesh) = meshes.get(collider_handler) {
                        let collider = Collider::trimesh_from_mesh(mesh).unwrap();
                        commands.entity(entity).insert(collider);

                        if val == "d" {
                            let mut commands_entity = commands.entity(entity);
                            commands_entity.insert(RigidBody::Dynamic);
                        }
                        if val == "s" {
                            let mut commands_entity = commands.entity(entity);
                            commands_entity.insert(RigidBody::Static);
                        }
                    }
                }
            }
        }
    }
    if let Ok(children) = parent_query.get(entity) {
        for child in children {
            process_scene_child(commands, *child, parent_query, name_query, mesh_handle_query, meshes);
        }
    }
}
