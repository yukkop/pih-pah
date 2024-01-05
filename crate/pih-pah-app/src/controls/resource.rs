use std::error;

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};
use std::mem::discriminant;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::hashmap;
use crate::util::validate_hash_map;
use crate::game::GameState;

#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum InputType {
    Keyboard(KeyCode),
    Mouse(MouseInput),
    // TODO: Gamepad(GamepadButtonType),
    // TODO: Touch screen
}

#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum MouseInput {
    Button(MouseButton),
    Axis(AxisName),
    Wheel(AxisName)
    // TODO: CursorPosition
    // TODO: TouchPad inputs
}

#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum AxisName {
    Horizontal,
    Vertical,
}

/// Represents a binding that can be changed by the player
#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum OptionsMode {
    /// Represents a binding that cannot be changed as it is crucial to the game logic
    Immutable,
    /// Represents a binding that the player can customize
    Customizable,
}

/// Represents the manner in which input is registered
#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum ActivationMode {
    /// Action is triggered when the button is pressed
    Hold,
    /// Action is triggered when the button is pressed
    /// and cannot be triggered again until the button is released
    /// you don't need to release all buttons in chord
    /// you can release only last pressed button
    Tap,
}

#[derive(Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub struct ActivationOptions {
    /// Manner in which input is registered
    pub mode: ActivationMode,
    /// Possibility to change this activation mode
    pub option: OptionsMode,
    /// Delay between activation
    pub delay: f32,
}

impl Default for ActivationOptions {
    fn default() -> Self {
        Self {
            mode: ActivationMode::Tap, // Because it is more safe
            option: OptionsMode::Immutable,
            delay: 0.05, // 20 times per second; 14 - world record?
        }
    }
}

impl ActivationOptions {
    pub fn new(mode: ActivationMode, option: OptionsMode) -> Self {
        Self {
            mode,
            option,
            ..Default::default()
        }
    }

    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }
}

/// Represents a combination of buttons that triggers an action
#[derive(Debug, PartialEq, Clone, Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub enum ButtonCombination {
    /// Single button press
    Single(InputType),
    /// Chord is a combination of buttons that must be pressed at the same time
    Chord(Vec<InputType>),
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
    pub input: ButtonCombination,
    pub conditions: Vec<BindingCondition>,
}

impl From<ButtonCombination> for Binding {
    fn from(input: ButtonCombination) -> Self {
        Binding {
            input,
            conditions: Vec::new(),
        }
    }
}

impl Binding {
    pub fn new(input: ButtonCombination) -> Self {
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
pub struct Bindings {
    /// List of bindings for action
    list: Vec<Binding>,
    /// Possibility to change this binding
    options: OptionsMode,
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            list: Vec::new(),
            options: OptionsMode::Immutable,
        }
    }
}

impl Bindings {
    pub fn new(bindings: Vec<Binding>, options: OptionsMode) -> Self {
        Self {
            list: bindings,
            options,
            ..Default::default()
        }
    }

    pub fn push(&mut self, binding: Binding) {
        if self.options == OptionsMode::Immutable {
            warn!("You try to push binding to immutable bindings");
            return;
        }
        self.list.push(binding);
    }

    pub fn clear(&mut self) {
        if self.options == OptionsMode::Immutable {
            warn!("You try to clear immutable bindings");
            return;
        }
        self.list.clear();
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Binding) -> bool,
    {
        if self.options == OptionsMode::Immutable {
            warn!("You try to retain immutable bindings");
            return;
        }
        self.list.retain(f);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Binding> {
        self.list.iter()
    }

    pub fn iter_mut(&mut self) -> Result<impl Iterator<Item = &mut Binding>, Box<dyn error::Error>> {
        if self.options == OptionsMode::Immutable {
            warn!("You try to iterate over immutable bindings");
            return  Err("You try to iterate over immutable bindings".into());
        }
        Ok(self.list.iter_mut())
    }

    pub fn is_customizable(&self) -> bool {
        self.options == OptionsMode::Customizable
    }

    pub fn is_immutable(&self) -> bool {
        self.options == OptionsMode::Immutable
    }
}

/// Contains all bindings for action
/// and different activation options
#[derive(Reflect, InspectorOptions, Serialize, Deserialize)]
#[reflect(InspectorOptions)]
pub struct BindingConfig {
    pub bindings: Bindings,
    #[serde(skip)]
    pub activation: ActivationOptions,
}

impl Default for BindingConfig {
    fn default() -> Self {
        Self {
            bindings: Bindings::default(),
            activation: ActivationOptions::default(),
        }
    }
}

impl BindingConfig {
    pub fn new(binding: Bindings, activation: ActivationOptions) -> Self {
        Self {
            bindings: binding,
            activation: activation,
            ..Default::default()
        }
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
            Action::LeverEditorForward => BindingConfig::new(
                Bindings::new(
                    vec![Binding::from(ButtonCombination::Single(InputType::Keyboard(KeyCode::W))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))],
                    OptionsMode::Customizable,
                ),
                ActivationOptions::new(ActivationMode::Hold, OptionsMode::Immutable),
            ),
            Action::LevelEditorBackward => BindingConfig::new(
                Bindings::new(
                    vec![Binding::from(ButtonCombination::Single(InputType::Keyboard(KeyCode::S))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))],
                    OptionsMode::Customizable,
                ),
                ActivationOptions::new(ActivationMode::Hold, OptionsMode::Immutable),
            ),
            Action::LevelEditorLeft => BindingConfig::new(
                Bindings::new(
                    vec![Binding::from(ButtonCombination::Single(InputType::Keyboard(KeyCode::A))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))],
                    OptionsMode::Customizable,
                ),
                ActivationOptions::new(ActivationMode::Hold, OptionsMode::Immutable),
            ),
            Action::LevelEditorRight => BindingConfig::new(
                Bindings::new(
                    vec![Binding::from(ButtonCombination::Single(InputType::Keyboard(KeyCode::D))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))],
                    OptionsMode::Customizable,
                ),
                ActivationOptions::new(ActivationMode::Hold, OptionsMode::Immutable),
            ),
            Action::LevelEditorFly => BindingConfig::new(
                Bindings::new(
                    vec![Binding::from(ButtonCombination::Single(InputType::Keyboard(KeyCode::Space))).with_condition(BindingCondition::InGameState(GameState::LevelEditor))],
                    OptionsMode::Customizable,
                ),
                ActivationOptions::new(ActivationMode::Hold, OptionsMode::Customizable),
            )
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