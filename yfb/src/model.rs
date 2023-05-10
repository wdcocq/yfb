use std::{
    convert::Infallible,
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use yew::AttrValue;

use crate::{
    field::{Field, FieldModifier},
    hooks::UseGenerationHandle,
    modifier::{Modifier, OptionModifier, VecModifier},
};

pub trait Model: ModelState {
    const NAME: &'static str;
}

pub trait ModelMarker {}

pub trait Value: ModelState<State = Field, Modifier = FieldModifier<Self>> {
    type Err;

    fn to_value(&self) -> AttrValue;
    fn from_value(value: &AttrValue) -> Result<Self, Self::Err>;
}

pub trait ValueMarker {}

pub trait ModelState: PartialEq + Sized + 'static {
    type State: State<Self> + PartialEq;
    type Modifier: Modifier<Self>;
}

pub trait State<T>: Dirty + std::fmt::Debug
where
    T: ModelState,
{
    fn create(model: &T, dirty: bool, generation: UseGenerationHandle) -> Self;
    fn update(&mut self, model: &T);
    fn generation(&self) -> usize;
}

pub trait Dirty {
    fn dirty(&self) -> bool;
}

impl<T> ModelState for Vec<T>
where
    T: ModelState,
{
    type Modifier = VecModifier<T>;
    type State = VecState<T>;
}

impl<T> Model for Vec<T>
where
    T: Model,
{
    const NAME: &'static str = T::NAME;
}

#[derive(PartialEq)]
pub struct VecState<T>
where
    T: ModelState,
{
    // Keeps track of the initial length, states after this length may be purged. as their initial state is not needed.
    initial_length: usize,
    valid_length: usize,
    generation: UseGenerationHandle,
    pub(crate) current: Vec<T::State>,
}

impl<T> std::fmt::Debug for VecState<T>
where
    T: ModelState,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VecState")
            .field("initial_length", &self.initial_length)
            .field("valid_length", &self.valid_length)
            .field("generation", &self.generation)
            .field("current", &self.current)
            .finish()
    }
}

impl<T> State<Vec<T>> for VecState<T>
where
    T: ModelState,
{
    fn create(model: &Vec<T>, with_initial: bool, generation: UseGenerationHandle) -> Self {
        Self {
            valid_length: model.len(),
            initial_length: model.len(),
            current: model
                .iter()
                .map(|m| State::create(m, with_initial, generation.clone()))
                .collect(),
            generation,
        }
    }

    fn update(&mut self, model: &Vec<T>) {
        self.current.extend(
            model
                .iter()
                .skip(self.current.len())
                .map(|m| State::create(m, false, self.generation.clone())),
        );

        if model.len() <= self.initial_length {
            self.current.truncate(self.initial_length);
        }

        for (s, m) in self.current.iter_mut().zip(model) {
            s.update(m);
        }

        self.valid_length = model.len();
    }

    fn generation(&self) -> usize {
        self.current
            .iter()
            .map(State::generation)
            .max()
            .unwrap_or_default()
    }
}

impl<T> Dirty for VecState<T>
where
    T: ModelState,
{
    fn dirty(&self) -> bool {
        if self.valid_length != self.initial_length {
            return true;
        }
        self.current
            .iter()
            .take(self.valid_length)
            .any(Dirty::dirty)
    }
}

impl<T> ModelState for Option<T>
where
    T: ModelState + Default,
{
    type Modifier = OptionModifier<T>;
    type State = T::State;
}

impl<T> Model for Option<T>
where
    T: Model + Default,
{
    const NAME: &'static str = T::NAME;
}

impl<T> State<Option<T>> for T::State
where
    T: ModelState + Default,
{
    fn create(model: &Option<T>, with_initial: bool, generation: UseGenerationHandle) -> Self {
        model
            .as_ref()
            .map(|m| State::create(m, with_initial, generation.clone()))
            .unwrap_or_else(|| State::create(&T::default(), false, generation))
    }

    fn update(&mut self, model: &Option<T>) {
        match model {
            Some(model) => self.update(model),
            None => self.update(&T::default()),
        }
    }

    fn generation(&self) -> usize {
        State::<T>::generation(self)
    }
}

impl<T> ModelState for T
where
    T: Value,
{
    type Modifier = FieldModifier<T>;
    type State = Field;
}

macro_rules! impl_value_marker {
    ($($t:ty),*) => {
        $( impl ValueMarker for $t {} )*
    };
}

impl_value_marker!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, String
);

impl<T> Value for T
where
    T: PartialEq + FromStr + ToString + ValueMarker + 'static,
{
    type Err = <T as FromStr>::Err;

    fn to_value(&self) -> AttrValue {
        self.to_string().into()
    }

    fn from_value(value: &AttrValue) -> Result<Self, Self::Err> {
        value.as_str().parse()
    }
}

impl Value for AttrValue {
    type Err = Infallible;

    fn to_value(&self) -> AttrValue {
        self.clone()
    }

    fn from_value(value: &AttrValue) -> Result<Self, Self::Err> {
        Ok(value.clone())
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wrapped<T>(pub T);

impl<T> Value for Wrapped<T>
where
    T: FromStr + ToString + PartialEq + 'static,
{
    type Err = <T as FromStr>::Err;

    fn to_value(&self) -> AttrValue {
        self.to_string().into()
    }

    fn from_value(value: &AttrValue) -> Result<Self, Self::Err> {
        value.as_str().parse().map(Self)
    }
}

impl<T> From<T> for Wrapped<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Display> Display for Wrapped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Deref for Wrapped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Wrapped<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hooks::use_generation;

    #[test]
    fn test_vec_state() {
        let generation = use_generation();
        let mut state = VecState::create(&vec![0, 1, 2], true, generation);

        assert_eq!(state.initial_length, 3);
        assert_eq!(state.current.len(), 3);
        assert!(!state.dirty());
        assert!(!state.current[0].dirty());
        assert!(!state.current[1].dirty());
        assert!(!state.current[2].dirty());

        state.update(&vec![1, 1]);
        assert_eq!(state.initial_length, 3);
        assert_eq!(state.current.len(), 3);
        assert!(state.dirty());
        assert!(state.current[0].dirty());
        assert!(!state.current[1].dirty());

        state.update(&vec![2, 3, 4, 5]);
        assert_eq!(state.initial_length, 3);
        assert_eq!(state.current.len(), 4);
        assert!(state.dirty());
        assert!(state.current[0].dirty());
        assert!(state.current[1].dirty());
        assert!(state.current[2].dirty());
        assert!(state.current[3].dirty());

        state.update(&vec![0, 1, 2]);
        assert_eq!(state.initial_length, 3);
        assert_eq!(state.current.len(), 3);
        assert!(!state.dirty());
        assert!(!state.current[0].dirty());
        assert!(!state.current[1].dirty());
        assert!(!state.current[2].dirty());
    }
}
