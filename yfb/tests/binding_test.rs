mod common;

use std::{fmt::Display, str::FromStr};

use common::render;
#[cfg(not(target_arch = "wasm32"))]
use tokio::test;
use validator::Validate;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::prelude::*;
use yfb::{model::ValueMarker, prelude::*};
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Model, Validate, Debug, PartialEq)]
struct Model {
    id: u32,
}

#[test]
async fn test_model_binding() {
    #[derive(Properties, PartialEq)]
    struct Props {
        binding: Binding<Model>,
    }

    #[function_component(BindingComp)]
    fn binding_comp(Props { binding }: &Props) -> Html {
        html! {
            <p>{&binding.state().id}</p>
        }
    }

    #[function_component(Comp)]
    fn comp() -> Html {
        let binding = use_binding(|| Model { id: 1 });

        html! {
            <BindingComp {binding}/>
        }
    }

    let html = render::<Comp>().await;
    assert_eq!(html, "<p>1</p>");
}

#[test]
async fn test_field_binding() {
    #[derive(Properties, PartialEq)]
    struct Props {
        binding: Binding<u32>,
    }

    #[function_component(BindingComp)]
    fn binding_comp(Props { binding }: &Props) -> Html {
        html! {
            <p>{binding.state()}</p>
        }
    }

    #[function_component(Comp)]
    fn comp() -> Html {
        let binding = use_binding(|| Model { id: 1 });

        html! {
            <BindingComp binding={binding.map(ModelIdMapping)}/>
        }
    }

    let html = render::<Comp>().await;
    assert_eq!(html, "<p>1</p>");
}

#[test]
async fn test_value_binding() {
    #[derive(Properties, PartialEq)]
    struct Props {
        binding: Binding<u32>,
    }

    #[function_component(BindingComp)]
    fn binding_comp(Props { binding }: &Props) -> Html {
        html! {
            <p>{binding.state()}</p>
        }
    }

    #[function_component(Comp)]
    fn comp() -> Html {
        let binding = use_named_binding("id", || 1);

        html! {
            <BindingComp {binding}/>
        }
    }

    let html = render::<Comp>().await;
    assert_eq!(html, "<p>1</p>");
}

#[test]
async fn test_value_marker() {
    #[derive(PartialEq)]
    enum Test {
        A,
        B,
    }

    impl FromStr for Test {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Test::A),
                "B" => Ok(Test::B),
                _ => Err("invalid value"),
            }
        }
    }

    impl Display for Test {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Test::A => write!(f, "A"),
                Test::B => write!(f, "B"),
            }
        }
    }

    impl ValueMarker for Test {}

    #[derive(Properties, PartialEq)]
    struct Props {
        binding: Binding<Test>,
    }

    #[function_component(BindingComp)]
    fn binding_comp(Props { binding }: &Props) -> Html {
        html! {
            <p>{binding.state()}</p>
        }
    }

    #[function_component(Comp)]
    fn comp() -> Html {
        let binding = use_named_binding("test", || Test::A);

        html! {
            <BindingComp {binding}/>
        }
    }

    let html = render::<Comp>().await;
    assert_eq!(html, "<p>A</p>");
}
