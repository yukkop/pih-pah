use std::error;

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};
use std::mem::discriminant;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::hashmap;
use crate::util::validate_hash_map;
use crate::{game::GameState, lobby::Lobby};

#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum Button {
    Keyboard(KeyCode),
    Mouse(MouseButton),
    // TODO: add MouseAxis,
}

/// Represents a binding that can be changed by the player
#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum ActionBinding {
    /// Represents a binding that cannot be changed as it is crucial to the game logic
    Immutable(InputMode),
    /// Represents a binding that the player can customize
    Customizable(InputMode),
}

/// Represents the manner in which input is registered
#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum InputMode {
    /// Action is triggered when the button is pressed
    Hold(ButtonCombination),
    /// Action is triggered when the button is pressed
    /// and cannot be triggered again until the button is released
    /// you don't need to release all buttons in chord
    /// you can release only last pressed button
    Tap(ButtonCombination),
}

/// Represents a combination of buttons that triggers an action
#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum ButtonCombination {
    /// Single button press
    Single(Button),
    /// Chord is a combination of buttons that must be pressed at the same time
    Chord(Vec<Button>),
}

#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum BindingCondition {
    /// During specific game state
    InGameState(GameState),
    /// Binding is active only if player is in pause menu
    DuringPauseMenu(bool),
    /// Binding is active only if player types text
    ListeningForText(bool),
}

/// Actions that can be performed by player
#[derive(
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    Debug,
    EnumIter,
    Reflect,
    InspectorOptions,
    Serialize,
    Deserialize,
)]
#[reflect(InspectorOptions)]
pub enum Action {
    /// Move forward
    LeverEditorForward,
    /// Move backward
    LevelEditorBackward,
    /// Move left
    LevelEditorLeft,
    /// Move right
    LevelEditorRight,
    LevelEditorFly,
}

/// Binding is a combination of buttons that triggers action
#[derive(Debug, PartialEq, Clone, Reflect, Serialize, Deserialize)]
pub struct Binding {
    pub input: ActionBinding,
    pub conditions: Vec<BindingCondition>,
}

impl From<ActionBinding> for Binding {
    fn from(input: ActionBinding) -> Self {
        Binding {
            input,
            conditions: Vec::new(),
        }
    }
}

impl Binding {
    pub fn new(input: ActionBinding) -> Self {
        Self {
            input,
            conditions: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: BindingCondition) -> Self {
        self.conditions.push(condition);
        self
    }
}

/// Input value that can be used in game logic
#[derive(Default, Clone, Copy, PartialEq, Debug, Reflect, Serialize, Deserialize)]
pub enum InputValue {
    #[default]
    Empty,
    Boolean(bool),
    Float(f32),
}

impl From<bool> for InputValue {
    fn from(value: bool) -> Self {
        InputValue::Boolean(value)
    }
}

impl From<f32> for InputValue {
    fn from(value: f32) -> Self {
        InputValue::Float(value)
    }
}

impl From<InputValue> for bool {
    fn from(value: InputValue) -> Self {
        match value {
            InputValue::Boolean(value) => value,
            _ => panic!("InputValue is not boolean"),
        }
    }
}

impl From<InputValue> for f32 {
    fn from(value: InputValue) -> Self {
        match value {
            InputValue::Float(value) => value,
            _ => panic!("InputValue is not float"),
        }
    }
}

impl InputValue {
    pub fn is_empty(&self) -> bool {
        matches!(self, InputValue::Empty)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, InputValue::Boolean(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, InputValue::Float(_))
    }

    pub fn as_boolean(&self) -> bool {
        match self {
            InputValue::Boolean(value) => *value,
            InputValue::Float(value) => {
                if *value > 0.0 {
                    true
                } else {
                    false
                }
            }
            InputValue::Empty => false,
        }
    }
}

#[derive(Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub struct BindingConfig {
    pub bindings: Vec<Binding>,
    #[serde(skip)]
    pub delay: f32, // TODO: delay struct
}

impl Default for BindingConfig {
    fn default() -> Self {
        Self {
            bindings: Vec::new(),
            delay: 0.1,
        }
    }
}

