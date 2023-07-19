use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties, UseStateHandle};
use yew_router::{
    components::Link,
    hooks::{use_location, use_navigator},
};
use yewdux::functional::use_store;

use crate::{
    api::user_api::api_logout_user,
    router::{self, MainRoute},
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub activesidebar: UseStateHandle<bool>,
}

#[function_component(Header)]
pub fn header_component(props: &HeaderProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap().path().to_string();

    let handle_logout = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let dispatch = store_dispatch.clone();
            let navigator = cloned_navigator.clone();
            spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let res = api_logout_user().await;
                match res {
                    Ok(_) => {
                        set_page_loading(false, dispatch.clone());
                        set_auth_user(None, dispatch.clone());
                        set_show_alert("Logged out successfully".to_string(), dispatch);
                        navigator.push(&router::MainRoute::LoginPage);
                    }
                    Err(e) => {
                        set_show_alert(e.to_string(), dispatch.clone());
                        set_page_loading(false, dispatch);
                    }
                };
            });
        })
    };

    let hamburger_on_pressed = {
        let activesidebar = props.activesidebar.clone();

        Callback::from(move |_| {
            activesidebar.set(!*activesidebar);
        })
    };

    html! {
        <header class="w-full">
          <nav class="h-16 flex justify-between gap-32 md:gap-80 items-center px-8 py-4 bg-slate-800 fixed top-0 left-0 right-0 z-10">
            <div>
              if user.is_none() {
                  <Link<MainRoute> to={MainRoute::HomePage}>
                    <p class="hidden md:block text-xl font-black hover:text-amber-600">{"Tasks Manager"}</p>
                    <p
                      class="md:hidden w-10 h-10 flex justify-center items-center bg-cyan-50 rounded-full p-2 text-xl \
                      border-2 border-amber-600 hover:scale-110 ease-in-out duration-500"
                     >
                      {"📝"}
                    </p>
                  </Link<MainRoute>>
              } else {
                  <Link<MainRoute> to={MainRoute::ProfilePage}>
                    <p class="hidden md:block text-xl font-black hover:text-amber-600">{"Tasks Manager"}</p>
                    <p
                      class="md:hidden w-10 h-10 flex justify-center items-center bg-cyan-50 rounded-full p-2 text-xl \
                      border-2 border-amber-600 hover:scale-110 ease-in-out duration-500"
                     >
                      {"📝"}
                    </p>
                  </Link<MainRoute>>
              }
            </div>

            <ul class="hidden md:flex items-center gap-4">
              if user.is_some() {
                 <>
                  <li class={format!("hover:text-amber-600 {}",
                    if &location == "/profile" { "border-b-2 border-amber-600" } else { "" } )}>
                    <Link<MainRoute> to={MainRoute::ProfilePage}>
                      {"Profile"}
                    </Link<MainRoute>>
                  </li>
                  <li class={format!("hover:text-amber-600 {}",
                    if &location == "/tasks" { "border-b-2 border-amber-600" } else { "" } )}>
                    <Link<MainRoute> to={MainRoute::TasksRoot}>
                    {"Tasks"}
                  </Link<MainRoute>>
                  </li>
                  <li class={format!("hover:text-amber-600 {}",
                    if &location == "/add-task" { "border-b-2 border-amber-600" } else { "" } )}>
                    <Link<MainRoute> to={MainRoute::AddTask}>
                      {"Add Task"}
                  </Link<MainRoute>>
                  </li>
                  <li class="hover:text-amber-600 cursor-pointer" onclick={handle_logout}>
                    {"Logout"}
                  </li>
                </>

              } else {
                <>
                  <li class={format!("hover:text-amber-600 {}",
                    if &location == "/" { "border-b-2 border-amber-600" } else { "" } )}>
                    <Link<MainRoute> to={MainRoute::HomePage}>{"Home"}</Link<MainRoute>>
                  </li>
                  <li class={format!("hover:text-amber-600 {}",
                    if &location == "/register" { "border-b-2 border-amber-600" } else { "" } )}>
                    <Link<MainRoute> to={MainRoute::RegisterPage}>
                      {"SignUp"}
                </Link<MainRoute>>
                  </li>
                  <li class={format!("hover:text-amber-600 {}",
                    if &location == "/login" { "border-b-2 border-amber-600" } else { "" } )}>
                    <Link<MainRoute> to={MainRoute::LoginPage}>
                      {"Login"}
                    </Link<MainRoute>>
                  </li>
                </>
              }
            </ul>

            <div class="block md:hidden" onclick={hamburger_on_pressed}>
              <img class="w-8 cursor-pointer active:bg-sky-600" src="img/hamburger.svg" alt="Hamburger button" />
            </div>
          </nav>
      </header>
    }
}
