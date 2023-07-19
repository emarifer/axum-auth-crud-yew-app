use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, virtual_dom::AttrValue, Callback, Event, FocusEvent, Html, NodeRef,
    Properties,
};

#[derive(Debug, Properties, PartialEq)]
pub struct TextareaInputProps {
    // pub input_type: Option<String>,
    pub label: String,
    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
    pub input_value: AttrValue,
}

#[function_component(TextareaInput)]
pub fn textarea_input_component(props: &TextareaInputProps) -> Html {
    // let input_type = props
    //     .input_type
    //     .clone()
    //     .unwrap_or_else(|| "text".to_string());
    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    let handle_on_input_blur = props.handle_on_input_blur.clone();
    let on_blur = {
        let cloned_input_name = props.name.clone();
        Callback::from(move |event: FocusEvent| {
            let input_name = cloned_input_name.clone();
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name, value));
        })
    };

    html! {
        <div>
          <label html={props.name.clone()} for={props.name.to_owned()}
            class="block text-ct-blue-600 hover:text-sky-400 mb-3 cursor-pointer">
            {props.label.clone()}
          </label>

          <textarea
            rows="5"
            id={props.name.to_owned()}
            placeholder="maximum 255 characters"
            class="block w-full rounded-2xl appearance-none text-ct-dark-600 focus:outline-none py-2 px-4"
            ref={props.input_ref.clone()}
            onchange={onchange}
            onblur={on_blur}
            value={&props.input_value}
          />

          <span class="text-red-500 text-xs pt-1 block">
            {error_message}
          </span>
        </div>
    }
}
