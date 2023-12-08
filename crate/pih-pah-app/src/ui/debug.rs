use std::any::TypeId;

use bevy::asset::{ReflectAsset, UntypedAssetId};
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy::render::camera::{CameraProjection, Viewport};
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiSet};
use bevy_inspector_egui::bevy_inspector::hierarchy::{hierarchy_ui, SelectedEntities};
use bevy_inspector_egui::bevy_inspector::{
    self, ui_for_entities_shared_components, ui_for_entity_with_children,
};
use egui::{Align2, Pos2};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use egui_gizmo::{Gizmo, GizmoMode, GizmoOrientation};

use crate::util::i18n::Uniq;

use super::{rich_text, MainCamera, ViewportRect};

lazy_static::lazy_static! {
    static ref MODULE: &'static str = module_path!().splitn(3, ':').nth(2).unwrap_or(module_path!());
}

#[derive(Default, Debug, Hash, States, PartialEq, Eq, Clone, Copy)]
pub enum DebugFrameState {
    Enable,
    #[default]
    Disable,
}

impl DebugFrameState {
    pub fn toggle(&mut self) -> Self {
        match self {
            DebugFrameState::Enable => *self = DebugFrameState::Disable,
            DebugFrameState::Disable => *self = DebugFrameState::Enable,
        }
        *self
    }
}

#[derive(Default, Debug, Hash, States, PartialEq, Eq, Clone, Copy)]
pub enum DebugState {
    #[default]
    Enable,
    Disable,
}

impl DebugState {
    pub fn toggle(&mut self) -> Self {
        match self {
            DebugState::Enable => *self = DebugState::Disable,
            DebugState::Disable => *self = DebugState::Enable,
        }
        *self
    }
}

#[derive(Debug, Default, Event)]
pub struct DebugMenuEvent;

#[derive(Default, Debug, Hash, States, PartialEq, Eq, Clone, Copy)]
enum DebugMenuState {
    Enable,
    #[default]
    Disable,
}

pub struct DebugUiPlugins;

impl Plugin for DebugUiPlugins {
    fn build(&self, app: &mut App) {
        app.add_event::<DebugMenuEvent>()
            .add_state::<DebugFrameState>()
            .add_state::<DebugState>()
            .add_state::<DebugMenuState>();
        let is_debug = std::env::var("DEBUG").is_ok();

        if is_debug {
            app.insert_resource(UiState::new())
                .add_systems(
                    PostUpdate,
                    show_ui_system
                        .run_if(in_state(DebugState::Enable))
                        .before(EguiSet::ProcessOutput)
                        .before(bevy::transform::TransformSystem::TransformPropagate),
                )
                .add_systems(
                    PostUpdate,
                    set_camera_viewport
                        .run_if(in_state(DebugState::Enable))
                        .after(show_ui_system),
                )
                .add_systems(
                    Update,
                    (set_gizmo_mode, push_window_menu_event).run_if(in_state(DebugState::Enable)),
                )
                .add_systems(
                    Update,
                    push_window_menu.run_if(
                        in_state(DebugState::Enable).and_then(in_state(DebugMenuState::Enable)),
                    ),
                )
                .add_systems(
                    OnEnter(DebugState::Disable),
                    (exit_debug, set_camera_viewport.after(exit_debug)),
                )
                .register_type::<Option<Handle<Image>>>()
                .add_systems(OnEnter(DebugFrameState::Enable), debug_frame_enable)
                .add_systems(OnEnter(DebugFrameState::Disable), debug_frame_disable)
                .register_type::<AlphaMode>();
        }
    }
}

fn push_window_menu_event(
    mut next_state_debug_menu: ResMut<NextState<DebugMenuState>>,
    mut debug_menu_event: EventReader<DebugMenuEvent>,
    windows: Query<&Window>,
    mut ui_state: ResMut<UiState>,
) {
    for _ in debug_menu_event.read() {
        if let Some(position) = windows.single().physical_cursor_position() {
            ui_state.menu_pos = position;
        } else {
            let window = windows.single();
            let window_size = egui::vec2(window.width(), window.height());
            ui_state.menu_pos = Vec2::new(window_size.x / 2.0, window_size.y / 2.0);
        }
        next_state_debug_menu.set(DebugMenuState::Enable);
    }
}

