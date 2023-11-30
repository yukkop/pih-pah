use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::ui::{rich_text, TRANSPARENT};
use crate::util::ResourceAction;

#[derive(Event)]
pub struct MenuEvent(pub ResourceAction);

#[derive(Resource)]
struct State {
    is_active: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_active: false,
        }
    }
}

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MenuEvent>()
            .init_resource::<State>()
            .add_systems(Update, handle_action)
            .add_systems(Update, handle_state);
    }
}

fn handle_action(
    mut reader: EventReader<MenuEvent>,
    mut state: ResMut<State>,
) {
    for MenuEvent(action) in reader.read() {
        match action {
            ResourceAction::Load => {
                state.is_active = true;
            },
            ResourceAction::Unload => {
                state.is_active = false;
            },
        }
    }
}

fn handle_state(
    mut context: EguiContexts,
    state: Res<State>,
) {
    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    if state.is_active {
        egui::Window::new("My UI")
            .frame(*TRANSPARENT)
            .anchor(egui::Align2::LEFT_BOTTOM, [0.0, 0.0])  // Anchor to bottom-left
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .show(ctx, |ui| {
                if ui.button("Button 1").clicked() {
                    // Handle button 1 click
                }
                if ui.button("Button 2").clicked() {
                    // Handle button 2 click
                }
                // Add more buttons as needed
            });
    }
}
