use gloo::timers::callback::Timeout;
use yew::{function_component, html, use_effect_with_deps, Html, Properties};
use yewdux::functional::use_store;

use crate::store::{set_hide_alert, Store};

#[derive(Debug, PartialEq, Properties)]
pub struct AlertProps {
    pub message: String,
    pub delayms: u32,
}

#[function_component(AlertComponent)]
pub fn alert_component(props: &AlertProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let show_alert = store.alert_input.show_alert;

    use_effect_with_deps(
        move |(show_alert, dispatch, delayms)| {
            let cloned_dispatch = dispatch.clone();
            if *show_alert {
                let handle =
                    Timeout::new(*delayms, move || set_hide_alert(cloned_dispatch)).forget();
                let clear_handle = move || {
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap(),
                        handle.as_f64().unwrap() as i32,
                    );
                };

                Box::new(clear_handle) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        },
        (show_alert, dispatch.clone(), props.delayms),
    );

    html! {
        <div id="myToast"
          class={format!("fixed top-[5.5rem] right-10 px-5 py-4 border-r-8 border-orange-500 bg-white drop-shadow-lg {}",
            if show_alert { "" } else { "hidden" })}>
          <p class="text-sm text-ct-dark-600">
            <span class="mr-2 inline-block px-3 py-1 rounded-full bg-blue-500 text-white font-extrabold">{"i"}</span>
            {props.message.clone()}
          </p>
        </div>
    }
}
