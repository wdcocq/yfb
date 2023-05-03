use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::model::ModelState;

pub type StateModelRc<T> = Rc<dyn StateModel<Model = T>>;

pub trait StateModel {
    type Model: ModelState;
    fn model(&self) -> Ref<Self::Model>;
    fn state(&self) -> Ref<<Self::Model as ModelState>::State>;
    fn as_mut(
        &self,
    ) -> (
        RefMut<Self::Model>,
        RefMut<<Self::Model as ModelState>::State>,
    );
}

impl<T> StateModel for RefCell<(T, T::State)>
where
    T: ModelState,
{
    type Model = T;

    fn model(&self) -> Ref<Self::Model> {
        Ref::map(self.borrow(), |(model, _)| model)
    }

    fn state(&self) -> Ref<<Self::Model as ModelState>::State> {
        Ref::map(self.borrow(), |(_, state)| state)
    }

    fn as_mut(
        &self,
    ) -> (
        RefMut<Self::Model>,
        RefMut<<Self::Model as ModelState>::State>,
    ) {
        RefMut::map_split(self.borrow_mut(), |(model, state)| (model, state))
    }
}

pub struct MappedOptionStateModel<T>
where
    T: ModelState + Default,
{
    parent: Rc<dyn StateModel<Model = Option<T>>>,
    shadow: RefCell<T>,
}

impl<T> MappedOptionStateModel<T>
where
    T: ModelState + Default,
{
    pub fn new(value: Rc<dyn StateModel<Model = Option<T>>>) -> Self {
        Self {
            parent: value,
            shadow: Default::default(),
        }
    }
}

impl<T> StateModel for MappedOptionStateModel<T>
where
    T: ModelState + Default,
{
    type Model = T;

    fn model(&self) -> Ref<T> {
        Ref::filter_map(self.parent.model(), Option::as_ref)
            .unwrap_or_else(|_| self.shadow.borrow())
    }

    fn state(&self) -> Ref<T::State> {
        self.parent.state()
    }

    fn as_mut(&self) -> (RefMut<T>, RefMut<T::State>) {
        let (model, state) = self.parent.as_mut();

        let model = RefMut::map(model, |m| m.get_or_insert_with(Default::default));

        (model, state)
    }
}

pub struct MappedVecStateModel<T>
where
    T: ModelState,
{
    parent: Rc<dyn StateModel<Model = Vec<T>>>,
    index: usize,
}

impl<T> MappedVecStateModel<T>
where
    T: ModelState,
{
    pub fn new(parent: Rc<dyn StateModel<Model = Vec<T>>>, index: usize) -> Self {
        Self { parent, index }
    }
}

impl<T> StateModel for MappedVecStateModel<T>
where
    T: ModelState,
{
    type Model = T;

    fn model(&self) -> Ref<Self::Model> {
        debug_assert!(self.parent.model().len() <= self.parent.state().current.len());
        Ref::map(self.parent.model(), |v| &v[self.index])
    }

    fn state(&self) -> Ref<<Self::Model as ModelState>::State> {
        debug_assert!(self.parent.model().len() <= self.parent.state().current.len());
        debug_assert!(self.index <= self.parent.model().len());
        Ref::map(self.parent.state(), |v| &v.current[self.index])
    }

    fn as_mut(
        &self,
    ) -> (
        RefMut<Self::Model>,
        RefMut<<Self::Model as ModelState>::State>,
    ) {
        let (model, state) = self.parent.as_mut();
        debug_assert!(model.len() <= state.current.len());
        (
            RefMut::map(model, |v| &mut v[self.index]),
            RefMut::map(state, |v| &mut v.current[self.index]),
        )
    }
}

pub trait Mapping: 'static {
    const NAME: &'static str;
    type From: ModelState;
    type To: ModelState;

    fn map_model<'a>(&self, model: &'a Self::From) -> &'a Self::To;
    fn map_model_mut<'a>(&self, model: &'a mut Self::From) -> &'a mut Self::To;
    fn map_state<'a>(
        &self,
        state: &'a <Self::From as ModelState>::State,
    ) -> &'a <Self::To as ModelState>::State;
    fn map_state_mut<'a>(
        &self,
        state: &'a mut <Self::From as ModelState>::State,
    ) -> &'a mut <Self::To as ModelState>::State;
}

pub struct MappedStateModel<M>
where
    M: Mapping,
{
    parent: Rc<dyn StateModel<Model = M::From>>,
    mapping: M,
}

impl<M> MappedStateModel<M>
where
    M: Mapping,
{
    pub fn new(parent: Rc<dyn StateModel<Model = M::From>>, mapping: M) -> Self {
        Self { parent, mapping }
    }
}

impl<M> StateModel for MappedStateModel<M>
where
    M: Mapping,
{
    type Model = M::To;

    fn model(&self) -> Ref<Self::Model> {
        Ref::map(self.parent.model(), |m| self.mapping.map_model(m))
    }

    fn state(&self) -> Ref<<Self::Model as ModelState>::State> {
        Ref::map(self.parent.state(), |s| self.mapping.map_state(s))
    }

    fn as_mut(
        &self,
    ) -> (
        RefMut<Self::Model>,
        RefMut<<Self::Model as ModelState>::State>,
    ) {
        let (model, state) = self.parent.as_mut();
        (
            RefMut::map(model, |m| self.mapping.map_model_mut(m)),
            RefMut::map(state, |s| self.mapping.map_state_mut(s)),
        )
    }
}
