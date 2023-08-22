use std::{
    cell::Ref,
    fmt::{self, Debug, Formatter},
    ops::Deref,
    rc::Rc,
};

use yew::{html::ImplicitClone, AttrValue};

use crate::{
    model::{Model, ModelState, State},
    modifier::Modifier,
    state_model::{
        MappedOptionStateModel, MappedStateModel, MappedVecStateModel, Mapping, StateModelRc,
    },
};

/// Use to bind between a [`Model`] or [`Value`](crate::model::Value) and a component
/// Use one of the [`use_binding()`](fn@crate::hooks::use_binding) or [`use_named_binding()`](fn@crate::hooks::use_named_binding) hooks to create a binding.
pub struct Binding<T>
where
    T: ModelState,
{
    state_model: StateModelRc<T>,
    name: AttrValue,
    valid_generation: usize,
}

impl<T: Debug> Debug for Binding<T>
where
    T: ModelState,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Binding")
            .field("name", &self.name.as_str())
            .field("valid_generation", &self.valid_generation)
            .field("model", &*self.model())
            .field("state", &*self.state())
            .finish()
    }
}

impl<T> Binding<T>
where
    T: ModelState,
{
    pub(crate) fn new(
        state_model: StateModelRc<T>,
        name: impl Into<AttrValue>,
        valid_generation: usize,
    ) -> Self {
        Self {
            state_model,
            name: name.into(),
            valid_generation,
        }
    }

    /// Gets the [`Model`] or [`Value`](crate::model::Value) the binding is bound to
    pub fn model(&self) -> Ref<T> {
        self.state_model.model()
    }

    /// Gets the current state of the [`Model`] or [`Value`](crate::model::Value) the binding is bound to
    pub fn state(&self) -> Ref<T::State> {
        self.state_model.state()
    }

    /// Create a modifier to modify the binding's model and state
    pub fn modifier(&self) -> T::Modifier {
        Modifier::create(self.state_model.clone())
    }

    /// The name of the binding, each subsequent binding will append to the root name.
    /// A snake_case name is automatically provided for structs deriving [`Model`].
    /// For [`Value`](crate::model::Value)'s a name has to be provided upon binding with [`use_named_binding()`](fn@crate::hooks::use_named_binding)
    /// ```
    /// # use validator::Validate;
    /// # use yfb::prelude::*;
    /// # use yew::prelude::*;
    /// # #[derive(PartialEq)]
    /// #[derive(Default, Model, Validate)]
    /// struct MyModel {
    ///     value: String
    /// }
    ///
    /// #[function_component]
    /// fn MyComponent() -> Html {
    ///     let binding = use_binding(MyModel::default);
    ///     assert_eq!(binding.value_binding().name(), "my_model.value");
    ///     // ...
    /// #   html!{}
    /// }
    /// ```
    pub fn name(&self) -> &AttrValue {
        &self.name
    }

    /// Resets the name of the binding, making the returned binding act as the root name.
    /// ```
    /// # use validator::Validate;
    /// # use yfb::prelude::*;
    /// # use yew::prelude::*;
    /// # #[derive(PartialEq)]
    /// #[derive(Default, Model, Validate)]
    /// struct MyModel {
    ///     value: String
    /// }
    ///
    /// #[function_component]
    /// fn MyComponent() -> Html {
    ///     let binding = use_binding(MyModel::default);
    ///     let value_binding = binding.value_binding().name_reset("value");
    ///     assert_eq!(value_binding.name(), "value");
    ///     // ...
    /// #   html!{}
    /// }
    /// ```
    pub fn name_reset(&self, name: impl Into<AttrValue>) -> Self {
        Self {
            state_model: self.state_model.clone(),
            name: name.into(),
            valid_generation: self.valid_generation,
        }
    }
}

impl<T> Binding<T>
where
    T: Model,
{
    #[doc(hidden)]
    /// Maps the binding to a child [`Model`] or [`Value`](crate::model::Value) through a [`Mapping`]
    /// Prefer using the generated field_binding() methods instead.
    pub fn map<M>(&self, mapping: M) -> Binding<M::To>
    where
        M: Mapping<From = T>,
    {
        let name: AttrValue = format!("{}.{}", self.name, M::NAME).into();
        Binding::new(
            Rc::new(MappedStateModel::new(self.state_model.clone(), mapping)),
            name,
            self.valid_generation,
        )
    }
}

impl<T> Binding<Option<T>>
where
    T: ModelState + Default,
{
    /// Maps a `Binding<Option<T>>` to a `Binding<T>`
    pub fn map_option(&self) -> Binding<T> {
        Binding::new(
            Rc::new(MappedOptionStateModel::new(self.state_model.clone())),
            self.name.clone(),
            self.valid_generation,
        )
    }
}

impl<T> Binding<Vec<T>>
where
    T: ModelState,
{
    /// Maps a `Binding<Vec<T>>` to a `Binding<T>` with the corresponding `index`
    pub fn map_item(&self, index: usize) -> Binding<T> {
        let name: AttrValue = format!("{}[{}]", self.name, index).into();
        Binding::new(
            Rc::new(MappedVecStateModel::new(self.state_model.clone(), index)),
            name,
            self.valid_generation,
        )
    }
}

impl<T> PartialEq for Binding<T>
where
    T: ModelState,
{
    fn eq(&self, other: &Self) -> bool {
        let max_generation = self
            .state()
            .deref()
            .generation()
            .max(other.state().deref().generation());

        max_generation <= self.valid_generation.min(other.valid_generation)
            && self.name == other.name
    }
}

