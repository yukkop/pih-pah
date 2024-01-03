use bevy::prelude::*;
use bevy_editor_pls::{controls, EditorPlugin};

pub struct EditorPlugins;

impl Plugin for EditorPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            // editor for easy debugging https://github.com/jakobhellermann/bevy_editor_pls
            // its included egui plugin and egui_inspector plugin
            EditorPlugin::default(),
        )
        .insert_resource(editor_controls());
    }
}

fn editor_controls() -> controls::EditorControls {
    let mut editor_controls = controls::EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Chord(vec![
                controls::Button::Keyboard(KeyCode::ControlRight),
                controls::Button::Keyboard(KeyCode::E)
            ]),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}