impl BindingConfig {
    pub fn new(binding: Vec<Binding>) -> Self {
        Self {
            bindings: binding,
            ..Default::default()
        }
    }

    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }
}

/// Contains all bindings for actions
#[derive(Resource, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(Resource, InspectorOptions)]
// TODO: Add delay for input
pub struct Controls(HashMap<Action, BindingConfig>); // TODO: change HashMap to Vec for faster iteration

impl Default for Controls {
    fn default() -> Self {
        let controls = Self(hashmap! {
            Action::LeverEditorForward => BindingConfig::new(vec![Binding::from(ActionBinding::Immutable(InputMode::Hold(ButtonCombination::Single(Button::Keyboard(KeyCode::W))))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))]),
            Action::LevelEditorBackward => BindingConfig::new(vec![Binding::from(ActionBinding::Immutable(InputMode::Hold(ButtonCombination::Single(Button::Keyboard(KeyCode::S))))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))]),
            Action::LevelEditorLeft => BindingConfig::new(vec![Binding::from(ActionBinding::Immutable(InputMode::Hold(ButtonCombination::Single(Button::Keyboard(KeyCode::A))))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))]),
            Action::LevelEditorRight => BindingConfig::new(vec![Binding::from(ActionBinding::Immutable(InputMode::Hold(ButtonCombination::Single(Button::Keyboard(KeyCode::D))))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))]),
            Action::LevelEditorFly => BindingConfig::new(vec![Binding::from(ActionBinding::Immutable(InputMode::Tap(ButtonCombination::Single(Button::Keyboard(KeyCode::Space))))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))])
        });

        // If you see this error, you may add new action in menu_actions
        // or make sure that you have only one MenuAction with the same name in the MenuActions
        assert!(validate_hash_map(&controls.0));

        controls
    }
}

impl Controls {
    /// Returns bindings for action
    pub fn get(&self, action: Action) -> Option<&BindingConfig> {
        self.0.get(&action)
    }

    /// Push new binding for action
    pub fn push(&mut self, action: Action, binding: Binding) {
        // TODO: warning if config is not exist yet
        self.0
            .entry(action)
            .or_insert_with(BindingConfig::default)
            .bindings
            .push(binding);
    }

    /// Remove all bindings for action
    pub fn remove(&mut self, action: Action) {
        // TODO: warning if config is not exist yet
        self.0
            .entry(action)
            .or_insert_with(BindingConfig::default)
            .bindings
            .clear();
    }

    pub fn remove_binding(&mut self, action: Action, binding: Binding) {
        if let Some(config) = self.0.get_mut(&action) {
            config.bindings.retain(|b| *b == binding);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Action, &BindingConfig)> {
        self.0.iter()
    }
}

/// Struct that contains user's inputs corresponding to actions
#[derive(Debug, PartialEq, Clone, Deref, DerefMut, Reflect)]
pub struct Inputs(HashMap<Action, InputValue>);

/// Resource that contains current user inputs
#[derive(Debug, PartialEq, Clone, Reflect)]
pub struct PlayerInputs {
    current: Inputs,
    previous: Inputs,
}

impl Default for PlayerInputs {
    fn default() -> Self {
        PlayerInputs {
            current: Inputs(
                Action::iter()
                    .map(|action| (action, InputValue::Empty))
                    .collect(),
            ),
            previous: Inputs(
                Action::iter()
                    .map(|action| (action, InputValue::Empty))
                    .collect(),
            ),
        }
    }
}

impl PlayerInputs {
    /// Returns current input value for action
    ///
    /// in any case faster that `get_many` method
    pub fn get(&self, action: Action) -> InputValue {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        *self.current.get(&action).unwrap()
    }

    /// Returns current input values for actions
    pub fn get_many(&self, actions: Vec<Action>) -> Vec<InputValue> {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        actions
            .iter()
            .map(|action| *self.current.get(action).unwrap())
            .collect()
    }

