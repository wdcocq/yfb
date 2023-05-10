use std::fmt::{self, Display, Formatter};

use yew::AttrValue;

use crate::{
    hooks::UseGenerationHandle,
    model::{Dirty, State, Value},
    modifier::{BaseModifier, Modifier},
    state_model::StateModelRc,
};

/// Contains the current state of a [`Model`](crate::model::Model)'s field
#[derive(PartialEq)]
pub struct Field {
    initial: Option<AttrValue>,
    value: AttrValue,
    message: Option<AttrValue>,
    error: Option<AttrValue>,
    generation: usize,
    generation_handle: UseGenerationHandle,
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Field")
            .field("initial", &self.initial.as_ref().map(|i| i.as_str()))
            .field("value", &self.value.as_str())
            .field("message", &self.message.as_ref().map(|m| m.as_str()))
            .field("error", &self.error.as_ref().map(|e| e.as_str()))
            .field("generation", &self.generation)
            .field("generation_handle", &self.generation_handle)
            .finish()
    }
}

impl Field {
    /// Returns whether the field passed validation
    pub fn valid(&self) -> bool {
        self.message.is_none()
    }

    /// Returns the current value of the field
    pub fn value(&self) -> &AttrValue {
        &self.value
    }

    pub(crate) fn set_value(&mut self, value: AttrValue) {
        if value != self.value {
            match self.initial.as_ref() {
                // Clone back the initial value so the new value can be dropped when not needed further
                Some(initial) if *initial == value => {
                    self.value = initial.clone();
                    self.message = None;
                }
                _ => self.value = value,
            }
            self.generation = self.generation_handle.increase();
        }
    }

    /// Returns `Some(..)` if the field did not pass validation, or else `None`
    pub fn message(&self) -> Option<&AttrValue> {
        self.message.as_ref()
    }

    pub(crate) fn set_message(&mut self, message: Option<AttrValue>) {
        if self.message != message {
            self.message = message;
            self.generation = self.generation_handle.increase();
        }
    }

    /// Returns `Some(..)` when the field was unable to parse the raw string input, or else `None`
    pub fn error(&self) -> Option<&AttrValue> {
        self.error.as_ref()
    }

    pub(crate) fn set_error(&mut self, error: Option<AttrValue>) {
        if self.error != error {
            self.error = error;
            self.generation = self.generation_handle.increase();
        }
    }
}

impl<T> State<T> for Field
where
    T: Value,
{
    fn create(value: &T, with_initial: bool, generation: UseGenerationHandle) -> Self {
        let value = value.to_value();

        Field {
            initial: with_initial.then(|| value.clone()),
            value,
            generation: generation.generation(),
            generation_handle: generation,
            message: Default::default(),
            error: Default::default(),
        }
    }

    fn update(&mut self, model: &T) {
        self.set_value(model.to_value());
    }

    fn generation(&self) -> usize {
        self.generation
    }
}

impl Dirty for Field {
    fn dirty(&self) -> bool {
        match self.initial.as_ref() {
            Some(initial) => *initial != self.value,
            None => !self.value.is_empty(),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct FieldModifier<T>(BaseModifier<T>)
where
    T: Value;

impl<T> Modifier<T> for FieldModifier<T>
where
    T: Value,
{
    fn create(state_model: StateModelRc<T>) -> Self {
        Self(BaseModifier(state_model))
    }

    fn state_model(&self) -> &StateModelRc<T> {
        self.0.state_model()
    }

    fn set_message(&self, message: Option<AttrValue>) {
        let (_, mut state) = self.state_model().as_mut();
        state.set_message(message);
    }
}

impl<T> FieldModifier<T>
where
    T: Value,
{
    pub fn set(&self, value: impl Into<AttrValue>) {
        let (mut model, mut state) = self.state_model().as_mut();
        let value = value.into();

        match T::from_value(&value) {
            Ok(t) => {
                *model = t;
                state.set_value(value);
            }
            Err(_) => {
                state.set_error(Some(
                    format!(
                        "Invalid value '{}' for type {}",
                        value,
                        std::any::type_name::<T>()
                    )
                    .into(),
                ));
            }
        }
    }

    pub fn set_value(&self, value: T) {
        let (mut model, mut state) = self.state_model().as_mut();
        state.set_value(value.to_value());
        *model = value;
    }
}
