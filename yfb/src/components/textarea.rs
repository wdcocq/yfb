use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::{html::ImplicitClone, prelude::*};

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Wrap {
    Soft,
    Hard,
}

impl ImplicitClone for Wrap {}

impl From<Wrap> for AttrValue {
    fn from(value: Wrap) -> Self {
        match value {
            Wrap::Soft => "soft".into(),
            Wrap::Hard => "hard".into(),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps {
    pub binding: Binding<String>,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub classes_invalid: Classes,
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or(20)]
    pub cols: u32,
    #[prop_or(5)]
    pub rows: u32,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub wrap: Option<Wrap>,
    #[prop_or_default]
    pub spellcheck: Option<bool>,
    #[prop_or_default]
    pub autocomplete: bool,
}

#[function_component(TextArea)]
pub fn text_area(
    TextAreaProps {
        binding,
        oninput,
        classes,
        classes_invalid,
        classes_valid,
        cols,
        rows,
        placeholder,
        disabled,
        wrap,
        spellcheck,
        autocomplete,
    }: &TextAreaProps,
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
            if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                binding.modifier().set(input.value());
            }
            e
        })
    };

    let autocomplete = if *autocomplete { "on" } else { "off" };
    // let spellcheck = spellcheck
    //     .map(|b| if b { "true" } else { "false" })
    //     .unwrap_or("default");

    html! {
        <textarea
            id={binding.name()}
            name={binding.name()}
            class={classes}
            cols={cols.to_string()}
            rows={rows.to_string()}
            {placeholder}
            wrap={wrap.map(AttrValue::from)}
            spellcheck={spellcheck.map(|b| b.to_string())}
            {autocomplete}
            {oninput}
            disabled={*disabled}
        />
    }
}
