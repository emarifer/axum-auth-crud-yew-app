pub mod task_form_page;

/****** ============================== TASK FORM CALLBACKS ============================== ******/

/****** Required Dependencies ******/

use std::{cell::RefCell, ops::Deref, rc::Rc};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, MouseEvent, NodeRef, SubmitEvent, UseStateHandle};
use yew_router::{
    history::{BrowserHistory, History},
    prelude::Navigator,
};
use yewdux::prelude::Dispatch;

use crate::{
    api::task_api::{api_create_task, api_delete_task, api_update_task},
    router,
    store::{set_page_loading, set_show_alert, Store},
};

// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

/****** Validation and Creation of Tasks ******/

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateTaskSchema {
    #[validate(
        length(min = 1, message = "Title is required"),
        length(max = 100, message = "Title cannot have more than 100 characters")
    )]
    title: String,
    #[validate(
        length(min = 1, message = "Description is required"),
        length(
            max = 255,
            message = "Description cannot be longer than 255 characters"
        )
    )]
    description: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<CreateTaskSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "title" => data.title = value,
            "description" => data.description = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

fn get_validate_input_callback(
    cloned_form: UseStateHandle<CreateTaskSchema>,
    cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>>,
    id_params: Option<String>,
) -> Callback<(String, String)> {
    Callback::from(move |(name, value): (String, String)| {
        if id_params.is_none() {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "title" => data.title = value,
                "description" => data.description = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        }
    })
}

pub fn get_submit_callback(
    cloned_form: UseStateHandle<CreateTaskSchema>,
    cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>>,
    store_dispatch: Dispatch<Store>,
    cloned_navigator: Navigator,
    cloned_title_input_ref: NodeRef,
    cloned_description_input_ref: NodeRef,
) -> Callback<SubmitEvent> {
    Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let dispatch = store_dispatch.clone();
        let form = cloned_form.clone();
        let validation_errors = cloned_validation_errors.clone();
        let navigator = cloned_navigator.clone();

        let title_input_ref = cloned_title_input_ref.clone();
        let description_input_ref = cloned_description_input_ref.clone();

        spawn_local(async move {
            match form.validate() {
                Ok(_) => {
                    let form_data = form.deref().clone();
                    set_page_loading(true, dispatch.clone());

                    let title_input = title_input_ref.cast::<HtmlInputElement>().unwrap();
                    let description_input =
                        description_input_ref.cast::<HtmlInputElement>().unwrap();

                    title_input.set_value("");
                    description_input.set_value("");

                    let form_json = serde_json::to_string(&form_data).unwrap();
                    let res = api_create_task(&form_json).await;
                    match res {
                        Ok(_) => {
                            set_page_loading(false, dispatch);
                            navigator.push(&router::MainRoute::TasksRoot);
                        }
                        Err(e) => {
                            set_page_loading(false, dispatch.clone());
                            set_show_alert(e.to_string(), dispatch);
                        }
                    };
                }
                Err(e) => {
                    validation_errors.set(Rc::new(RefCell::new(e)));
                }
            }
        });
    })
}

/****** Updating/Editing Tasks ******/

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
struct UpdateTaskSchema {
    title: String,
    description: String,
    completed: bool,
}

fn get_update_title_description_callback(
    name: &'static str,
    cloned_update_task_form: UseStateHandle<UpdateTaskSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_update_task_form.deref().clone();
        match name {
            "title" => data.title = value,
            "description" => data.description = value,
            _ => (),
        }
        cloned_update_task_form.set(data);
    })
}

// Since the Input Checkbox does not have a custom component,
// the capture of the value and its setting in its
// corresponding state are done separately.
fn get_onchange_checkbox_callback(
    cloned_handle_completed_checkbox: Callback<bool>,
) -> Callback<Event> {
    Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().checked();
        // log(&value.to_string());
        cloned_handle_completed_checkbox.emit(value);
    })
}

fn get_update_checkbox_callback(
    cloned_update_task_form: UseStateHandle<UpdateTaskSchema>,
) -> Callback<bool> {
    Callback::from(move |value| {
        let mut data = cloned_update_task_form.deref().clone();
        data.completed = value;
        cloned_update_task_form.set(data);
    })
}

fn get_on_update_callback(
    cloned_update_task_form: UseStateHandle<UpdateTaskSchema>,
    cloned_id: Option<String>,
    store_dispatch: Dispatch<Store>,
    cloned_navigator: Navigator,
) -> Callback<MouseEvent> {
    Callback::from(move |_event: MouseEvent| {
        let update_task_form = cloned_update_task_form.clone();
        let id = cloned_id.clone();
        let dispatch = store_dispatch.clone();
        let navigator = cloned_navigator.clone();

        spawn_local(async move {
            if let Some(id_params) = id {
                set_page_loading(true, dispatch.clone());

                let update_task = UpdateTaskSchema {
                    title: update_task_form.title.to_owned(),
                    description: update_task_form.description.to_owned(),
                    completed: update_task_form.completed,
                };

                let update_task_json = serde_json::to_string(&update_task).unwrap();
                let response = api_update_task(id_params, &update_task_json).await;
                match response {
                    Ok(_) => {
                        set_page_loading(false, dispatch.clone());
                        navigator.push(&router::MainRoute::TasksRoot);
                    }
                    Err(e) => {
                        set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            }
        })
    })
}

/****** Deletion of Tasks ******/

fn get_on_delete_calllback(
    cloned_id: Option<String>,
    store_dispatch: Dispatch<Store>,
    cloned_navigator: Navigator,
) -> Callback<MouseEvent> {
    Callback::from(move |_event: MouseEvent| {
        let id = cloned_id.clone();
        let dispatch = store_dispatch.clone();
        let navigator = cloned_navigator.clone();

        spawn_local(async move {
            if let Some(id_params) = id {
                let response = api_delete_task(id_params).await;
                match response {
                    Ok(_) => {
                        // set_page_loading(false, dispatch.clone());
                        navigator.push(&router::MainRoute::TasksRoot);
                    }
                    Err(e) => {
                        // set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            }
        })
    })
}

/****** Go to Previous Page ******/

fn get_go_back_callback() -> Callback<MouseEvent> {
    Callback::from(|_event: MouseEvent| {
        let history = BrowserHistory::new();
        history.back();
    })
}
