use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Local};
use validator::ValidationErrors;
use yew::{
    function_component, html, use_effect_with_deps, use_state, virtual_dom::AttrValue, Html,
    NodeRef, Properties,
};
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;

use crate::{
    api::task_api::api_get_single_task,
    components::ui::{
        form_input::FormInput, loading_button::LoadingButton, spinner::Spinner,
        textarea_input::TextareaInput,
    },
    helpers::date_convert,
    layout::Layout,
    router,
    store::{set_show_alert, Store},
};

#[derive(Debug, PartialEq, Properties)]
pub struct TaskFormPageProp {
    pub id: Option<String>,
}

#[function_component(TaskFormPage)]
pub fn task_form_page(TaskFormPageProp { id }: &TaskFormPageProp) -> Html {
    let (store, dispatch) = use_store::<Store>();

    let form = use_state(|| super::CreateTaskSchema::default());
    let update_task_form = use_state(|| super::UpdateTaskSchema::default());
    let date = use_state(|| DateTime::<Local>::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let navigator = use_navigator().unwrap();

    let title_input_ref = NodeRef::default();
    let description_input_ref = NodeRef::default();

    /****** Validation and Creation of Tasks ******/

    let handle_title_input = super::get_input_callback("title", form.clone());
    let handle_description_input = super::get_input_callback("description", form.clone());

    let validate_input_on_blur =
        super::get_validate_input_callback(form.clone(), validation_errors.clone(), id.clone());

    let on_submit = super::get_submit_callback(
        form.clone(),
        validation_errors.clone(),
        dispatch.clone(),
        navigator.clone(),
        title_input_ref.clone(),
        description_input_ref.clone(),
    );

    /****** Editing/Deleting Tasks | Go to Previous Page ******/

    let handle_update_title =
        super::get_update_title_description_callback("title", update_task_form.clone());
    let handle_update_description =
        super::get_update_title_description_callback("description", update_task_form.clone());

    /* ========= */
    // Since the Input Checkbox does not have a custom component,
    // the capture of the value and its setting in its
    // corresponding state are done separately.
    let handle_completed_checkbox = super::get_update_checkbox_callback(update_task_form.clone());

    let onchange_checkbox =
        super::get_onchange_checkbox_callback(handle_completed_checkbox.clone());

    /* ========= */
    let on_update = super::get_on_update_callback(
        update_task_form.clone(),
        id.clone(),
        dispatch.clone(),
        navigator.clone(),
    );

    /* ========= */
    let on_delete = super::get_on_delete_calllback(id.clone(), dispatch.clone(), navigator.clone());

    /* ========= */
    let go_back = super::get_go_back_callback();

    /****** Getting the task from the ID when the component is mounted ******/

    let cloned_id = id.clone();
    let cloned_dispatch = dispatch.clone();
    let cloned_update_task_form = update_task_form.clone();
    let cloned_date = date.clone();
    let cloned_navigator = navigator.clone();

    use_effect_with_deps(
        move |_| {
            let dispatch = cloned_dispatch.clone();
            let update_task_form = cloned_update_task_form.clone();
            let date = cloned_date.clone();
            let navigator = cloned_navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // set_page_loading(true, dispatch.clone());
                if let Some(id_params) = cloned_id {
                    let response = api_get_single_task(id_params).await;
                    match response {
                        Ok(task) => {
                            // set_page_loading(false, dispatch.clone());
                            let single_task = super::UpdateTaskSchema {
                                title: task.title,
                                description: task.description,
                                completed: task.completed,
                            };

                            update_task_form.set(single_task);
                            date.set(task.created_at);
                        }
                        Err(e) => {
                            // set_page_loading(false, dispatch.clone());
                            set_show_alert(e.to_string(), dispatch);
                            navigator.push(&router::MainRoute::TasksRoot);
                        }
                    }
                }
            });
        },
        (),
    );

    html! {
        <Layout>
          <section class="bg-ct-blue-600 py-6 md:py-12 rounded-2xl grid place-items-center">
            <div class="w-11/12 md:w-full lg:w-2/3 md:px-8">

              if id.is_some() {
                <>
                  <h1 class="text-xl md:text-3xl font-bold text-center my-8 text-amber-600">
                    {"Task Detail"}
                  </h1>
                  <div class="mx-auto mt-4 text-sm md:text-lg font-thin md:font-light">
                    <p class="text-sky-300 text-start">
                      {format!("▷ Task ID: {}", id.clone().unwrap())}
                    </p>
                    <p class="text-sky-300 text-start mt-3 mb-2">
                     {format!("▷ Created At: {}", date_convert(*date))}
                    </p>
                  </div>
                </>
              } else {
                <h1 class="text-xl md:text-3xl font-bold text-center my-8 text-amber-600">
                  {"Create a New Task"}
                </h1>
              }

              <form
                onsubmit={on_submit}
                class="w-11/12 md:w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-4 md:p-8 space-y-2 md:space-y-5"
              >
                <FormInput label="Title" name="title" input_type="text" input_ref={title_input_ref}
                  handle_onchange={
                    if id.is_some() {
                      handle_update_title
                    } else {
                      handle_title_input
                    }
                  }
                  errors={&*validation_errors}
                  handle_on_input_blur={validate_input_on_blur.clone()}
                  input_value={if id.is_some() {
                      AttrValue::from(update_task_form.clone().title.to_owned())
                    } else {
                      AttrValue::from(form.clone().title.to_owned())
                    }
                  }
                />

                <TextareaInput label="Description" name="description" input_ref={description_input_ref}
                  handle_onchange={
                    if id.is_some() {
                      handle_update_description
                    } else {
                      handle_description_input
                    }
                  }
                  errors={&*validation_errors}
                  handle_on_input_blur={validate_input_on_blur.clone()}
                  input_value={if id.is_some() {
                      AttrValue::from(update_task_form.clone().description.to_owned())
                    } else {
                      AttrValue::from(form.clone().description.to_owned())
                    }
                  }
                />

                if id.is_some() {
                  <>
                    <label for="completed" class="block text-ct-blue-600 hover:text-sky-400 mb-3 cursor-pointer">
                      {"Completed"}
                    </label>
                    <input
                      type="checkbox"
                      id="completed"
                      checked={update_task_form.clone().completed}
                      class="rounded w-5 h-5 bg-transparent border-gray-300 border-2 checked:accent-emerald-500"
                      onchange={onchange_checkbox}
                    />

                    <div class="mt-3 md:mt-6 grid grid-cols-2 md:grid-cols-3 gap-1 md:gap-2 auto-rows-fr text-xs md:text-base">
                      <button type="button" onclick={go_back}
                        class="bg-purple-600 px-2 py-1 md:px-4 md:py-2 rounded-md flex justify-center items-center gap-2"
                      >
                        <span class="inline-block whitespace-nowrap">{"← Go Back"}</span>
                        <span class="inline-block text-base md:text-2xl font-black text-amber-400">{"◁"}</span>
                      </button>
                      <button type="button" onclick={on_update} class="bg-lime-600 px-2 py-1 md:px-4 md:py-2 rounded-md">
                        if store.page_loading {
                          <div class="flex justify-center items-center gap-1 md:gap-3">
                            <Spinner />
                            <span class="text-slate-500 inline-block">{"Loading..."}</span>
                          </div>
                        } else {
                          <span>{"Save"}</span>
                        }
                        </button>
                      <button type="button" onclick={on_delete} class="bg-red-500 px-2 py-1 md:px-4 md:py-2 rounded-md">{"Delete"}</button>
                    </div>
                  </>
                }

                if id.is_none() {
                  <LoadingButton
                    loading={store.page_loading}
                    text_color={Some("text-ct-blue-600".to_string())}
                  >
                    {"Submit"}
                  </LoadingButton>
                }
              </form>
            </div>
          </section>
        </Layout>
    }
}

/*
 * Equal height rows in CSS Grid Layout. SEE:
 * https://stackoverflow.com/questions/44488357/equal-height-rows-in-css-grid-layout
 * https://tailwindcss.com/docs/grid-auto-rows#basic-usage
 */
