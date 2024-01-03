use std::error;

use bevy::{input::{keyboard::KeyCode, mouse::MouseButton}, utils::HashMap, prelude::{DerefMut, Deref}, ecs::system::Resource};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::mem::discriminant;

use crate::game::GameState;

#[derive(Debug)]
pub enum Button {
    Keyboard(KeyCode),
    Mouse(MouseButton),
    // TODO: add MouseAxis,
}

#[derive(Debug)]
pub enum ButtonCombination {
    /// Single button press
    Single(Button),
    /// Chord is a combination of buttons that must be pressed at the same time
    Chord(Vec<Button>),
}

#[derive(Debug)]
pub enum BindingCondition {
    /// During specific game state
    InGameState(GameState),
    /// Binding is active only if player is in pause menu
    DuringPauseMenu(bool),
    /// Binding is active only if player types text
    ListeningForText(bool),
}

/// Actions that can be performed by player
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, EnumIter)]
pub enum Action {
    /// Move forward
    Forward,
    /// Move backward
    Backward,
    /// Move left
    Left,
    /// Move right
    Right,
    Jump,
}

/// Binding is a combination of buttons that triggers action
#[derive(Debug)]
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

/// Input value that can be used in game logic
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum InputValue {
    #[default]
    Empty,
    Boolean(bool),
    Float(f32),
}

/// Struct that contains user's inputs corresponding to actions
#[derive(Debug, Deref, DerefMut)]
pub struct Inputs(HashMap<Action, InputValue>);

/// Resource that contains current user inputs
#[derive(Debug, Resource)]
pub struct PlayerInputs {
    current: Inputs,
    previous: Inputs,
}

impl Default for PlayerInputs {
    fn default() -> Self {
        PlayerInputs {
            current: Inputs(Action::iter().map(|action| (action, InputValue::Empty)).collect()),
            previous: Inputs(Action::iter().map(|action| (action, InputValue::Empty)).collect()),
        }
    }
}

impl PlayerInputs {
    /// Returns current input value for action
    pub fn get(&self, action: Action) -> InputValue {
        *self.current.get(&action).unwrap()
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
    pub fn forced_set(&mut self, action: Action, value: InputValue) {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        let current_input = self.current.get_mut(&action).unwrap();
        *self.previous.get_mut(&action).unwrap() = *current_input;
        *current_input = value;
    }

    /// Set input value to new 
    /// if it is not the same InputValue type return `Error()`
    pub fn set(&mut self, action: Action, value: InputValue) -> Result<(), Box<dyn error::Error>> {
        // SAFETY: action is always valid
        // because we iterate over all actions in `default` method
        let current_input = self.current.get_mut(&action).unwrap();
        let previos_input = self.previous.get_mut(&action).unwrap();

        // check if enum type is the same ignoring value
        if discriminant(current_input) == discriminant(previos_input) {
            *previos_input = *current_input;
            *current_input = value;
            Ok(())
        }
        else {
            Err("Input type is not the same".into())
        }
    }

    /// Update current inputs
    pub fn forced_update(&mut self, inputs: Inputs) {
        *self.previous = self.current.clone();
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