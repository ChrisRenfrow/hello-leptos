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
        <IterationComplex />
    }
}

// Sub-sections

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
fn Iteration() -> impl IntoView {
    view! {
        <h2>3.4 Iteration</h2>
        <IterationVecPatt />
        <DynamicList initial_len=5 />
    }
}

#[component]
fn IterationComplex() -> impl IntoView {
    view! {
        <h2>3.5 Iterating over more complex data</h2>
        <ComplexFor />
    }
}

// Sub-section components
// 3.1
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

// 3.4
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
fn DynamicList(initial_len: usize) -> impl IntoView {
    let mut next_counter_id = initial_len;
    // Generate list of counters based on initial length, including the id for each
    let initial_counters = (0..initial_len)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    // Store initial list as a signal for modification over time.
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        // add counter and signal to list of counters
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        // increment id to ensure it's unique
        next_counter_id += 1
    };

    view! {
        <strong>"Dynamic list and items using <For/>"</strong>
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For each=counters
                     // Using an index as a key is generally bad practice unless your list can only grow
                     key=|counter| counter.0
                     // Define the children of the For element
                     // Receives the items from the each iterator
                     children=move |(id, (count, set_count))| {
                         view! {
                             <li>
                                 <button
                                    // Increment count
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                 >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            // TIL: Vec::retain - Retains elements that match the predicate in their original order
                                            // In this case, all elements that don't match the removed id
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                             </li>
                         }
                     }
                />
            </ul>
        </div>
    }
}

// 3.5
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[derive(Debug, Clone)]
struct DatabaseEntryImproved {
    key: String,
    // Wrapping the value in the signal allows us more
    // efficiency when updating.
    value: RwSignal<i32>,
}

#[component]
fn ComplexFor() -> impl IntoView {
    let initial_data = vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ];

    let (data, set_data) = create_signal(initial_data.clone());

    // This won't work. The value for each row changes, but the key does not.
    // This won't trigger element refreshes.
    let problem = view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    };

    // This forces an element refresh using the key, but at a hit to efficiency.
    // Each <p> is completely re-rendered, rather than making a fine-grained
    // adjustment to the inner text value.
    let change_key = view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update values"
        </button>
        <For
            each=data
            // Link the key with the element's value.
            // This causes the key to change when the value is updated, triggering an element update.
            key=|state| (state.key.clone(), state.value)
            let:child
        >
            <p>{child.value}</p>
        </For>

    };

    let initial_data_imp = vec![
        DatabaseEntryImproved {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntryImproved {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntryImproved {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ];
    let (data_imp, set_data_imp) = create_signal(initial_data_imp.clone());

    // By wrapping the value in a signal, we can nest the signals and offer fine-grained updates.
    // This option is highly efficient, but is cumbersome
    let nested_signals = view! {
        <button on:click=move |_| {
            set_data_imp.update(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            logging::log!("{:?}", data_imp.get());
        }>
            "Update values"
        </button>
        <For
            each=data_imp
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    };

    // By using Leptos' create_memo, we can create a derived computation that only triggers
    // a reactive update when its value has changed.
    // The benefits of this approach are largely the same as the nested signals approach.
    // But it comes at the cost of some additional chores and safety precautions.
    let memoized_slices = view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=move || data().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }
        />
    };

    view! {
        <button on:click=move |_| {
            set_data.set(initial_data.clone());
            // This doesn't work as each value is a signal.
            // There would need to be some extra book-keeping involved to preserve the initial value.
            // This demonstrates an aspect of nested signals in that they are cumbersome to work with.
            set_data_imp.set(initial_data_imp.clone());
        }>
            "Reset values"
        </button>
        <br />
        <strong>Problem (no update)</strong>
        <br />
        {problem}
        <strong>Option 1: Change the Key</strong>
        <br />
        {change_key}
        <strong>Option 2: Nested Signals</strong>
        <br />
        {nested_signals}
        <strong>Option 3: Memoized Slices</strong>
        <br />
        {memoized_slices}
    }
}
