use anyhow::Error;
use serde_derive::{Deserialize};
use yew::{Component, ComponentLink, html, Html};
use yew::format::{Json, Nothing};
use yew::services::fetch::*;

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
                // println!("META: {:?}, {:?}", meta, data);
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
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::FetchData)>
                    { "Hoge" }
                </button>

                <ul>
                    { for self.data.iter().map(|user| {
                        html! {
                            <li>
                                { user.username.to_string() }
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
