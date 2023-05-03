# YFB - Yew Form Bindings

YFB provides easy binding between models and input components for [Yew](https://crates.io/crates/yew).

Validation is provided by the [validator](https://crates.io/crates/validator) crate.

## Example

```rust
use validator::Validate;
use yfb::{self, prelude::*};
use yew::prelude::*;

#[derive(Default, PartialEq, Model, Validate)]
struct User {
    name: String,
    age: u8,
}

#[function_component(App)]
fn app() -> Html {
    let user = use_binding(User::default);

    html! {
        <>
            <yfb::Input<String> binding={user.name_binding()}/>
            <yfb::Input<u8> binding={user.age_binding()}/>
            if !user.model().name.is_empty() {
                <div>
                    { format!("Hi {}! Your age is {}, is that correct?",
                        user.model().name,
                        user.model().age) }
                </div>
            }
        </>
    }
}
```
