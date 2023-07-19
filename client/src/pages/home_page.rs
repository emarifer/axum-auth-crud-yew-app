use yew::{function_component, html, Html};

use crate::layout::Layout;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <Layout>
          <>
            <h1 class="w-5/6 md:w-full mt-8 md:mt-16 text-xl md:text-4xl font-bold text-center text-amber-600 mx-auto md:tracking-wide">
              {"Home"}
            </h1>

            <hr class="w-80 md:w-2/3 mx-auto border border-zinc-500 mt-4 mb-2 md:mt-12 md:mb-4" />

            <div class="w-72 md:w-1/2 mx-auto text-zinc-500">
              <h3>
                {"Task Manager is a Fullstack application completely made in Rust. Allows authentication with a \
                cloud database (Supabase) using JsonWebToken. Once the authentication is done, \
                the user can read, create, update and delete his own tasks (CRUD to the database)."}
              </h3>

              <p class="text-center my-4 tracking-[.75rem] md:tracking-[2rem] mx-auto">{"* * *"}</p>

              <p class="text-center font-medium text-amber-600 mx-auto">{"We hope you enjoy the experience!"}</p>
            </div>

            <hr class="w-80 md:w-2/3 mx-auto border border-zinc-500 mt-2 mb-4 md:mt-4 md:mb-12" />
          </>
        </Layout>
    }
}
