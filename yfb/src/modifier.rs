use std::{cell::Ref, rc::Rc};

use yew::AttrValue;

use crate::{
    binding::BindingValidation,
    model::{Dirty, ModelState, State, Value},
    state_model::{MappedStateModel, MappedVecStateModel, Mapping, StateModelRc},
};

pub type OptionModifier<T> = BaseModifier<Option<T>>;
pub type VecModifier<T> = BaseModifier<Vec<T>>;

pub trait Modifier<T>
where
    T: ModelState,
{
    fn create(state_model: StateModelRc<T>, validation: Rc<BindingValidation>) -> Self;
    fn state_model(&self) -> &StateModelRc<T>;
    fn validation(&self) -> &Rc<BindingValidation>;
    fn map<M>(&self, mapping: M) -> <M::To as ModelState>::Modifier
    where
        M: Mapping<From = T>,
    {
        let state_model = Rc::new(MappedStateModel::new(self.state_model().clone(), mapping));
        Modifier::create(state_model, self.validation().clone())
    }

    fn model(&self) -> Ref<T> {
        self.state_model().model()
    }

    fn state(&self) -> Ref<T::State> {
        self.state_model().state()
    }

    fn set_message(&self, _message: Option<AttrValue>) {}

    fn dirty(&self) -> bool {
        self.state().dirty()
    }
}

pub struct BaseModifier<T>(pub(crate) StateModelRc<T>, pub(crate) Rc<BindingValidation>)
where
    T: ModelState;

impl<T> Modifier<T> for BaseModifier<T>
where
    T: ModelState,
{
    fn create(state_model: StateModelRc<T>, validation: Rc<BindingValidation>) -> Self {
        Self(state_model, validation)
    }

    fn state_model(&self) -> &StateModelRc<T> {
        &self.0
    }

    fn validation(&self) -> &Rc<BindingValidation> {
        &self.1
    }
}

impl<T> OptionModifier<T>
where
    T: ModelState + Default,
{
    pub fn take(&self) -> Option<T> {
        let (mut model, mut state) = self.state_model().as_mut();
        let tmp = model.take();
        State::<Option<T>>::update(&mut *state, &*model);
        self.validation().invalidate();
        tmp
    }

    pub fn replace(&self, value: T) -> Option<T> {
        let (mut model, mut state) = self.state_model().as_mut();
        let tmp = (*model).replace(value);
        State::<Option<T>>::update(&mut *state, &*model);
        self.validation().invalidate();
        tmp
    }
}

impl<T> VecModifier<T>
where
    T: ModelState,
{
    pub fn push(&self, value: T) {
        let (mut model, mut state) = self.state_model().as_mut();
        model.push(value);
        (*state).update(&*model);
        self.validation().invalidate();
    }

    pub fn item_modifier(&self, index: usize) -> T::Modifier {
        Modifier::create(
            Rc::new(MappedVecStateModel::new(self.state_model().clone(), index)),
            self.validation().clone(),
        )
    }
}
pub struct FieldModifier<T>(BaseModifier<T>)
where
    T: Value;

impl<T> Modifier<T> for FieldModifier<T>
where
    T: Value,
{
    fn create(state_model: StateModelRc<T>, validation: Rc<BindingValidation>) -> Self {
        Self(BaseModifier(state_model, validation))
    }

    fn state_model(&self) -> &StateModelRc<T> {
        self.0.state_model()
    }

    fn validation(&self) -> &Rc<BindingValidation> {
        self.0.validation()
    }

    fn set_message(&self, message: Option<AttrValue>) {
        let (_, mut state) = self.state_model().as_mut();
        state.set_message(message);
        self.validation().invalidate();
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

        self.validation().invalidate();
    }

    pub fn set_value(&self, value: T) {
        let (mut model, mut state) = self.state_model().as_mut();
        state.set_value(value.to_value());
        *model = value;
        self.validation().invalidate();
    }
}
