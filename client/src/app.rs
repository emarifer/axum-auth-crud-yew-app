use wasm_bindgen::{closure::Closure, JsCast};
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, MouseEvent};
use yew_router::{BrowserRouter, Switch};
use yewdux::functional::use_store;

use crate::{
    components::ui::{
        alert::{AlertComponent, AlertProps},
        spinner::Spinner,
    },
    router::{switch_main, MainRoute},
    store::Store,
};

#[function_component(App)]
pub fn app() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let is_page_loading = store.page_loading.clone();

    let alert_props = AlertProps {
        message,
        delayms: 5000,
    };

    /****** Up Button Handling ******/

    let show_button_to_top = use_state(|| false);

    let scroll_to_top = Callback::from(|_e: MouseEvent| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .document_element()
            .unwrap()
            .scroll_to_with_scroll_to_options(
                web_sys::ScrollToOptions::new()
                    .behavior(web_sys::ScrollBehavior::Smooth)
                    .top(0.0),
            );
    });

    {
        let show_button_to_top = show_button_to_top.clone();

        // Setting the button up when the component is mounted
        use_effect_with_deps(
            move |_| {
                let mut scroll_listener = None;

                if let Some(window) = web_sys::window() {
                    let listener = {
                        // Es necesario crear un scope porque el objeto window es movido al closure
                        let window = window.clone();
                        // Crea un Closure a partir de un Box<dyn Fn> - este tiene que ser 'static
                        Closure::<dyn Fn()>::wrap(Box::new(move || {
                            let scroll_top = window.scroll_y().unwrap();
                            // log(&scroll_top.to_string()); // imprime en consola el valor del scroll_y
                            show_button_to_top.set(scroll_top > 300.0);
                        }))
                    };

                    window
                        .add_event_listener_with_callback(
                            "scroll",
                            listener.as_ref().unchecked_ref(),
                        )
                        .unwrap();

                    scroll_listener = Some(listener)
                }
                // Esta es la función de limpieza(cleanup) del use_effect_with_deps
                move || drop(scroll_listener)
            },
            (), // Tupla de dependencias: No hay, por lo que el use_effect_with_deps solo se hace
                // la primera vez
        );
    }

    let button_to_up = {
        let show_button_to_top = show_button_to_top.clone();

        if *show_button_to_top {
            html! {
                <button onclick={scroll_to_top}
                  class="bg-amber-700 hover:bg-amber-500 rounded-full fixed bottom-4 right-4 p-2 z-50">
                  <svg fill="currentColor" class="w-6" viewBox="0 0 16 16">
                    <path d="m7.247 4.86-4.796 5.481c-.566.647-.106 1.659.753 1.659h9.592a1 \
                    1 0 0 0 .753-1.659l-4.796-5.48a1 1 0 0 0-1.506 0z"/>
                  </svg>
                </button>
            }
        } else {
            html!()
        }
    };

    html! {
        <BrowserRouter>
          <Switch<MainRoute> render={switch_main} />

          if show_alert {
            <AlertComponent
              message={alert_props.message}
              delayms={alert_props.delayms}
            />
          }

          if is_page_loading {
            <div class="pt-4 pl-2 top-[5.5rem] fixed">
              <Spinner width={Some("1.5rem")} height={Some("1.5rem")} color="text-ct-yellow-600" />
            </div>
          }

          {button_to_up}
        </BrowserRouter>
    }
}