    /// Returns `true` if current `InputValue` has become `InputValue::Boolean(true)` in this frame
    /// If input has not same type as `InputValue` on previous frame call `panic`
    pub fn just_pressed(&self, action: Action) -> bool {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        if let InputValue::Boolean(current) = *self.current.get(&action).unwrap() {
            if let InputValue::Boolean(previous) = *self.previous.get(&action).unwrap() {
                return current && !previous;
            }
            panic!("Previous input is not boolean");
        }
        panic!("This input is not boolean")
    }

    /// Returns `true` if current `InputValue` has become `InputValue::Boolean(true)` in this frame
    /// If input has not same type as `InputValue` on previous frame return `Error()`
    pub fn get_just_pressed(&self, action: Action) -> Result<bool, Box<dyn error::Error>> {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        if let InputValue::Boolean(current) = *self.current.get(&action).unwrap() {
            if let InputValue::Boolean(previous) = *self.previous.get(&action).unwrap() {
                return Ok(current && !previous);
            }
            return Err("Previous input is not boolean".into());
        }
        Err("This input is not boolean".into())
    }

    /// Set input value to new
    pub fn forced_set(&mut self, action: Action, value: impl Into<InputValue>) {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        let current_input = self.current.get_mut(&action).unwrap();
        *self.previous.get_mut(&action).unwrap() = *current_input;
        *current_input = value.into();
    }

    /// Set input value to new
    /// if it is not the same InputValue type return `Error()`
    pub fn set(
        &mut self,
        action: Action,
        value: impl Into<InputValue>,
    ) -> Result<(), Box<dyn error::Error>> {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        let current_input = self.current.get_mut(&action).unwrap();
        let previos_input = self.previous.get_mut(&action).unwrap();

        // check if enum type is the same ignoring value
        if discriminant(current_input) == discriminant(previos_input) {
            *previos_input = *current_input;
            *current_input = value.into();
            Ok(())
        } else {
            Err("Input type is not the same".into())
        }
    }

    /// Update current inputs
    pub fn forced_update(&mut self, inputs: Inputs) {
        *self.previous = self.current.clone().0;
        *self.current = inputs.0;
    }

    /// Update current inputs
    /// if any input is not the same InputValue type return `Error()`
    /// ! remember that it work safely but slow
    pub fn update(&mut self, inputs: Inputs) -> Result<(), Box<dyn error::Error>> {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        for (action, value) in inputs.0.iter() {
            let current_input = self.current.get_mut(action).unwrap();
            let previos_input = self.previous.get_mut(action).unwrap();

            // check if enum type is not the same ignoring value
            if discriminant(current_input) != discriminant(previos_input) {
                return Err("Input type is not the same".into());
            }
            *previos_input = *current_input;
            *current_input = *value;
        }
        Ok(())
    }
}

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
            'bindings_loop: for binding in &config.bindings {
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

                // TODO: match on one condition look like your ugly face
                match &binding.input {
                    ActionBinding::Customizable(mode) | ActionBinding::Immutable(mode) => {
                        match mode {
                            InputMode::Hold(button) | InputMode::Tap(button) => {
                                match button {
                                    ButtonCombination::Single(button) => match button {
                                        Button::Keyboard(key) => {
                                            player
                                                .inputs
                                                .forced_set(*action, keyboard_input.pressed(*key));
                                        }
                                        Button::Mouse(button) => {
                                            player.inputs.forced_set(
                                                *action,
                                                mouse_input.get_pressed().any(|b| b == button),
                                            );
                                        }
                                    },
                                    ButtonCombination::Chord(buttons) => {
                                        // If any button in chord is not pressed we skip this `binding`
                                        for button in buttons {
                                            match button {
                                                Button::Keyboard(key) => {
                                                    if !keyboard_input.pressed(*key) {
                                                        continue 'bindings_loop;
                                                    }
                                                }
                                                Button::Mouse(button) => {
                                                    if !mouse_input
                                                        .get_pressed()
                                                        .any(|b| b == button)
                                                    {
                                                        continue 'bindings_loop;
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
                    }
                }
            }
        }
    } else {
        log::error!("You like [`Player`] is not in lobby")
    }
}

#[cfg(test)]
mod test {
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
