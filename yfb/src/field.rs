use std::fmt::{self, Display, Formatter};

use yew::AttrValue;

use crate::model::{Dirty, State, Value};

/// Contains the current state of a [`Model`](crate::model::Model)'s field
#[derive(Debug, Default, PartialEq)]
pub struct Field {
    initial: Option<AttrValue>,
    value: AttrValue,
    message: Option<AttrValue>,
    error: Option<AttrValue>,
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
        }
    }

    /// Returns `Some(..)` if the field did not pass validation, or else `None`
    pub fn message(&self) -> Option<&AttrValue> {
        self.message.as_ref()
    }

    pub(crate) fn set_message(&mut self, message: Option<AttrValue>) {
        self.message = message;
    }

    /// Returns `Some(..)` when the field was unable to parse the raw string input, or else `None`
    pub fn error(&self) -> Option<&AttrValue> {
        self.error.as_ref()
    }

    pub(crate) fn set_error(&mut self, error: Option<AttrValue>) {
        self.error = error;
    }
}

impl<T> State<T> for Field
where
    T: Value,
{
    fn create(value: &T, with_initial: bool) -> Self {
        let value = value.to_value();

        Field {
            initial: with_initial.then(|| value.clone()),
            value,
            ..Default::default()
        }
    }

    fn update(&mut self, model: &T) {
        self.set_value(model.to_value());
    }
}

impl Dirty for Field {
    fn dirty(&self) -> bool {
        match self.initial.as_ref() {
            Some(initial) => *initial != self.value,
            None => true,
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
