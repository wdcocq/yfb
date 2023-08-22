use std::rc::Rc;

use web_sys::HtmlSelectElement;
use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VChild};

use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Options {
    Controlled(VChild<SelectOption>),
    Uncontrolled(Html),
}

impl From<VChild<SelectOption>> for Options {
    fn from(child: VChild<SelectOption>) -> Self {
        Options::Controlled(child)
    }
}

impl From<Html> for Options {
    fn from(child: Html) -> Self {
        Options::Uncontrolled(child)
    }
}

impl From<Options> for Html {
    fn from(value: Options) -> Self {
        match value {
            Options::Controlled(child) => child.into(),
            Options::Uncontrolled(child) => child,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps<T: Value> {
    pub binding: Binding<T>,
    pub children: ChildrenRenderer<Options>,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub classes_invalid: Classes,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(Select)]
pub fn select<T: Value>(
    SelectProps {
        binding,
        autocomplete,
        disabled,
        multiple,
        classes,
        classes_valid,
        classes_invalid,
        children,
        onchange,
    }: &SelectProps<T>,
) -> Html {
    let selected = binding.state().value().clone();
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

    let onchange = {
        let binding = binding.clone();

        onchange.reform(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                binding.modifier().set(input.value());
            }

            e
        })
    };

    let autocomplete = if *autocomplete { "on" } else { "off" };

    html! {
        <select
            id={binding.name()}
            name={binding.name()}
            {autocomplete}
            disabled={*disabled}
            multiple={*multiple}
            class={classes}
            {onchange}
        >
            { for children.iter().map(move |option| {
                match option {
                    Options::Controlled(mut option) => {
                        let props = Rc::make_mut(&mut option.props);
                        props.selected = props.value == selected;
                        option.into()
                    },
                    Options::Uncontrolled(option) => {
                        option
                    }
                }
            })}
        </select>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectOptionProps {
    pub value: AttrValue,
    #[prop_or_default]
    pub children: Option<Children>,
    #[prop_or_default]
    selected: bool,
}

#[function_component(SelectOption)]
pub fn select_item(
    SelectOptionProps {
        value,
        children,
        selected,
    }: &SelectOptionProps,
) -> Html {
    html! {
        <option selected={*selected} {value}>
            if let Some(children) = children {
                {children.clone()}
            } else {
                {value}
            }
        </option>
    }
}
