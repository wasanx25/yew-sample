#![recursion_limit = "256"]

use anyhow::Error;
use serde_derive::Deserialize;
use wasm_bindgen::JsCast;
use yew::{Component, ComponentLink, DragEvent, html, Html};
use yew::format::{Json, Nothing};
use yew::services::ConsoleService;
use yew::services::fetch::*;
use yew::utils::document;
use yew::web_sys::{Element, Node};

pub enum Msg {
    FetchData,
    FetchReady(Result<Vec<User>, Error>),
    Ignore,
    Drag(DragEvent),
    Drop(DragEvent),
    DragOver(DragEvent),
}

pub struct Model {
    link: ComponentLink<Self>,
    data: Vec<User>,
    fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    id: i64,
    name: String,
    username: String,
    email: String,
    website: String,
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
            Msg::Drag(e) => {
                match e.data_transfer() {
                    None => {}
                    Some(data) => {
                        let id = e.target().unwrap().dyn_ref::<Element>().unwrap().id();
                        data.set_data("item-id", &id);
                    }
                }
                true
            }
            Msg::Drop(e) => {
                match e.data_transfer() {
                    None => {
                        ConsoleService::log("data-transfer-none");
                        false
                    }
                    Some(data) => {
                        ConsoleService::log("data-transfer-some");
                        data.set_drop_effect("move");
                        let id = data.get_data("item-id").unwrap();
                        let d = document();
                        let dragging_element = d.get_element_by_id(&id).unwrap();
                        let dragging_node = dragging_element.dyn_ref::<Node>().unwrap();

                        let dropped_element = e.target().unwrap();
                        let dropped_node = dropped_element.dyn_ref::<Node>();

                        let item_list = d.get_element_by_id("item-list");
                        item_list.unwrap().insert_before(&dragging_node, dropped_node);
                        true
                    }
                }
            }
            Msg::DragOver(e) => {
                e.prevent_default();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="content">
                    <button onclick=self.link.callback(|_| Msg::FetchData)>
                        { "Hoge" }
                    </button>

                    <ul id="item-list">
                        { for self.data.iter().map(|user| {
                            html! {
                                <li id={ format!("item-{:?}", user.id) }
                                    draggable=true
                                    ondrop=self.link.callback(|e: DragEvent| Msg::Drop(e))
                                    ondragover=self.link.callback(|e: DragEvent| Msg::DragOver(e))
                                    ondragstart=self.link.callback(|e: DragEvent| Msg::Drag(e))>
                                    { user.username.to_string() }
                                </li>
                            }
                        })}
                    </ul>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
