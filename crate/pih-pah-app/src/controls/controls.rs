use bevy::prelude::*;

use crate::{game::GameState, lobby::Lobby};

use super::*;


pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Controls>()
            .register_type::<Controls>()
            .add_systems(Update, save_input);
    }
}

/// Process all hard inputs and bindings to update [`PlayerInputs`]
fn save_input(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut lobby: ResMut<Lobby>,
    controls: Res<Controls>,
    game_state: Res<State<GameState>>,
) {
    let me = lobby.me;
    if let Some(player) = lobby.players.get_mut(&me) {
        for (action, config) in controls.iter() {
            'bindings_loop: for binding in config.bindings.iter() {
                for condition in &binding.conditions {
                    match condition {
                        BindingCondition::InGameState(state) => {
                            if state != game_state.get() {
                                continue 'bindings_loop;
                            }
                        }
                        BindingCondition::DuringPauseMenu(_value) => {
                            todo!();
                        }
                        BindingCondition::ListeningForText(_value) => {
                            todo!();
                        }
                    }
                    break;
                }

                match &binding.input {
                    ButtonCombination::Single(button) => match button {
                        InputType::Keyboard(key) => {
                            player
                                .inputs
                                .forced_set(*action, keyboard_input.pressed(*key));
                        }
                        InputType::Mouse(input) => {
                            match input {
                                MouseInput::Axis(_axis) => {
                                    todo!();
                                }
                                MouseInput::Button(button) => {
                                    if !mouse_input
                                        .get_pressed()
                                        .any(|b| b == button)
                                    {
                                        continue 'bindings_loop;
                                    }
                                }
                                MouseInput::Wheel(_axis) => {
                                    todo!();
                                }
                            }
                        }
                    },
                    ButtonCombination::Chord(buttons) => {
                        // If any button in chord is not pressed we skip this `binding`
                        for button in buttons {
                            match button {
                                InputType::Keyboard(key) => {
                                    if !keyboard_input.pressed(*key) {
                                        continue 'bindings_loop;
                                    }
                                }
                                InputType::Mouse(input) => {
                                    match input {
                                        MouseInput::Axis(_axis) => {
                                            todo!();
                                        }
                                        MouseInput::Button(button) => {
                                            if !mouse_input
                                                .get_pressed()
                                                .any(|b| b == button)
                                            {
                                                continue 'bindings_loop;
                                            }
                                        }
                                        MouseInput::Wheel(_axis) => {
                                            todo!();
                                        }
                                    }
                                }
                            }
                        }

                        // TODO: should be [`Chord`](ButtonCombination::Chord) only [`Boolean`](InputValue::Boolean) type
                        player.inputs.forced_set(*action, true);
                    }
                }
            }
        }
    } else {
        log::error!("You like [`Player`] is not in lobby")
    }
}

#[cfg(test)]
mod performance_test {
    use crate::{
        controls::PlayerInputs,
        util::test::{enable_loggings, measure_time, Times},
    };

    use super::{Action, Controls};
    use std::time::Duration;

    /// Test for execution time for [`Controls`] get
    ///
    /// Example:
    /// ```
    ///     cargo test --package pih-pah-app --lib --features "dev, ui_egui" -- controls::test::controls_get --exact --nocapture
    /// ```
    #[test]
    fn controls_get() {
        enable_loggings();

        let controls = Controls::default();

        let duration = measure_time(
            || {
                controls.get(Action::LeverEditorForward);
                controls.get(Action::LevelEditorBackward);
                controls.get(Action::LevelEditorLeft);
                controls.get(Action::LevelEditorRight);
            },
            Times::default(),
        );

        log::info!("time: {:?}", duration);
    }

    /// Test for execution speed for [`PlayerInputs`] get
    fn player_inputs_get() -> Duration {
        enable_loggings();

        let inputs = PlayerInputs::default();

        let duration = measure_time(
            || {
                inputs.get(Action::LeverEditorForward);
                inputs.get(Action::LevelEditorBackward);
                inputs.get(Action::LevelEditorLeft);
                inputs.get(Action::LevelEditorRight);
            },
            Times::default(),
        );

        duration
    }

    /// Test for execution speed for [`PlayerInputs`] get_many
    fn player_inputs_get_many() -> Duration {
        enable_loggings();

        let inputs = PlayerInputs::default();

        let duration = measure_time(
            || {
                inputs.get_many(vec![
                    Action::LeverEditorForward,
                    Action::LevelEditorBackward,
                    Action::LevelEditorLeft,
                    Action::LevelEditorRight,
                ]);
            },
            Times::default(),
        );

        duration
    }

    /// Test for execution speed for [`PlayerInputs`] get and get_many
    ///
    /// Example:
    /// ```
    ///     cargo test --package pih-pah-app --lib --features "dev, ui_egui" -- controls::test::compare_player_inputs_get_and_get_many --exact --nocapture
    /// ```
    #[test]
    fn compare_player_inputs_get_and_get_many() {
        let get = player_inputs_get();
        let get_many = player_inputs_get_many();

        log::info!("get: {:?}", get);
        log::info!("get_many: {:?}", get_many);
    }

    /// Test for execution speed for [`InputValue`] casting
    #[test]
    fn action_casting() {
        enable_loggings();

        let inputs = PlayerInputs::default();

        let duration = measure_time(
            || {
                let _ = (inputs.get(Action::LeverEditorForward).as_boolean() as i8
                    - inputs.get(Action::LevelEditorBackward).as_boolean() as i8)
                    as f32;
            },
            10000000.into(),
        );

        log::info!("with casting: {:?}", duration);

        let inputs = PlayerInputs::default();

        let duration = measure_time(
            || {
                let _ = inputs.get(Action::LeverEditorForward);
                let _ = inputs.get(Action::LevelEditorBackward);
            },
            10000000.into(),
        );

        log::info!("without casting: {:?}", duration);
    }
}
