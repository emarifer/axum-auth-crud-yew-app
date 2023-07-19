use yew::{function_component, html, /* use_effect_with_deps, */ Html};
// use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;

use crate::{
    // api::user_api::api_user_info,
    helpers::date_convert,
    layout::Layout,
    // router,
    store::Store,
};

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, _) = use_store::<Store>();
    let user = store.auth_user.clone();
    // let navigator = use_navigator().unwrap();

    // use_effect_with_deps(
    //     move |_| {
    //         let dispatch = dispatch.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             set_page_loading(true, dispatch.clone());
    //             let response = api_user_info().await;
    //             match response {
    //                 Ok(user) => {
    //                     set_page_loading(false, dispatch.clone());
    //                     set_auth_user(Some(user), dispatch);
    //                 }
    //                 Err(e) => {
    //                     set_page_loading(false, dispatch.clone());
    //                     set_show_alert(e.to_string(), dispatch);
    //                     navigator.push(&router::MainRoute::LoginPage);
    //                 }
    //             }
    //         });
    //     },
    //     (),
    // );

    html! {
        <Layout>
          <section class="bg-ct-blue-600 px-8 py-12 rounded-2xl">
            <p class="text-xl md:text-3xl font-bold text-center my-8 text-amber-600">{"Your Profile Page"}</p>
            <div class="bg-zinc-800 p-8 rounded-xl">
              <div class="mx-auto text-slate-500 md:pl-8 text-sm md:text-base">
                if let Some(user) = user {
                  <div class="mt-8">
                      <div class="mb-4">
                        <span class="text-amber-600">{"▷ ID: "}</span><span class="font-light">{user.id}</span>
                      </div>
                      <div class="mb-4">
                        <span class="text-amber-600">{"▷ Username: "}</span><span class="font-light">{user.username}</span>
                      </div>
                      <div class="mb-4">
                        <span class="text-amber-600">{"▷ Email: "}</span><span class="font-light">{user.email}</span>
                      </div>
                      <div class="mb-4">
                        <span class="text-amber-600">{"▷ Created At: "}</span><span class="font-light">{date_convert(user.created_at)}</span>
                      </div>
                  </div>
                } else {
                  <p class="mb-4">{"Loading..."}</p>
                }
              </div>
            </div>
          </section>
        </Layout>
    }
}