fn push_window_menu(
    mut next_state_debug_menu: ResMut<NextState<DebugMenuState>>,
    mut ui_state: ResMut<UiState>,
    mut context: EguiContexts,
    windows: Query<&Window>,
) {
    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    let ctx = context.ctx_mut();
    egui::Window::new("Mouse moved")
        .anchor(
            Align2::LEFT_TOP,
            [ui_state.menu_pos.x - 10., ui_state.menu_pos.y - 35.],
        )
        .collapsible(false)
        .movable(false)
        .show(ctx, |ui| {
            let new_window = {
                if ui
                    .button(rich_text(
                        "ViewPort".to_string(),
                        Uniq::Module(&MODULE),
                        &font,
                    ))
                    .clicked()
                {
                    Some(EguiWindow::GameView)
                } else if ui
                    .button(rich_text(
                        "Hierarchy".to_string(),
                        Uniq::Module(&MODULE),
                        &font,
                    ))
                    .clicked()
                {
                    Some(EguiWindow::Hierarchy)
                } else if ui
                    .button(rich_text(
                        "Resources".to_string(),
                        Uniq::Module(&MODULE),
                        &font,
                    ))
                    .clicked()
                {
                    Some(EguiWindow::Resources)
                } else if ui
                    .button(rich_text(
                        "Assets".to_string(),
                        Uniq::Module(&MODULE),
                        &font,
                    ))
                    .clicked()
                {
                    Some(EguiWindow::Assets)
                } else if ui
                    .button(rich_text(
                        "Inspector".to_string(),
                        Uniq::Module(&MODULE),
                        &font,
                    ))
                    .clicked()
                {
                    Some(EguiWindow::Inspector)
                } else {
                    None
                }
            };

            if let Some(new_window) = new_window {
                ui_state.new_window(new_window);
            }

            // TODO this is clip more that we need
            let rect = ui.clip_rect();
            if let Some(position) = windows.single().cursor_position() {
                if !rect.contains(Pos2 {
                    x: position.x,
                    y: position.y,
                }) {
                    next_state_debug_menu.set(DebugMenuState::Disable);
                }
            } else {
                next_state_debug_menu.set(DebugMenuState::Disable);
            }
        });
}

fn exit_debug(mut viewport_rect: ResMut<ViewportRect>, windows: Query<&Window>) {
    let window = windows.single();
    let window_size = egui::vec2(window.width(), window.height());
    viewport_rect.set(egui::Rect::from_min_size(Default::default(), window_size));
}

fn debug_frame_disable(mut context: EguiContexts) {
    context.ctx_mut().set_style(egui::Style {
        debug: egui::style::DebugOptions {
            debug_on_hover: false,
            ..default()
        },
        ..default()
    });
}

fn debug_frame_enable(mut context: EguiContexts) {
    context.ctx_mut().set_style(egui::Style {
        debug: egui::style::DebugOptions {
            debug_on_hover: true,
            ..default()
        },
        ..default()
    });
}

fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

/// make camera only render to view not obstructed by UI
fn set_camera_viewport(
    viewport_rect: Res<ViewportRect>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
) {
    if let Ok(mut cam) = cameras.get_single_mut() {
        let Ok(window) = primary_window.get_single() else {
            return;
        };

        let scale_factor = window.scale_factor() * egui_settings.scale_factor;

        let viewport_pos = viewport_rect.left_top().to_vec2() * scale_factor as f32;
        let viewport_size = viewport_rect.size() * scale_factor as f32;

        cam.viewport = Some(Viewport {
            physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
            physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
            depth: 0.0..1.0,
        });
    }
}

fn set_gizmo_mode(input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    for (key, mode) in [
        (KeyCode::R, GizmoMode::Rotate),
        (KeyCode::G, GizmoMode::Translate),
        (KeyCode::S, GizmoMode::Scale),
    ] {
        if input.just_pressed(key) {
            ui_state.gizmo_mode = mode;
        }
    }
}

#[derive(Eq, PartialEq)]
enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, UntypedAssetId),
}

#[derive(Resource)]
struct UiState {
    state: DockState<EguiWindow>,
    selected_entities: SelectedEntities,
    selection: InspectorSelection,
    gizmo_mode: GizmoMode,
    /// Position of the context menu (where was mouse)
    menu_pos: Vec2,
}

