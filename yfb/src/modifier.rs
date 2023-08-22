use std::{cell::Ref, rc::Rc};

use yew::AttrValue;

use crate::{
    model::{Dirty, ModelState, State, Value},
    state_model::{MappedStateModel, MappedVecStateModel, Mapping, StateModelRc},
};

pub type OptionModifier<T> = BaseModifier<Option<T>>;
pub type VecModifier<T> = BaseModifier<Vec<T>>;

pub trait Modifier<T>
where
    T: ModelState,
{
    fn create(state_model: StateModelRc<T>) -> Self;
    fn state_model(&self) -> &StateModelRc<T>;
    fn map<M>(&self, mapping: M) -> <M::To as ModelState>::Modifier
    where
        M: Mapping<From = T>,
    {
        let state_model = Rc::new(MappedStateModel::new(self.state_model().clone(), mapping));
        Modifier::create(state_model)
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

    fn replace_model(&self, model: T) {
        let (mut model_mut, mut state) = self.state_model().as_mut();
        *model_mut = model;
        State::<T>::update(&mut *state, &*model_mut, true);
    }
}

pub struct BaseModifier<T>(pub(crate) StateModelRc<T>)
where
    T: ModelState;

impl<T> Modifier<T> for BaseModifier<T>
where
    T: ModelState,
{
    fn create(state_model: StateModelRc<T>) -> Self {
        Self(state_model)
    }

    fn state_model(&self) -> &StateModelRc<T> {
        &self.0
    }
}

impl<T> OptionModifier<T>
where
    T: ModelState + Default,
{
    pub fn take(&self) -> Option<T> {
        let (mut model, mut state) = self.state_model().as_mut();
        let tmp = model.take();
        State::<Option<T>>::update(&mut *state, &*model, false);
        tmp
    }

    pub fn replace(&self, value: T) -> Option<T> {
        let (mut model, mut state) = self.state_model().as_mut();
        let tmp = (*model).replace(value);
        State::<Option<T>>::update(&mut *state, &*model, false);
        tmp
    }
}
impl<T> OptionModifier<T>
where
    T: Value + Default,
{
    pub fn set_initial(&self, value: Option<T>) {
        let (_, mut state) = self.state_model().as_mut();
        state.set_initial(value.map(|v| v.to_value()));
    }
}

impl<T> VecModifier<T>
where
    T: ModelState,
{
    pub fn push(&self, value: T) {
        let (mut model, mut state) = self.state_model().as_mut();
        model.push(value);
        (*state).update(&*model, false);
    }

    pub fn remove(&self, index: usize) {
        let (mut model, mut state) = self.state_model().as_mut();
        model.remove(index);
        (*state).update(&*model, false);
    }

    pub fn item_modifier(&self, index: usize) -> T::Modifier {
        Modifier::create(Rc::new(MappedVecStateModel::new(
            self.state_model().clone(),
            index,
        )))
    }
}