impl<T> Clone for Binding<T>
where
    T: ModelState,
{
    fn clone(&self) -> Self {
        Self {
            state_model: self.state_model.clone(),
            name: self.name.clone(),
            valid_generation: self.valid_generation,
        }
    }
}

impl<T> ImplicitClone for Binding<T> where T: ModelState {}

#[cfg(test)]
mod tests {
    use validator::Validate;
    use yfb_derive::Model;

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_binding_name() {
        #[derive(Debug, PartialEq, Model, Validate)]
        #[yfb(path = "crate")]
        struct Parent {
            child: Child,
            child_opt: Option<Child>,
            child_none: Option<Child>,
            child_vec: Vec<Child>,
        }

        #[derive(Clone, Debug, Default, PartialEq, Model, Validate)]
        #[yfb(path = "crate")]
        struct Child {
            name: String,
            name_opt: Option<String>,
            name_vec: Vec<String>,
        }

        let binding = use_binding(|| Parent {
            child: Default::default(),
            child_opt: Some(Default::default()),
            child_none: None,
            child_vec: vec![Default::default(); 2],
        });

        assert_eq!(binding.name(), "parent");
        assert_eq!(binding.child_binding().name(), "parent.child");
        assert_eq!(binding.child_opt_binding().name(), "parent.child_opt");
        assert_eq!(
            binding.child_opt_binding().map_option().name(),
            "parent.child_opt"
        );
        assert_eq!(binding.child_none_binding().name(), "parent.child_none");
        assert_eq!(binding.child_vec_binding().name(), "parent.child_vec");
        assert_eq!(
            binding.child_binding().name_binding().name(),
            "parent.child.name"
        );
        assert_eq!(
            binding
                .child_opt_binding()
                .map_option()
                .name_opt_binding()
                .name(),
            "parent.child_opt.name_opt"
        );
        assert_eq!(
            binding
                .child_none_binding()
                .map_option()
                .name_opt_binding()
                .map_option()
                .name(),
            "parent.child_none.name_opt"
        );
        assert_eq!(
            binding
                .child_vec_binding()
                .map_item(0)
                .name_vec_binding()
                .name(),
            "parent.child_vec[0].name_vec"
        );
        assert_eq!(
            binding
                .child_vec_binding()
                .map_item(1)
                .name_vec_binding()
                .name(),
            "parent.child_vec[1].name_vec"
        );
        assert_eq!(
            binding
                .child_vec_binding()
                .map_item(0)
                .name_vec_binding()
                .map_item(0)
                .name(),
            "parent.child_vec[0].name_vec[0]"
        );
    }

    #[test]
    fn test_binding_name_reset() {
        #[derive(Debug, Default, PartialEq, Model, Validate)]
        #[yfb(path = "crate")]
        struct Parent {
            child: Child,
        }

        #[derive(Clone, Debug, Default, PartialEq, Model, Validate)]
        #[yfb(path = "crate")]
        struct Child {
            name: String,
        }

        let binding = use_binding(|| Parent {
            child: Default::default(),
        });

        let child_binding = binding.map(ParentChildMapping).name_reset("child");
        assert_eq!(child_binding.name(), "child");
        assert_eq!(child_binding.map(ChildNameMapping).name(), "child.name");
        let name_binding = child_binding.map(ChildNameMapping).name_reset("name");
        assert_eq!(name_binding.name(), "name");
    }

    #[test]
    fn test_binding_value_name() {
        let binding = use_named_binding("answer", || 42);
        assert_eq!(binding.name(), "answer");
    }

    #[test]
    fn test_binding_value_option() {
        // fn test<T: Value>(binding: Binding<T>) {
        // }
        let binding = use_named_binding("answer", || Some("test".to_string()));
        let binding_option = binding.map_option();
        assert_eq!(binding_option.state().value(), "test");
        binding.modifier().replace_model(None);
        assert_eq!(*binding.model(), None);
        assert_eq!(binding_option.state().value(), "");
        assert_ne!(binding_option, binding_option);
    }

    #[test]
    fn test_owned_option() {
        #[derive(Debug, Default, PartialEq, Model, Validate)]
        #[yfb(path = "crate")]
        struct Parent {
            name: String,
        }

        let binding = use_binding(|| {
            Some(Parent {
                name: "foo".to_string(),
            })
        });
        assert!(binding.model().is_some());
        binding.map_option().modifier().name().set("bar");
        assert!(binding.model().is_some());
        assert_eq!(binding.model().as_ref().unwrap().name, "bar");

        let binding = use_binding(|| Option::<Parent>::None);
        assert!(binding.model().is_none());
        binding.map_option().modifier().name().set("bar");
        assert!(binding.model().is_some());
        assert_eq!(binding.model().as_ref().unwrap().name, "bar");
    }

    #[test]
    fn test_generation() {
        let binding = use_named_binding("answer", || 42);
        binding.modifier().set("43");
        assert_eq!(*binding.model(), 43);
        assert_ne!(binding, binding);
    }

    #[test]
    fn test_validation() {
        #[derive(Debug, Default, PartialEq, Model, Validate)]
        #[yfb(path = "crate")]
        struct Model {
            #[validate(range(min = 18, max = 20, message = "Not within range"))]
            a: u32,
            b: u32,
        }

        let binding = use_binding(Model::default);
        binding.modifier().b().set("42");
        assert_eq!(binding.state().a.message(), None);
        assert_eq!(binding.state().b.message(), None);

        binding.modifier().a().set("42");
        assert_eq!(
            binding.state().a.message().map(AttrValue::as_str),
            Some("Not within range")
        );
        assert_eq!(binding.state().b.message(), None);
        binding.modifier().a().set("18");
        assert_eq!(binding.state().a.message(), None);
        assert_eq!(binding.state().b.message(), None);
    }
}
