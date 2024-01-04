use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};
use strum_macros::EnumIter;

use crate::{
    option::{ApplyOptions, ExemptOptions},
    util::validate_hash_map,
};

use super::MenuWindow;

/// Menu logic actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum OptionsAction {
    Exempt,
    Close,
    Apply,
    Ok,
}

/// Resource that contains all menu logic actions systems
#[derive(Default, Resource, Deref, DerefMut)]
pub struct OptionsActions(HashMap<OptionsAction, SystemId>);

impl OptionsActions {
    pub fn get(&self, action: OptionsAction) -> SystemId {
        self.0.get(&action).copied().unwrap()
    }
}

pub struct OptionsPlugins;

impl Plugin for OptionsPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<OptionsActions>()
            .add_systems(Startup, register);
    }
}

/// System that runs once at startup to register all menu actions systems
fn register(world: &mut World) {
    let exempt_id = world.register_system(exempt);
    let close_id = world.register_system(close);
    let apply_id = world.register_system(apply);
    let ok_id = world.register_system(ok);

    if let Some(mut options_actions) = world.get_resource_mut::<OptionsActions>() {
        options_actions.insert(OptionsAction::Exempt, exempt_id);
        options_actions.insert(OptionsAction::Close, close_id);
        options_actions.insert(OptionsAction::Apply, apply_id);
        options_actions.insert(OptionsAction::Ok, ok_id);

        // If you see this error, you may add new action in menu_actions
        // or make sure that you have only one MenuAction with the same name in the MenuActions
        assert!(validate_hash_map(&options_actions));
    }
}

/// Exempt `Options` to last applied options
fn exempt(mut options_exempt: EventWriter<ExemptOptions>) {
    options_exempt.send(ExemptOptions);
}

/// Close option window and exempt not applied options
fn close(mut menu_next_state: ResMut<NextState<MenuWindow>>) {
    // SAFETY: `OptionWindow` is the only one window in the game
    // and we can opent it from many states
    // here we close it from any state
    menu_next_state.set(MenuWindow::Empty);
}

/// Apply options
fn apply(mut options_applying: EventWriter<ApplyOptions>) {
    options_applying.send(ApplyOptions);
}

/// Apply and close options window
fn ok(mut commands: Commands, options_action: Res<OptionsActions>) {
    commands.run_system(options_action.get(OptionsAction::Apply));
    commands.run_system(options_action.get(OptionsAction::Close));
}
