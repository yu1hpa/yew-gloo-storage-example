use yew::prelude::*;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

#[function_component(App)]
fn app() -> Html {
    LocalStorage::set("yew", "Hello Yew!!!").ok();
    let yew: String = LocalStorage::get("yew").unwrap_or_default();
    html! {
        <h1>{ yew }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
