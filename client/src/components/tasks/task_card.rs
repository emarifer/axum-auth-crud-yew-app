use uuid::Uuid;
use yew::{function_component, html, Html, Properties};
use yew_router::components::Link;

use crate::router::TasksRoute;

#[derive(Debug, Properties, PartialEq)]
pub struct TaskCardProps {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[function_component(TaskCard)]
pub fn task_card(props: &TaskCardProps) -> Html {
    html! {
        <Link<TasksRoute>
          to={TasksRoute::TaskEditPage { id: props.id.to_string() }}
          classes="block bg-primary shadow-lg shadow-black border border-gray-600 px-6 py-3 rounded-md hover:-translate-y-1.5 ease-in duration-300">

          <header class="mb-2">
            <h3 title={props.description.clone()} class="text-base md:text-lg text-amber-600">
              {&props.title}
            </h3>
          </header>

          <main class="grid grid-cols-6 gap-4">
            <p title={props.description.clone()}
              class="block col-span-5 text-sm md:text-base text-slate-500 font-semibold whitespace-nowrap overflow-hidden text-ellipsis">
              {&props.description}
           </p>
           <input
             type="checkbox"
             checked={props.completed}
             class={
               format!("pointer-events-none rounded w-5 h-5 bg-transparent border-amber-600 border-2 checked:accent-emerald-500 {}",
                 if !props.completed { "appearance-none" } else { "" }
               )
             }
           />
          </main>
        </Link<TasksRoute>>
    }
}
