use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    pub binding: Binding<bool>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
}

#[function_component(Checkbox)]
pub fn checkbox(
    CheckboxProps {
        binding,
        classes,
        ontoggle,
    }: &CheckboxProps,
) -> Html {
    let ontoggle = {
        let binding = binding.clone();

        ontoggle.reform(move |e: InputEvent| {
            if let Some(elem) = e.target_dyn_into::<HtmlInputElement>() {
                binding.modifier().set_value(elem.checked());
            }
            *binding.model()
        })
    };

    html! {
        <input
            id={binding.name()}
            name={binding.name()}
            class={classes.clone()}
            type="checkbox"
            oninput={ontoggle}
            checked={*binding.model()}
            class={classes.clone()}
         />
    }
}
