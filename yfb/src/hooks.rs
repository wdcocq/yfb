use yew::prelude::*;

use crate::{
    binding::{Binding, BindingValidation},
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
    let update = use_force_update();
    let state_model = use_memo(
        |deps| {
            let model = init_fn(deps);
            let state = crate::model::State::create(&model, true);
            std::cell::RefCell::new((model, state))
        },
        deps,
    );
    Binding::new(state_model, name, BindingValidation::new(update))
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct UseForceUpdateHandle;

#[cfg(test)]
impl UseForceUpdateHandle {
    pub fn force_update(&self) {}
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
    let state = crate::model::State::create(&model, true);
    Binding::new(
        std::rc::Rc::new(std::cell::RefCell::new((model, state))),
        name.into(),
        BindingValidation::new(UseForceUpdateHandle),
    )
}
