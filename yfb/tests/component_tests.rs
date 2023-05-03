mod common;

#[cfg(target_arch = "wasm32")]
use std::time::Duration;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use common::*;
#[cfg(not(target_arch = "wasm32"))]
use tokio::test;
use validator::Validate;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;
#[cfg(target_arch = "wasm32")]
use web_sys::*;
#[cfg(target_arch = "wasm32")]
use yew::platform::time::sleep;
use yew::prelude::*;
use yfb::{components::*, prelude::*};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
async fn test_input() {
    #[derive(Clone, PartialEq, Model, Validate)]
    struct Model {
        input: String,
    }

    #[function_component(Test)]
    pub fn test() -> Html {
        let binding = use_binding(|| Model {
            input: "test".into(),
        });

        html! {
            <>
                <Input<String> binding={binding.input_binding()}/>
                <p>{&binding.state().input}</p>
                <p>{&binding.model().input}</p>
            </>
        }
    }

    #[derive(Clone, PartialEq, Properties)]
    struct ExpectedProps {
        value: String,
    }

    #[function_component(Expected)]
    fn expected(ExpectedProps { value }: &ExpectedProps) -> Html {
        // Neet to distinguish between web and native because `value` gets stripped in browsers
        #[cfg(target_arch = "wasm32")]
        html! {
            <>
                <input
                    id="model.input"
                    name="model.input"
                    type="text"
                    autocomplete="off"
                />
                <p>{value}</p>
                <p>{value}</p>
            </>
        }

        #[cfg(not(target_arch = "wasm32"))]
        html! {
            <>
                <input
                    id="model.input"
                    name="model.input"
                    type="text"
                    autocomplete="off"
                    value={value.clone()}
                />
                <p>{value}</p>
                <p>{value}</p>
            </>
        }
    }

    let expected = render_with_props::<Expected>(ExpectedProps {
        value: "test".to_string(),
    })
    .await;
    assert_eq!(render::<Test>().await, expected);

    #[cfg(target_arch = "wasm32")]
    {
        let elem = get_first_element_of::<HtmlInputElement>();
        elem.set_value("changed");
        elem.dispatch_event(&InputEvent::new("input").unwrap())
            .unwrap();
        sleep(Duration::ZERO).await;

        assert_eq!(
            common::get_output(),
            render_with_props::<Expected>(ExpectedProps {
                value: "changed".to_string()
            })
            .await
        );
    }
}

#[test]
async fn test_checkbox() {
    #[function_component(Test)]
    pub fn test() -> Html {
        let binding = use_named_binding("value", || false);

        html! {
            <>
                <Checkbox binding={&binding}/>
                <p>{&binding.state()}</p>
                <p>{&binding.model()}</p>
            </>
        }
    }

    #[derive(Clone, PartialEq, Properties)]
    struct ExpectedProps {
        value: bool,
    }

    #[function_component(Expected)]
    fn expected(ExpectedProps { value }: &ExpectedProps) -> Html {
        html! {
            <>
                <input
                    id="value"
                    name="value"
                    type="checkbox"
                    checked={*value}
                />
                <p>{value}</p>
                <p>{value}</p>
            </>
        }
    }

    let expected = render_with_props::<Expected>(ExpectedProps { value: false }).await;
    assert_eq!(render::<Test>().await, expected);

    #[cfg(target_arch = "wasm32")]
    {
        get_first_element_of::<HtmlInputElement>().click();
        sleep(Duration::ZERO).await;

        assert_eq!(
            common::get_output(),
            render_with_props::<Expected>(ExpectedProps { value: true }).await
        );
    }
}

#[test]
async fn test_select() {
    #[derive(Copy, Clone, PartialEq)]
    enum TestEnum {
        A,
        B,
    }

    impl Display for TestEnum {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::A => write!(f, "A"),
                Self::B => write!(f, "B"),
            }
        }
    }

    impl FromStr for TestEnum {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Self::A),
                "B" => Ok(Self::B),
                _ => Err("Invalid value"),
            }
        }
    }

    impl ValueMarker for TestEnum {}

    #[function_component(Test)]
    pub fn test() -> Html {
        let binding = use_named_binding("value", || TestEnum::A);

        html! {
            <>
                <Select<TestEnum> binding={&binding}>
                    <SelectOption value={TestEnum::A.to_string()}/>
                    <SelectOption value={TestEnum::B.to_string()}/>
                </Select<TestEnum>>
                <p>{&binding.state()}</p>
                <p>{&binding.model()}</p>
            </>
        }
    }

    #[derive(Clone, PartialEq, Properties)]
    struct ExpectedProps {
        value: TestEnum,
    }

    #[function_component(Expected)]
    fn expected(ExpectedProps { value }: &ExpectedProps) -> Html {
        html! {
            <>
                <select
                    id="value"
                    name="value"
                    autocomplete="off"
                >
                    <option value="A" selected={*value == TestEnum::A}>{"A"}</option>
                    <option value="B" selected={*value == TestEnum::B}>{"B"}</option>
                </select>
                <p>{value}</p>
                <p>{value}</p>
            </>
        }
    }

    let expected = render_with_props::<Expected>(ExpectedProps { value: TestEnum::A }).await;
    assert_eq!(render::<Test>().await, expected);

    #[cfg(target_arch = "wasm32")]
    {
        let elem = get_first_element_of::<HtmlSelectElement>();
        elem.set_value("B");
        elem.dispatch_event(&Event::new("change").unwrap()).unwrap();
        sleep(Duration::ZERO).await;

        assert_eq!(
            common::get_output(),
            render_with_props::<Expected>(ExpectedProps { value: TestEnum::B }).await
        );
    }
}

#[test]
async fn test_textarea() {
    #[derive(Copy, Clone, PartialEq)]
    enum TestEnum {
        A,
        B,
    }

    impl Display for TestEnum {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::A => write!(f, "A"),
                Self::B => write!(f, "B"),
            }
        }
    }

    impl FromStr for TestEnum {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Self::A),
                "B" => Ok(Self::B),
                _ => Err("Invalid value"),
            }
        }
    }

    impl ValueMarker for TestEnum {}

    #[function_component(Test)]
    pub fn test() -> Html {
        let binding = use_named_binding("value", || "test".to_string());

        html! {
            <>
                <TextArea binding={&binding}/>
                <p>{&binding.state()}</p>
                <p>{&binding.model()}</p>
            </>
        }
    }

    #[derive(Clone, PartialEq, Properties)]
    struct ExpectedProps {
        value: String,
    }

    #[function_component(Expected)]
    fn expected(ExpectedProps { value }: &ExpectedProps) -> Html {
        html! {
            <>
                <textarea
                    id="value"
                    name="value"
                    cols="20"
                    rows="5"
                    autocomplete="off"
                />
                <p>{value}</p>
                <p>{value}</p>
            </>
        }
    }

    let expected = render_with_props::<Expected>(ExpectedProps {
        value: "test".into(),
    })
    .await;
    assert_eq!(render::<Test>().await, expected);

    #[cfg(target_arch = "wasm32")]
    {
        let elem = get_first_element_of::<web_sys::HtmlTextAreaElement>();
        elem.set_value("changed");
        elem.dispatch_event(&Event::new("input").unwrap()).unwrap();
        sleep(Duration::ZERO).await;

        assert_eq!(
            common::get_output(),
            render_with_props::<Expected>(ExpectedProps {
                value: "changed".into()
            })
            .await
        );
    }
}
