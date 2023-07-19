use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::types::{Task, User};

/****** Auth Store ******/

pub fn set_auth_user(user: Option<User>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
    })
}

/****** Tasks Store ******/

pub fn set_tasks_user(tasks: Option<Vec<Task>>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.tasks_user = tasks;
    })
}

/****** Store Status & Info ******/

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Store)]
pub struct Store {
    pub auth_user: Option<User>,
    pub tasks_user: Option<Vec<Task>>,
    pub page_loading: bool,
    pub alert_input: AlertInput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

pub fn set_page_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.page_loading = loading;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}
