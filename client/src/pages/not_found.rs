use yew::{function_component, html, Callback, Html};
use yew_router::history::{BrowserHistory, History};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let go_back = Callback::from(|_| {
        let history = BrowserHistory::new();
        history.back();
    });

    html! {
        <div class="text-center mt-32 mb-14 md:mt-80 md:mb-40">
          <h1 class="text-6xl text-indigo-700 font-extrabold mx-auto mb-16">
            {"Error 404: Not Found"}
          </h1>

          <button
            onclick={go_back}
            class="bg-rose-700 rounded-lg w-auto h-auto mx-auto text-lg p-1 duration-300 hover:scale-90"
          >
            <span class="block bg-neutral-900 py-4 px-6 rounded-md duration-300 hover:bg-transparent">
              {"← Go Back"}
              <span class="ml-6 text-2xl font-black text-amber-400">{"◁"}</span>
            </span>
          </button>
        </div>
    }
}
