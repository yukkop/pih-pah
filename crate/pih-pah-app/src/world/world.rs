use bevy::prelude::*;
use crate::{province, ui};
use crate::province::ProvincePlugins;
use crate::sound::SoundPlugins;
use crate::ui::UiPlugins;
use crate::util::ResourceAction;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((SoundPlugins, ProvincePlugins, UiPlugins))
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut ui_menu_writer: EventWriter<ui::MenuEvent>,
    mut province_menu_writer: EventWriter<province::MenuEvent>,
) {
    ui_menu_writer.send(ui::MenuEvent(ResourceAction::Load));
    province_menu_writer.send(province::MenuEvent(ResourceAction::Load));
}