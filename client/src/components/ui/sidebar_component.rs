use wasm_bindgen_futures::spawn_local;
use yew::{
    classes, function_component, html, Callback, Html, MouseEvent, Properties, UseStateHandle,
};
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
pub struct SidebarProps {
    pub activesidebar: UseStateHandle<bool>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
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

    let close_on_pressed = {
        let activesidebar = props.activesidebar.clone();

        Callback::from(move |_| {
            activesidebar.set(!*activesidebar);
        })
    };

    let overlay = if *props.activesidebar {
        html!( <div onclick={close_on_pressed.clone()} class="bg-black opacity-60 fixed top-16 right-0 bottom-0 left-0 z-30"></div>  )
    } else {
        html!()
    };

    let sidebar_style = "fixed top-16 bottom-0 left-0 z-40 w-2/3 p-4 bg-slate-700 \
                               flex flex-col transition-transform -translate-x-full duration-700";
    let sidebar_visible = "translate-x-0";
    let link_style = "text-sm active:text-amber-600 pb-1";

    html! {
        <>
          {overlay}
          <div class={classes!(sidebar_style, if *props.activesidebar { sidebar_visible } else { "" })}>
            <button onclick={close_on_pressed.clone()} class="absolute top-4 right-4 active:bg-slate-800">
              <svg class="w-6" viewBox="0 0 16 16">
                <g stroke="currentColor">
                  <path stroke-width="1" d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 \
                  8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z" />
                </g>
              </svg>
            </button>

            <div class="flex flex-col gap-4 mt-4">
              if user.is_none() {
                <>
                  <div>
                    <Link<MainRoute> to={MainRoute::HomePage} >
                      <span
                        class={classes!(if &location == "/" { "border-b-2 border-amber-600" } else { "" }, link_style)}
                        onclick={close_on_pressed.clone()}
                      >
                        <svg fill="currentColor" class="w-4 inline mr-2" viewBox="0 0 16 16">
                          <path d="M6.5 14.5v-3.505c0-.245.25-.495.5-.495h2c.25 0 .5.25.5.5v3.5a.5.5 0 0 0 \
                          .5.5h4a.5.5 0 0 0 .5-.5v-7a.5.5 0 0 0-.146-.354L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293L8.354 \
                          1.146a.5.5 0 0 0-.708 0l-6 6A.5.5 0 0 0 1.5 7.5v7a.5.5 0 0 0 .5.5h4a.5.5 0 0 0 .5-.5Z"/>
                        </svg>
                        {"Home"}
                      </span>
                    </Link<MainRoute>>
                  </div>

                  <div>
                    <Link<MainRoute> to={MainRoute::RegisterPage}>
                      <span
                        class={classes!(if &location == "/register" { "border-b-2 border-amber-600" } else { "" }, link_style)}
                        onclick={close_on_pressed.clone()}
                      >
                        <svg fill="currentColor" class="w-4 inline mr-2"  viewBox="0 0 640 512">
                          <path d="m624 208h-64v-64c0-8.8-7.2-16-16-16h-32c-8.8 0-16 7.2-16 16v64h-64c-8.8 0-16 7.2-16 \
                          16v32c0 8.8 7.2 16 16 16h64v64c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16v-64h64c8.8 0 16-7.2 \
                          16-16v-32c0-8.8-7.2-16-16-16zm-400 48c70.7 0 128-57.3 128-128s-57.3-128-128-128-128 57.3-128 \
                          128 57.3 128 128 128zm89.6 32h-16.7c-22.2 10.2-46.9 16-72.9 16s-50.6-5.8-72.9-16h-16.7c-74.2 \
                          0-134.4 60.2-134.4 134.4v41.6c0 26.5 21.5 48 48 48h352c26.5 0 48-21.5 \
                          48-48v-41.6c0-74.2-60.2-134.4-134.4-134.4z" />
                        </svg>
                        {"SignUp"}
                      </span>
                    </Link<MainRoute>>
                  </div>

                  <div>
                    <Link<MainRoute> to={MainRoute::LoginPage}>
                      <span
                        class={classes!(if &location == "/login" { "border-b-2 border-amber-600" } else { "" }, link_style)}
                        onclick={close_on_pressed.clone()}
                      >
                        <svg fill="currentColor" class="w-4 inline mr-2" viewBox="0 0 16 16">
                          <path d="M9.293 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V4.707A1 1 0 0 0 \
                          13.707 4L10 .293A1 1 0 0 0 9.293 0zM9.5 3.5v-2l3 3h-2a1 1 0 0 1-1-1zM11 8a3 3 0 1 1-6 0 3 3 0 0 1 6 0zm2 \
                          5.755V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-.245S4 12 8 12s5 1.755 5 1.755z"/>
                        </svg>
                        {"Login"}
                      </span>
                    </Link<MainRoute>>
                  </div>
                </>
              } else {
                <>
                  <div>
                    <Link<MainRoute> to={MainRoute::ProfilePage}>
                      <span
                        class={classes!(if &location == "/profile" { "border-b-2 border-amber-600" } else { "" }, link_style)}
                        onclick={close_on_pressed.clone()}
                      >
                        <svg fill="currentColor" class="w-4 inline mr-2" viewBox="0 0 16 16">
                          <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z" />
                          <path d="M2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 \
                          0 0-2-2H2zm12 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1v-1c0-1-1-4-6-4s-6 3-6 4v1a1 \
                          1 0 0 1-1-1V2a1 1 0 0 1 1-1h12z" />
                        </svg>
                        {"Profile"}
                      </span>
                    </Link<MainRoute>>
                  </div>

                  <div>
                    <Link<MainRoute> to={MainRoute::TasksRoot}>
                      <span
                        class={classes!(if &location == "/tasks" { "border-b-2 border-amber-600" } else { "" }, link_style)}
                        onclick={close_on_pressed.clone()}
                      >
                        <svg fill="currentColor" class="w-4 inline mr-2"  viewBox="0 0 16 16">
                          <path fill-rule="evenodd" d="M2 2.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 \
                          0 0 0 .5-.5V3a.5.5 0 0 0-.5-.5H2zM3 3H2v1h1V3z" />
                          <path d="M5 3.5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5zM5.5 7a.5.5 \
                          0 0 0 0 1h9a.5.5 0 0 0 0-1h-9zm0 4a.5.5 0 0 0 0 1h9a.5.5 0 0 0 0-1h-9z" />
                          <path fill-rule="evenodd" d="M1.5 7a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 \
                          0 1-.5.5H2a.5.5 0 0 1-.5-.5V7zM2 7h1v1H2V7zm0 3.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 \
                          0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5H2zm1 .5H2v1h1v-1z" />
                        </svg>
                        {"Tasks"}
                      </span>
                    </Link<MainRoute>>
                  </div>

                  <div>
                    <Link<MainRoute> to={MainRoute::AddTask}>
                      <span
                        class={classes!(if &location == "/add-task" { "border-b-2 border-amber-600" } else { "" }, link_style)}
                        onclick={close_on_pressed.clone()}
                      >
                        <svg class="w-4 inline mr-2" viewBox="0 0 20 20">
                          <g fill="#ecfeff">
                            <path d="m8.85355 6.14645c.19527.19526.19527.51184 0 .7071l-2 2c-.19526.19527-.51184.19527-.7071 \
                            0l-1-1c-.19527-.19526-.19527-.51184 0-.7071.19526-.19527.51184-.19527.7071 0l.64645.64644 \
                            1.64645-1.64644c.19526-.19527.51184-.19527.7071 0z" />
                            <path d="m8.85355 11.1464c.19527.1953.19527.5119 0 .7072l-2 2c-.19526.1952-.51184.1952-.7071 \
                            0l-1-1c-.19527-.1953-.19527-.5119 0-.7072.19526-.1952.51184-.1952.7071 0l.64645.6465 \
                            1.64645-1.6465c.19526-.1952.51184-.1952.7071 0z" />
                            <path d="m10 7.5c0-.27614.2239-.5.5-.5h3c.2761 0 .5.22386.5.5s-.2239.5-.5.5h-3c-.2761 \
                            0-.5-.22386-.5-.5z" />
                            <path d="m5.5 3c-1.38071 0-2.5 1.11929-2.5 2.5v9c0 1.3807 1.11929 2.5 2.5 \
                            2.5h4.09971c-.16194-.3168-.29407-.6514-.39268-1h-3.70703c-.82843 \
                            0-1.5-.6716-1.5-1.5v-9c0-.82843.67157-1.5 1.5-1.5h9c.8284 0 1.5.67157 \
                            1.5 1.5v3.70703c.3486.09861.6832.23074 1 .39268v-4.09971c0-1.38071-1.1193-2.5-2.5-2.5z" />
                            <path d="m14.5 19c2.4853 0 4.5-2.0147 4.5-4.5s-2.0147-4.5-4.5-4.5-4.5 \
                            2.0147-4.5 4.5 2.0147 4.5 4.5 4.5zm0-7c.2761 0 .5.2239.5.5v1.5h1.5c.2761 \
                            0 .5.2239.5.5s-.2239.5-.5.5h-1.5v1.5c0 .2761-.2239.5-.5.5s-.5-.2239-.5-.5v-1.5h-1.5c-.2761 \
                            0-.5-.2239-.5-.5s.2239-.5.5-.5h1.5v-1.5c0-.2761.2239-.5.5-.5z" />
                          </g>
                        </svg>
                        {"Add Task"}
                      </span>
                    </Link<MainRoute>>
                  </div>

                  <div>
                      <span class="cursor-pointer text-sm hover:text-amber-600 ml-[2px]" onclick={handle_logout}>
                        <svg fill="currentColor" class="w-4 inline mr-2" viewBox="0 0 16 16">
                          <path fill-rule="evenodd" d="M10 12.5a.5.5 0 0 1-.5.5h-8a.5.5 0 0 1-.5-.5v-9a.5.5 0 \
                          0 1 .5-.5h8a.5.5 0 0 1 .5.5v2a.5.5 0 0 0 1 0v-2A1.5 1.5 0 0 0 9.5 2h-8A1.5 1.5 0 0 0 \
                          0 3.5v9A1.5 1.5 0 0 0 1.5 14h8a1.5 1.5 0 0 0 1.5-1.5v-2a.5.5 0 0 0-1 0v2z" />
                          <path fill-rule="evenodd" d="M15.854 8.354a.5.5 0 0 0 0-.708l-3-3a.5.5 0 0 0-.708.708L14.293 \
                          7.5H5.5a.5.5 0 0 0 0 1h8.793l-2.147 2.146a.5.5 0 0 0 .708.708l3-3z" />
                        </svg>
                        {"Logout"}
                      </span>
                  </div>
                </>
              }
            </div>
          </div>
        </>
    }
}

/*
 * Ancho CSS de una etiqueta <span>. VER:
 * https://stackoverflow.com/questions/621401/css-width-of-a-span-tag#621409
 */
