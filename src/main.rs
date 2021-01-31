use yew::{html, Component, ComponentLink, Html};

pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Increment)>
                    { "Increment" }
                </button>
                <button onclick=self.link.callback(|_| Msg::Decrement)>
                    { "Decrement" }
                </button>

                <p>{ "Current Value: " } <strong>{ self.value }</strong></p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
