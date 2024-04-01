use leptos::*;

mod basic_components;
mod forms_and_inputs;
mod iteration;

use basic_components::*;
use iteration::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <BasicComponent/>
        <Iteration/>
        <IterationComplex/>
    }
}
