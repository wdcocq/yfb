use std::{cell::Cell, rc::Rc};

use yew::prelude::*;

use crate::{
    binding::Binding,
    model::{Model, ModelState},
};

/// Get a binding of model `T`
#[cfg_attr(not(test), hook)]
pub fn use_binding<T>(init_fn: impl FnOnce() -> T) -> Binding<T>
where
    T: Model + 'static,
{
    use_binding_with_deps(|_| init_fn(), ())
}

/// Get a binding of model `T` with dependencies
#[cfg_attr(not(test), hook)]
pub fn use_binding_with_deps<T, D>(init_fn: impl FnOnce(&D) -> T, deps: D) -> Binding<T>
where
    T: Model + 'static,
    D: PartialEq + 'static,
{
    use_named_binding_with_deps(T::NAME, init_fn, deps)
}

/// Get a binding of value `T` with the root name `name`
/// Useful for creating bindings of a [`Value`](crate::model::Value) without requiring a [`Model`]
#[cfg_attr(not(test), hook)]
pub fn use_named_binding<T>(name: impl Into<AttrValue>, init_fn: impl FnOnce() -> T) -> Binding<T>
where
    T: ModelState + 'static,
{
    use_named_binding_with_deps(name, |_| init_fn(), ())
}

/// Get a binding of value `T` with the root name `name` wuth dependencies
/// Useful for creating bindings of a [`Value`](crate::model::Value) without requiring a [`Model`]
#[cfg(not(test))]
#[hook]
pub fn use_named_binding_with_deps<T, D>(
    name: impl Into<AttrValue>,
    init_fn: impl FnOnce(&D) -> T,
    deps: D,
) -> Binding<T>
where
    T: ModelState + 'static,
    D: PartialEq + 'static,
{
    let generation = use_generation();
    let first = use_memo(|_| std::cell::Cell::new(true), ());
    let state_model = {
        let generation = generation.clone();
        use_memo(
            move |deps| {
                if !first.get() {
                    generation.increase();
                } else {
                    first.set(false);
                }
                let model = init_fn(deps);
                let state = crate::model::State::create(&model, true, generation);
                std::cell::RefCell::new((model, state))
            },
            deps,
        )
    };

    Binding::new(state_model, name, generation.generation())
}

#[derive(Clone)]
pub struct UseGenerationHandle {
    generation: Rc<Cell<usize>>,
    update: UseForceUpdateHandle,
}

impl PartialEq for UseGenerationHandle {
    fn eq(&self, other: &Self) -> bool {
        self.generation.get() == other.generation.get()
    }
}

impl std::fmt::Debug for UseGenerationHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseGenerationHandle")
            .field("generation", &self.generation.get())
            .finish()
    }
}

impl UseGenerationHandle {
    pub fn increase(&self) -> usize {
        let next_gen = self.generation.get().wrapping_add(1);
        self.generation.set(next_gen);
        self.update.force_update();
        next_gen
    }

    pub fn generation(&self) -> usize {
        self.generation.get()
    }
}

#[cfg_attr(not(test), hook)]
pub fn use_generation() -> UseGenerationHandle {
    #[cfg(not(test))]
    {
        UseGenerationHandle {
            generation: use_memo(|_| Cell::new(0), ()),
            update: use_force_update(),
        }
    }
    #[cfg(test)]
    {
        UseGenerationHandle {
            generation: Rc::new(Cell::new(0)),
            update: UseForceUpdateHandle,
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct UseForceUpdateHandle;

#[cfg(test)]
impl UseForceUpdateHandle {
    fn force_update(&self) {}
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct UseStateHandle<T>(pub std::rc::Rc<std::cell::RefCell<T>>)
where
    T: Copy;

#[cfg(test)]
impl<T> UseStateHandle<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self(std::cell::RefCell::new(value).into())
    }

    pub fn set(&self, value: T) {
        *self.0.borrow_mut() = value;
    }
}

#[cfg(test)]
impl<T> std::ops::Deref for UseStateHandle<T>
where
    T: Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(*self.0).as_ptr() }
    }
}

#[cfg(test)]
pub fn use_named_binding_with_deps<T, D>(
    name: impl Into<AttrValue>,
    init_fn: impl FnOnce(&D) -> T,
    deps: D,
) -> Binding<T>
where
    T: ModelState + 'static,
    D: PartialEq + 'static,
{
    let model = init_fn(&deps);
    let generation = use_generation();
    let state = crate::model::State::create(&model, true, generation);

    Binding::new(
        std::rc::Rc::new(std::cell::RefCell::new((model, state))),
        name.into(),
        0,
    )
}
