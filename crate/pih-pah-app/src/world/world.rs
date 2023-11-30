use bevy::prelude::*;
use crate::{province, ui};
use crate::province::ProvincePlugins;
use crate::sound::SoundPlugins;
use crate::ui::{UiAction, UiPlugins};
use crate::util::ResourceAction;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((SoundPlugins, ProvincePlugins, UiPlugins))
            .add_systems(Startup, setup)
            .add_systems(Update, input);
    }
}

fn setup(
    mut ui_menu_writer: EventWriter<ui::MenuEvent>,
    mut province_menu_writer: EventWriter<province::MenuEvent>,
) {
    ui_menu_writer.send(ui::MenuEvent(ResourceAction::Load));
    province_menu_writer.send(province::MenuEvent(ResourceAction::Load));
}

fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_game_menu_writer: EventWriter<ui::GameMenuEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        ui_game_menu_writer.send(ui::GameMenuEvent(UiAction::Toggle));
    }
}