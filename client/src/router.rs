use yew::{html, Html};
use yew_router::{components::Redirect, Routable, Switch};

use crate::pages::{
    home_page::HomePage, login_page::LoginPage, not_found::NotFound, profile_page::ProfilePage,
    register_page::RegisterPage, task_form_page::task_form_page::TaskFormPage,
    tasks_page::TasksPage,
};

#[derive(Clone, PartialEq, Routable)]
pub enum MainRoute {
    #[at("/register")]
    RegisterPage,
    #[at("/login")]
    LoginPage,
    #[at("/tasks")]
    TasksRoot,
    #[at("/tasks/*")]
    Tasks,
    #[at("/add-task")]
    AddTask,
    #[at("/profile")]
    ProfilePage,
    #[at("/")]
    HomePage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum TasksRoute {
    #[at("/tasks")]
    TasksPage,
    #[at("/tasks/:id")]
    TaskEditPage { id: String },
    #[not_found]
    #[at("/tasks/404")]
    NotFound,
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::HomePage => html!(<HomePage />),
        MainRoute::RegisterPage => html!(<RegisterPage />),
        MainRoute::LoginPage => html!(<LoginPage />),
        MainRoute::TasksRoot | MainRoute::Tasks => {
            html!(<Switch<TasksRoute> render={switch_tasks} />)
        }
        MainRoute::AddTask => html!(<TaskFormPage />),
        MainRoute::ProfilePage => html!(<ProfilePage />),
        MainRoute::NotFound => html!(<NotFound />),
    }
}

pub fn switch_tasks(route: TasksRoute) -> Html {
    match route {
        TasksRoute::TasksPage => html! {<TasksPage />},
        TasksRoute::TaskEditPage { id } => html! {<TaskFormPage id={id} />},
        TasksRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>},
    }
}
