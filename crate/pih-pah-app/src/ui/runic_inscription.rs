use std::any::type_name;

use bevy::{
    app::{App, Plugin, Startup, Update},
    core::Name,
    core_pipeline::{
        clear_color::ClearColorConfig,
        core_2d::{Camera2d, Camera2dBundle},
    },
    ecs::{
        entity::Entity,
        query::With,
        schedule::{
            common_conditions::{in_state, not},
            IntoSystemConfigs, State, NextState, States,
        },
        system::{Commands, Query, ResMut, Resource, SystemParamFunction}, event::EventReader, component::Component,
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    log::info,
    math::Vec2,
    prelude::{default, SpatialBundle},
    render::{
        color::Color,
        view::Msaa, camera::Camera,
    },
    transform::components::Transform,
    window::Window, input::mouse::MouseMotion,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    plugin::ShapePlugin,
    shapes, prelude::tess::geom::euclid::default,
};
use bevy_xpbd_3d::components::Position;
use bincode::de;
use egui_dock::window_state;

use crate::{
    lobby::PlayerInputs,
    map::MapState,
    world::Me,
};

const THRESHOLD: f32 = 6.;

use super::MouseGrabState;

#[derive(Default, Debug, Hash, States, PartialEq, Eq, Clone, Copy)]
pub enum InscriptionState{
    #[default]
    Nothing,
    Drawing,
}

#[derive(Component)]
pub struct ActiveLine;

pub struct RunicInscriptionPlugins;

impl Plugin for RunicInscriptionPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_state::<InscriptionState>()
            .insert_resource(Msaa::Sample4)
            .add_plugins(ShapePlugin)
            .init_resource::<RunicInscriptionGrid>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                runic_inscription_grid.run_if(not(in_state(MapState::Menu))),
            )
            .add_systems(Update, runic_inscription.run_if(in_state(InscriptionState::Drawing)));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 10,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
}

#[derive(Default, Resource)]
pub struct RunicInscriptionGrid {
    entity: Option<Entity>,
    grid: Vec<Vec2>,
    rune: Vec<Vec2>,
}

pub fn runic_inscription_grid(
    mut commands: Commands,
    mut windows: Query<&Window>,
    inputs: Query<&PlayerInputs, With<Me>>,
    mut grid_resource: ResMut<RunicInscriptionGrid>,
    mut next_state_mouse_grab: ResMut<NextState<MouseGrabState>>,
    mut next_state_inscription: ResMut<NextState<InscriptionState>>,
) {
    if let Ok(input) = inputs.get_single() {
        if input.get().inscription {
            if let Some(_) = grid_resource.entity {
                return;
            }
            let window = windows.single_mut();
            let window_size = egui::vec2(window.width(), window.height());

            let side_length = 40.0;
            let grid_width = (window_size.x / side_length) as i32 / 3; // Number of points wide
            let grid_height = (window_size.y / side_length) as i32 / 3; // Number of points tall
            let radius = 2.0;

            let dot = shapes::Circle {
                radius,
                ..shapes::Circle::default()
            };

            let height = side_length * (3.0_f32).sqrt() / 2.0; // Height of an equilateral triangle

            let mut points: Vec<Vec2> = Vec::new();

            let grid_half_width = grid_width / 2;
            let grid_half_height = grid_height / 2;

            for row in -grid_half_height..grid_half_height {
                for col in -grid_half_width..grid_half_width {
                    let x = col as f32 * side_length + (row % 2) as f32 * (side_length / 2.0);
                    let y = row as f32 * height;
                    points.push(Vec2 { x, y });
                }
            }

            let entity = commands
                .spawn((Name::new("grid"), SpatialBundle::default()))
                .with_children(|parrent_commands| {
                    for point in points.clone() {
                        parrent_commands.spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&dot),
                                spatial: SpatialBundle {
                                    transform: Transform::from_xyz(point.x, point.y, 0.0),
                                    ..default()
                                },
                                ..default()
                            },
                            Fill::color(Color::CYAN),
                        ));
                    }
                })
                .id();

            grid_resource.entity = Some(entity);
            grid_resource.grid = points;
            grid_resource.rune = vec![];
            next_state_mouse_grab.set(MouseGrabState::Disable);
            next_state_inscription.set(InscriptionState::Drawing);
        } else {
            if let Some(grid_entity) = grid_resource.entity {
                if let Some(entity) = commands.get_entity(grid_entity) {
                    {
                        entity.despawn_recursive();
                        grid_resource.entity = None;
                        next_state_mouse_grab.set(MouseGrabState::Enable);
                        next_state_inscription.set(InscriptionState::Nothing);
                    }
                }
            }
        }
    }
}

pub fn runic_inscription(
    mut commands: Commands,
    mut grid_resource: ResMut<RunicInscriptionGrid>,
    mut motion_evr: EventReader<MouseMotion>,
    windows: Query<&Window>,
    active_line_query: Query<Entity, With<ActiveLine>>,
) {
    if let Ok(window) = windows.get_single() { 
        if let Some(position) = window.physical_cursor_position() {
            let window_size = egui::vec2(window.width(), window.height());
            let position = Vec2::new(position.x - window_size.x / 2.,  window_size.y / 2. - position.y);

            // process active line 
            if let Some(point) = grid_resource.rune.last() {
                for entity in active_line_query.iter() {
                    if let Some(entity_commands) = commands.get_entity(entity) {
                        entity_commands.despawn_recursive();
                    }
                }

                let line = shapes::Line(*point, Vec2::new(position.x, position.y));
                if let Some(entity) = grid_resource.entity {
                    if let Some(mut entity_commands) = commands.get_entity(entity) {
                        entity_commands.with_children(|parent_commands| {
                            parent_commands.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&line),
                                    ..default()
                                }, 
                                Stroke::color(Color::CYAN),
                                ActiveLine,
                                Name::new(type_name::<ActiveLine>()))
                            );
                        });
                    }
                }
            }

            // new point
        if let Some(position) = grid_resource.grid.iter().position(|val| {
                // TODO threshold should be based on window size
                // TODO threshold should be calculated before, in grid_resource
                position.x >= val.x - THRESHOLD && position.x <= val.x + THRESHOLD && position.y >= val.y - THRESHOLD && position.y <= val.y + THRESHOLD
            }) {
                let new_point = grid_resource.grid.remove(position);

                if let Some(last_point) = grid_resource.rune.last() {
                    let line = shapes::Line(*last_point, new_point);
                    if let Some(entity) = grid_resource.entity {
                        if let Some(mut entity_commands) = commands.get_entity(entity) {
                            entity_commands.with_children(|parent_commands| {
                                parent_commands.spawn((
                                    ShapeBundle {
                                        path: GeometryBuilder::build_as(&line),
                                        ..default()
                                    }, 
                                    Stroke::color(Color::CYAN),
                                    Name::new(type_name::<ActiveLine>()))
                                );
                            });
                        }
                    }
                }
                grid_resource.rune.push(new_point.clone());
            }
        }
    }
}