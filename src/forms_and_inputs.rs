use leptos::*;

#[component]
pub fn FormsAndInputs() -> impl IntoView {
    view! {
        <h2>3.6 Forms and Inputs</h2>
        <ControlledInputs/>
        <UncontrolledInputs/>
        <SpecialCaseInputs/>
    }
}

#[component]
// "controlled input" means the framework controls the state of the input element
fn ControlledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <strong>Controlled Inputs</strong>
        <br/>
        <input
            type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }

            // the prop syntax lets you update a DOM property,
            // rather than an attribute
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
// "uncontrolled input" means that the browser controls the state of the input element
fn UncontrolledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop page reload
        ev.prevent_default();
        // get value from input element
        let value = input_element().expect("<input> should be mounted").value();
        set_name(value);
    };

    view! {
        <strong>Uncontrolled Inputs</strong>
        <br/>
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
// These are special case inputs related to the subtle differences between
// the value attribute and the value property.
fn SpecialCaseInputs() -> impl IntoView {
    // Text areas don't support the value **attribute**,
    // but they do support the value **property**
    let (some_value, set_some_value) = create_signal("Text areas are special".to_string());
    let text_area = view! {
        <p>Text areas:</p>
        <textarea
            prop:value=move || some_value.get()
            on:input=move |ev| {
                set_some_value.set(event_target_value(&ev));
            }
        >

            {move || some_value.get_untracked()}
        </textarea>
    };

    // The select element doesn't support the value **attribute** *nor* the value **property**.
    // This is often obscured in other frameworks with a value field on the select element,
    // not so with leptos.
    let (value, set_value) = create_signal("B".to_string());
    /*  We could repeat this for every option below...
     *  ```
     *  <option
     *      value="A"
     *      selected=move || value() == "A"
     *  >
     *      "A"
     *  </option>
     *  ```
     *  Or, we could refactor it out as it's own component (see `SelectOption`)!
     */
    let select = view! {
        <p>Select:</p>
        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value(new_value);
        }>

            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>

        </select>
    };

    view! {
        <strong>Special Cases</strong>
        <br/>
        {text_area}
        <br/>
        {select}
    }
}

#[component]
fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option value=is selected=move || value() == is>
            {is}
        </option>
    }
}
