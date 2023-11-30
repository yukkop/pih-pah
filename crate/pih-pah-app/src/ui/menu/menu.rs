use bevy::prelude::*;
use crate::util::ResourceAction;

#[derive(Event)]
pub struct MenuEvent(pub ResourceAction);

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MenuEvent>()
            .add_systems(Update, handle_action);
    }
}

fn handle_action(mut reader: EventReader<MenuEvent>) {
    for MenuEvent(action) in reader.read() {
        match action {
            ResourceAction::Load => {
                load();
            },
            ResourceAction::Unload => {
                unload();
            },
        }
    }
}

fn load() {

}

fn unload() {

}
