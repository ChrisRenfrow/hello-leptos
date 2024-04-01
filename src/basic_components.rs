use leptos::*;

#[component]
pub fn BasicComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <h2>3.1 A Basic Component</h2>
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>

            "Count++"
        </button>
        <br/>
        <ProgressBar progress=count/>
        <br/>
        <ProgressBar progress=double_count/>
    }
}

#[component]
fn ProgressBar<F>(#[prop(default = 100)] max: u16, progress: F) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! { <progress max=max value=progress></progress> }
}