impl UiState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![EguiWindow::GameView]);
        let tree = state.main_surface_mut();
        let [game, _inspector] =
            tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![EguiWindow::Hierarchy]);
        let [_game, _bottom] =
            tree.split_below(game, 0.8, vec![EguiWindow::Resources, EguiWindow::Assets]);

        Self {
            state,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            gizmo_mode: GizmoMode::Translate,
            menu_pos: Vec2::default(),
        }
    }

    pub fn new_window(&mut self, window: EguiWindow) {
        let tree = self.state.main_surface_mut();
        tree.push_to_focused_leaf(window);
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
            gizmo_mode: self.gizmo_mode,
        };
        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

#[derive(Debug)]
enum EguiWindow {
    GameView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
}

struct TabViewer<'a> {
    world: &'a mut World,
    selected_entities: &'a mut SelectedEntities,
    selection: &'a mut InspectorSelection,
    gizmo_mode: GizmoMode,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            EguiWindow::GameView => {
                self.world
                    .resource_mut::<ViewportRect>()
                    .set(ui.clip_rect());

                draw_gizmo(ui, self.world, self.selected_entities, self.gizmo_mode);
            }
            EguiWindow::Hierarchy => {
                let selected = hierarchy_ui(self.world, ui, self.selected_entities);
                if selected {
                    *self.selection = InspectorSelection::Entities;
                }
            }
            EguiWindow::Resources => select_resource(ui, &type_registry, self.selection),
            EguiWindow::Assets => select_asset(ui, &type_registry, self.world, self.selection),
            EguiWindow::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[entity] => ui_for_entity_with_children(self.world, entity, ui),
                    entities => ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_resource(
                        self.world,
                        type_id,
                        ui,
                        name,
                        &type_registry,
                    )
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_asset(
                        self.world,
                        type_id,
                        handle,
                        ui,
                        &type_registry,
                    );
                }
            },
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::GameView)
    }
}

fn draw_gizmo(
    ui: &mut egui::Ui,
    world: &mut World,
    selected_entities: &SelectedEntities,
    gizmo_mode: GizmoMode,
) {
    if let Ok((cam_transform, projection)) = world
        .query_filtered::<(&GlobalTransform, &Projection), With<MainCamera>>()
        .get_single(world)
    {
        let view_matrix = Mat4::from(cam_transform.affine().inverse());
        let projection_matrix = projection.get_projection_matrix();

        if selected_entities.len() != 1 {
            return;
        }

        for selected in selected_entities.iter() {
            let Some(transform) = world.get::<Transform>(selected) else {
                continue;
            };
            let model_matrix = transform.compute_matrix();

            let Some(result) = Gizmo::new(selected)
                .model_matrix(model_matrix.to_cols_array_2d())
                .view_matrix(view_matrix.to_cols_array_2d())
                .projection_matrix(projection_matrix.to_cols_array_2d())
                .orientation(GizmoOrientation::Local)
                .mode(gizmo_mode)
                .interact(ui)
            else {
                continue;
            };

            let mut transform = world.get_mut::<Transform>(selected).unwrap();
            *transform = Transform {
                translation: Vec3::from(<[f32; 3]>::from(result.translation)),
                rotation: Quat::from_array(<[f32; 4]>::from(result.rotation)),
                scale: Vec3::from(<[f32; 3]>::from(result.scale)),
            };
        }
    }
}

fn select_resource(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    selection: &mut InspectorSelection,
) {
    let mut resources: Vec<_> = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| {
            (
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
            )
        })
        .collect();
    resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

    for (resource_name, type_id) in resources {
        let selected = match *selection {
            InspectorSelection::Resource(selected, _) => selected == type_id,
            _ => false,
        };

        if ui.selectable_label(selected, resource_name).clicked() {
            *selection = InspectorSelection::Resource(type_id, resource_name.to_string());
        }
    }
}

fn select_asset(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    world: &World,
    selection: &mut InspectorSelection,
) {
    let mut assets: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<ReflectAsset>()?;
            Some((
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
                reflect_asset,
            ))
        })
        .collect();
    assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    for (asset_name, asset_type_id, reflect_asset) in assets {
        let handles: Vec<_> = reflect_asset.ids(world).collect();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
                    _ => false,
                };

                if ui
                    .selectable_label(selected, format!("{:?}", handle))
                    .clicked()
                {
                    *selection =
                        InspectorSelection::Asset(asset_type_id, asset_name.to_string(), handle);
                }
            }
        });
    }
}
