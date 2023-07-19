use chrono::{Datelike, Utc};
use yew::{function_component, html, use_effect_with_deps, use_state, Children, Html, Properties};
use yew_router::hooks::{use_location, use_navigator};
use yewdux::functional::use_store;

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

use crate::components::ui::{header_component::Header, sidebar_component::Sidebar};

use crate::{
    api::user_api::api_user_info,
    router,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};

#[derive(Debug, Properties, PartialEq)]
pub struct LayoutProp {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(LayoutProp { children }: &LayoutProp) -> Html {
    let (store, dispatch) = use_store::<Store>();
    #[allow(unused)]
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap().path().to_owned();

    let show_sidebar = use_state(|| false);

    /****** User Authentication Check when component is mounted ******/

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();

            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let response = api_user_info().await;
                match response {
                    Ok(user) => {
                        set_page_loading(false, dispatch.clone());
                        set_auth_user(Some(user), dispatch);
                    }
                    Err(e) => {
                        // log(&location);
                        if location.as_str().ends_with("/")
                            || location.as_str().ends_with("/login")
                            || location.as_str().ends_with("/register")
                        {
                            set_page_loading(false, dispatch.clone());
                            return;
                        }
                        set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                        navigator.push(&router::MainRoute::LoginPage);
                    }
                }
            });
        },
        (),
    );

    html! {
        <>
          <Header activesidebar={show_sidebar.clone()} />

          <main class="mt-36 px-6 md:px-12">
            {children.clone()}
          </main>

          <footer class="px-12 my-6 text-center">
            <a
              class="italic tracking-wider hover:text-sky-500 ease-in duration-300 flex justify-center items-center gap-2 md:gap-4"
              href="https://github.com/emarifer?tab=repositories"
              target="_blank"
            >
              {format!("© {} by Enrique Marín", Utc::now().year())}
              <svg fill="currentColor" class="w-4" viewBox="0 0 16 16">
                <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 \
                0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 \
                1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 \
                0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 \
                0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 \
                0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 \
                0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
              </svg>
            </a>
          </footer>

          <Sidebar activesidebar={show_sidebar} />
        </>
    }
}
