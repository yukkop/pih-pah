use std::{any::type_name, f32::consts::PI};

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
            IntoSystemConfigs, State, NextState, States, OnExit, OnEnter,
        },
        system::{Commands, Query, ResMut, Resource, SystemParamFunction}, event::EventReader, component::Component,
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    log::info,
    math::Vec2,
    prelude::{default, SpatialBundle},
    render::{
        color::Color,
        view::Msaa, camera::Camera, mesh::shape::Circle,
    },
    transform::{components::Transform, commands},
    window::Window, input::mouse::MouseMotion, text::YAxisOrientation,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    plugin::ShapePlugin,
    shapes, prelude::tess::geom::euclid::default,
};
use bevy_xpbd_3d::{components::Position, parry::query::point};
use bincode::de;
use egui_dock::window_state;

use crate::{
    lobby::PlayerInputs,
    map::MapState,
    world::Me,
};

const DOT_RADIUS: f32 = 2.0;
const THRESHOLD: f32 = 7.;
const CONNECTION_RADIUS: f32 = THRESHOLD + DOT_RADIUS;
const SIDE_LENGTH_FACTOR: f32 = 0.03;
const DOT: shapes::Circle = shapes::Circle {
    radius: DOT_RADIUS,
    center: Vec2::ZERO,
};

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
            .add_systems(Update, runic_inscription.run_if(in_state(InscriptionState::Drawing)))
            .add_systems(OnEnter(InscriptionState::Nothing), teardown);
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
    grid: Vec<GridPoint>,
    rune: Vec<GridPoint>,
    // TODO static is ok?
    last_point: Option<Entity>,
}

impl RunicInscriptionGrid {
    fn renew(&mut self, grid: Vec<GridPoint>, entity: Entity, firs_point: GridPoint) {
        self.grid = grid;
        self.entity = Some(entity);
        self.rune = vec![firs_point];
        self.last_point = Some(firs_point.entity);
    }

    fn update(&mut self, grid: Vec<GridPoint>) {
        self.grid = grid;
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct GridPoint {
    entity: Entity,
    position: Vec2,
}

impl GridPoint {
    fn new(entity: Entity, position: Vec2) -> Self {
        Self {
            entity,
            position,
        }
    }
}

fn hexagon(
    mut already_exists: Vec<GridPoint>,
    rune: &Vec<GridPoint>,
    parent_commands: &mut bevy::prelude::ChildBuilder<'_, '_, '_>,
    center: Vec2,
    side_length: f32,
) -> (Vec<GridPoint>, Vec<GridPoint>) {
    let mut points = vec![];
    for i in 0..6 {
        let angle = PI / 3.0 * i as f32; // 60 degrees * i
        let x = center.x + side_length * angle.cos();
        let y = center.y + side_length * angle.sin();
        if let Some(grid_point_position) = already_exists.iter().position(|val| {
            x >= val.position.x - THRESHOLD && x <= val.position.x + THRESHOLD && y >= val.position.y - THRESHOLD && y <= val.position.y + THRESHOLD
        }) {
            let old = already_exists.remove(grid_point_position);
            points.push(old);
            continue;
        }

        if rune.iter().any(|val| {
            x >= val.position.x - THRESHOLD && x <= val.position.x + THRESHOLD && y >= val.position.y - THRESHOLD && y <= val.position.y + THRESHOLD
        }) {
            info!("runes: {:?}", rune);
            continue;
        }

        points.push(GridPoint::new(create_node(parent_commands, x, y), Vec2::new( x, y )));
    }
    (points, already_exists)
}

fn create_node(
    parent_commands: &mut bevy::prelude::ChildBuilder<'_, '_, '_>,
    x: f32,
    y: f32,
) -> Entity {
    parent_commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&DOT),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            ..default()
        },
        Fill::color(Color::CYAN),
    )).id()
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
            let window = windows.single_mut();
            let window_size = egui::vec2(window.width(), window.height());

            let side_length = window_size.x * SIDE_LENGTH_FACTOR;

            let mut points: Vec<GridPoint> = Vec::new();

            let center = Vec2::new(0., 0.);

            // if grid already rendered
            if let Some(entity) = grid_resource.entity {
                let last_rune_point = grid_resource.rune.last().copied();

                // if rune get a new point draw new hex, remove old points and update `last_point`
                if grid_resource.last_point.is_some() && last_rune_point.is_some() && grid_resource.last_point.unwrap() != last_rune_point.unwrap().entity 
                {
                    let mut points_to_remove = vec![];
                    commands
                        .entity(entity)
                        .with_children(|parent_commands: &mut bevy::prelude::ChildBuilder<'_, '_, '_>| {
                            let new_points;
                            (new_points, points_to_remove) = hexagon(grid_resource.grid.clone(), &grid_resource.rune, parent_commands, last_rune_point.unwrap().position, side_length);
                            points.extend(new_points);
                        });
                    for point in points_to_remove {
                        commands.entity(point.entity).despawn_recursive();
                    }
                    grid_resource.update(points);
                    grid_resource.last_point = last_rune_point.unwrap().entity.into();
                } 
                return;
            }
            // let height = side_length * (3.0_f32).sqrt() / 2.0; // Height of an equilateral triangle

            let mut firs_point: Option<GridPoint> = None;
            let entity = commands
                .spawn((Name::new("Grid"), SpatialBundle::default()))
                .with_children(|parent_commands: &mut bevy::prelude::ChildBuilder<'_, '_, '_>| {
                    firs_point = Some(GridPoint::new(create_node(parent_commands, center.x, center.y), center));
                    let (new_points, _) = hexagon(vec![], &grid_resource.rune, parent_commands, center, side_length);
                    points.extend(new_points);
                })
                .id();

            grid_resource.renew(points, entity, firs_point.unwrap());
            next_state_mouse_grab.set(MouseGrabState::Disable);
            next_state_inscription.set(InscriptionState::Drawing);
        } else {
            if grid_resource.entity.is_some() {
                next_state_mouse_grab.set(MouseGrabState::Enable);
                next_state_inscription.set(InscriptionState::Nothing);
            }
        }
    }
}

pub fn teardown(
    mut commands: Commands,
    mut grid_resource: ResMut<RunicInscriptionGrid>,
) {
    if let Some(grid_entity) = grid_resource.entity {
        if let Some(entity) = commands.get_entity(grid_entity) {
            {
                entity.despawn_recursive();
                grid_resource.entity = None;
                grid_resource.rune = vec![];
            }
        }
    }
}

pub fn runic_inscription(
    mut commands: Commands,
    mut grid_resource: ResMut<RunicInscriptionGrid>,
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

                let line = shapes::Line(point.position, Vec2::new(position.x, position.y));
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
                position.x >= val.position.x - CONNECTION_RADIUS && position.x <= val.position.x + CONNECTION_RADIUS && position.y >= val.position.y - THRESHOLD && position.y <= val.position.y + THRESHOLD
            }) {
                let new_point = grid_resource.grid.remove(position);

                if let Some(last_point) = grid_resource.rune.last() {
                    let line = shapes::Line(last_point.position, new_point.position);
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
                grid_resource.rune.push(new_point);
            }
        }
    }
}