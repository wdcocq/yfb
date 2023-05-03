use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    html::{ImplicitClone, IntoPropValue},
    prelude::*,
};

use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Tel,
    Url,
    Date,
    Search,
}

impl From<InputType> for &'static str {
    fn from(value: InputType) -> Self {
        match value {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Date => "date",
            InputType::Search => "search",
        }
    }
}

impl ImplicitClone for InputType {}

impl IntoPropValue<Option<AttrValue>> for InputType {
    fn into_prop_value(self) -> Option<AttrValue> {
        <AttrValue as From<&'static str>>::from(self.into()).into()
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps<T: Value> {
    pub binding: Binding<T>,
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or(InputType::Text)]
    pub input_type: InputType,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub classes: Classes,
    /// Classes that are applied when the field is dirt and invalid
    #[prop_or_default]
    pub classes_invalid: Classes,
    /// Classes that are applied when the field is dirty and valid
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub tabindex: Option<AttrValue>,
    #[prop_or_default]
    pub hidden: bool,
}

#[function_component(Input)]
pub fn input<T: Value>(
    InputProps {
        binding,
        input_ref,
        autocomplete,
        input_type,
        placeholder,
        disabled,
        classes,
        classes_invalid,
        classes_valid,
        oninput,
        tabindex,
        hidden,
    }: &InputProps<T>,
) -> Html {
    let classes = classes!(
        classes.clone(),
        binding
            .state()
            .dirty()
            .then(|| match binding.state().valid() {
                true => classes_valid.clone(),
                false => classes_invalid.clone(),
            })
    );

    let oninput = {
        let binding = binding.clone();

        oninput.reform(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                binding.modifier().set(target.value());
            }
            e
        })
    };

    let autocomplete = if *autocomplete { "on" } else { "off" };

    html! {
        <input
            ref={input_ref}
            id={binding.name()}
            name={binding.name()}
            class={classes}
            type={*input_type}
            {autocomplete}
            {placeholder}
            value={binding.state().value()}
            {oninput}
            disabled={*disabled}
            {tabindex}
            hidden={*hidden}
        />
    }
}
