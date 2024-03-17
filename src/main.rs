use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <BasicComponent />
        <Iteration />
    }
}

#[component]
fn IterationVecPatt() -> impl IntoView {
    let values = vec![0, 1, 2];
    let part_one = view! {
        <strong>"Purely static"</strong>
        <p>{values.clone()}</p>
        <ul>
            {
                values.into_iter()
                      .map(|n| view! { <li>{n}</li> })
                      // The same as .collect::<Vec<_>>()
                      .collect_view()
            }
        </ul>
    };

    // The list of views may be static, but the interface can be dynamic

    // Create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));

    // Each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    let part_two = view! {
        <strong>"Static list, dynamic items"</strong>
        <ul>{counter_buttons}</ul>
    };

    view! {
        <h4>"The Vec<_> Pattern (for static views)"</h4>
        {part_one}
        <br/>
        {part_two}
    }
}

#[component]
fn Iteration() -> impl IntoView {
    view! {
        <h2>3.4 Iteration</h2>
        <IterationVecPatt />
    }
}

#[component]
fn BasicComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <h2>3.1 A Basic Component</h2>
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
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
    view! {
        <progress
            max=max
            value=progress
        />
    }
}
