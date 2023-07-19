use yew::{function_component, html, use_effect_with_deps, Html};
use yewdux::functional::use_store;

use crate::{
    api::task_api::api_get_tasks,
    components::tasks::task_card::TaskCard,
    layout::Layout,
    // router,
    store::{set_show_alert, set_tasks_user, Store},
};

fn capitalize(username: String) -> String {
    let mut v: Vec<char> = username.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect::<String>()
}

#[function_component(TasksPage)]
pub fn tasks_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let tasks = store.tasks_user.clone();
    let user = store.auth_user.clone();
    // let navigator = use_navigator().unwrap();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // set_page_loading(true, dispatch.clone());
                let response = api_get_tasks().await;
                match response {
                    Ok(tasks) => {
                        // set_page_loading(false, dispatch.clone());
                        set_tasks_user(Some(tasks), dispatch);
                    }
                    Err(e) => {
                        // set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                        // navigator.push(&router::MainRoute::LoginPage);
                    }
                }
            });
        },
        (),
    );

    html! {
        <Layout>
          <section class="bg-ct-blue-600 px-8 py-12 rounded-2xl">
            <div class="flex flex-col justify-center mx-auto">
                if let Some(user) = user {
                  <h1 class="text-xl md:text-3xl font-bold text-center my-8 text-amber-600">
                    {format!("Welcome back to your Task List, {}!!", capitalize(user.username))}
                  </h1>
                } else {
                  <p class="mb-4">{"Loading..."}</p>
                }

                if let Some(user_tasks) = tasks {
                    if user_tasks.len() > 0 {
                      <ul class="bg-zinc-800 p-8 rounded-xl grid md:grid-cols-2 lg:grid-cols-3 gap-2">
                        {
                          user_tasks.into_iter().map(|task| {
                           html!{
                               <li key={task.id.to_string()}>
                                 <TaskCard
                                   id={task.id}
                                   title={task.title}
                                   description={task.description}
                                   completed={task.completed}
                                 />
                               </li>
                           }
                          }).collect::<Html>()
                        }
                      </ul>
                    } else {
                      <div class="flex gap-1 md:gap-2 justify-center items-center mx-auto">
                        <svg fill="currentColor" class="w-4 md:w-5" viewBox="0 0 16 16">
                          <path
                            d="M5.884 6.68a.5.5 0 1 0-.768.64L7.349 10l-2.233 2.68a.5.5 0 0 0 .768.64L8 \
                            10.781l2.116 2.54a.5.5 0 0 0 .768-.641L8.651 10l2.233-2.68a.5.5 0 0 0-.768-.64L8 9.219l-2.116-2.54z"
                          />
                          <path
                            d="M14 14V4.5L9.5 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2zM9.5 3A1.5 1.5 0 0 0 11 \
                            4.5h2V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h5.5v2z"
                          />
                        </svg>
                        <h3 class="text-center text-xs md:text-xl font-thin md:font-medium">
                          {"No tasks yet, please add a new task"}
                        </h3>
                      </div>
                    }
                } else {
                  <p class="mb-4">{"Loading..."}</p>
                }
            </div>
          </section>
        </Layout>
    }
}

/*
 * CAPITALIZE TEXT. SEE:
 * https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
 */
