#![recursion_limit = "256"]
mod components;
mod contents;

use anyhow::Error;
use yew::{Component, ComponentLink, html, Html};
use yew::format::{Json, Nothing};
use yew::services::fetch::*;
use components::drag_list::DragList;
use contents::user::User;

pub enum Msg {
    FetchData,
    FetchReady(Result<Vec<User>, Error>),
    Ignore,
}

pub struct Model {
    link: ComponentLink<Self>,
    data: Vec<User>,
    fetch_task: Option<FetchTask>,
}

impl Model {
    fn fetch(&mut self) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<Vec<User>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Ignore
                }
            },
        );

        let request = Request::get("https://jsonplaceholder.typicode.com/users").body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, data: Vec::new(), fetch_task: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchData => {
                let task = self.fetch();
                self.fetch_task = Some(task);
                true
            }
            Msg::FetchReady(response) => {
                self.data = response.ok().unwrap();
                true
            }
            Msg::Ignore => {
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let users = &self.data;
        html! {
            <div class="container">
                <div class="user">
                    <button onclick=self.link.callback(|_| Msg::FetchData)>
                        { "Hoge" }
                    </button>

                    <DragList users=users />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
