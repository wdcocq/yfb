mod common;

use std::{fmt::Display, str::FromStr};

use common::*;
#[cfg(not(target_arch = "wasm32"))]
use tokio::test;
use validator::Validate;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::prelude::*;
use yfb::{model::ValueMarker, prelude::*};
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
#[cfg(target_arch = "wasm32")]
use web_sys::*;
#[cfg(target_arch = "wasm32")]
use yew::platform::time::sleep;

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

    #[function_component(Test)]
    fn test() -> Html {
        let binding = use_named_binding("id", || 1);

        html! {
            <BindingComp {binding}/>
        }
    }

    let html = render::<Test>().await;
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

#[cfg(target_arch = "wasm32")]
#[test]
async fn test_binding_eq() {
    #[derive(Debug, Default, PartialEq, Model, Validate)]
    struct Model {
        a: u32,
        b: Option<u32>,
        c: Child,
        d: Option<Child>,
    }

    #[derive(Debug, Default, PartialEq, Model, Validate)]
    struct Child {
        a: u32,
    }

    #[function_component(Test)]
    fn test() -> Html {
        let binding = use_binding(Model::default);
        let binding_a = binding.a_binding();
        let binding_b = binding.b_binding().map_option();
        let binding_c = binding.c_binding().a_binding();
        let binding_d = binding.d_binding().map_option().a_binding();

        let state = use_mut_ref(|| 0);
        *state.borrow_mut() += 1;
        let onclick = {
            let binding_a = binding_a.clone();
            let binding_b = binding_b.clone();
            let binding_c = binding_c.clone();
            let binding_d = binding_d.clone();
            let state = state.clone();

            Callback::from(move |_| match *state.borrow() {
                1 => binding_a.modifier().set("42"),
                2 => binding_b.modifier().set("42"),
                3 => binding_c.modifier().set("42"),
                4 => binding_d.modifier().set("42"),
                _ => {}
            })
        };

        html! {
            <>
                <div id="button" {onclick}/>
                <div id="result">
                    <BindingComp binding={binding_a}/>
                    <BindingComp binding={binding_b}/>
                    <ChildBindingComp binding={binding.c_binding()}/>
                    <ChildBindingComp binding={binding.d_binding().map_option()}/>
                </div>
            </>
        }
    }
    #[derive(Properties, PartialEq)]
    struct Props {
        binding: Binding<u32>,
    }

    #[function_component(BindingComp)]
    fn binding_comp(Props { binding }: &Props) -> Html {
        let state = use_mut_ref(|| 0);
        *state.borrow_mut() += 1;
        assert!(
            *state.borrow() <= 2,
            "Should not have been triggered: {binding:?}"
        );

        html! {
            <p>{binding.state()}</p>
        }
    }

    #[derive(Properties, PartialEq)]
    struct ChildProps {
        binding: Binding<Child>,
    }

    #[function_component(ChildBindingComp)]
    fn binding_comp(ChildProps { binding }: &ChildProps) -> Html {
        let state = use_mut_ref(|| 0);
        *state.borrow_mut() += 1;
        assert!(
            *state.borrow() <= 2,
            "Should not have been triggered: {binding:?}"
        );

        html! {
            <BindingComp binding={binding.a_binding()}/>
        }
    }

    render::<Test>().await;
    let elem = get_element_by_id::<HtmlElement>("button");

    for i in 0..4 {
        let expected = format!(
            "<p>{}</p><p>{}</p><p>{}</p><p>{}</p>",
            (i > 0) as u8 * 42,
            (i > 1) as u8 * 42,
            (i > 2) as u8 * 42,
            (i > 3) as u8 * 42,
        );
        assert_eq!(get_result(), expected);
        elem.click();
        sleep(std::time::Duration::ZERO).await;
    }
}

#[cfg(target_arch = "wasm32")]
#[test]
async fn test_binding_invalidation_deps() {
    #[function_component(Test)]
    fn test() -> Html {
        let state = use_state(|| 1);
        let binding = use_named_binding_with_deps("test", |s| *s, *state);
        let onclick = move |_| state.set(*state + 1);

        html! {
            <>
                <div id="button" {onclick}/>
                <div id="result">
                    <BindingComp {binding}/>
                </div>
            </>
        }
    }
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

    render::<Test>().await;
    let elem = get_element_by_id::<HtmlElement>("button");

    for i in 1..=3 {
        let expected = format!("<p>{}</p>", i);
        assert_eq!(get_result(), expected);
        elem.click();
        sleep(std::time::Duration::ZERO).await;
    }
}
