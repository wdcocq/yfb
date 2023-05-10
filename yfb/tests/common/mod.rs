use std::time::Duration;

use yew::{
    html::ChildrenProps,
    prelude::*,
    suspense::{use_future_with_deps, SuspensionResult},
    BaseComponent,
};

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub async fn render<T>() -> String
where
    T: BaseComponent,
    T::Properties: Default,
{
    render_with_props::<T>(T::Properties::default()).await
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub async fn render_with_props<T>(props: T::Properties) -> String
where
    T: BaseComponent,
{
    yew::Renderer::<T>::with_root_and_props(
        gloo::utils::document().get_element_by_id("output").unwrap(),
        props,
    )
    .render();
    yew::platform::time::sleep(Duration::from_millis(50)).await;
    get_output()
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn get_output() -> String {
    gloo::utils::document()
        .get_element_by_id("output")
        .unwrap()
        .inner_html()
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn get_result() -> String {
    gloo::utils::document()
        .get_element_by_id("result")
        .unwrap()
        .inner_html()
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn get_first_element_of<T: wasm_bindgen::JsCast>() -> T {
    use wasm_bindgen::JsCast;
    gloo::utils::document()
        .get_element_by_id("output")
        .unwrap()
        .first_element_child()
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn get_element_by_id<T: wasm_bindgen::JsCast>(id: &'static str) -> T {
    use wasm_bindgen::JsCast;
    gloo::utils::document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn get_first_element_name<T: wasm_bindgen::JsCast>(name: &'static str) -> T {
    use wasm_bindgen::JsCast;
    gloo::utils::document()
        .get_element_by_id("output")
        .unwrap()
        .get_elements_by_tag_name(name)
        .item(0)
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub async fn render<T>() -> String
where
    T: BaseComponent,
    T::Properties: Default + Send,
{
    render_with_props::<T>(T::Properties::default()).await
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub async fn render_with_props<T>(props: T::Properties) -> String
where
    T: BaseComponent,
    T::Properties: Send,
{
    yew::ServerRenderer::<T>::with_props(|| props)
        .hydratable(false)
        .render()
        .await
}

#[function_component(Test)]
#[allow(dead_code)]
pub fn test(ChildrenProps { children }: &ChildrenProps) -> Html {
    html! {
        <Suspense fallback={html!{}}>
            {children.clone()}
        </Suspense>
    }
}

#[macro_export]
#[allow(dead_code)]
macro_rules! create_test_comp {
    ($t:ty) => {
        #[function_component(Test)]
        pub fn test() -> Html {
            html! {
                <Suspense>
                    <$t/>
                </Suspense>
            }
        }
    };
}

#[hook]
#[allow(dead_code)]
pub fn use_once<F>(f: F) -> SuspensionResult<()>
where
    F: FnOnce() + 'static,
{
    use_future_with_deps(
        |_| async {
            f();
            yew::platform::time::sleep(Duration::from_millis(100)).await;
        },
        (),
    )?;

    Ok(())
}
