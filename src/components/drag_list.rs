use yew::{Component, ComponentLink, DragEvent, html, Html};
use yew::prelude::*;
use yew::utils::document;
use yew::web_sys::{Element, Node};
use wasm_bindgen::{JsCast, JsValue};
use js_sys::{Array};

use crate::contents::user::User;

const DRAG_OVER_CLASS_NAME: &'static str = "drag-over";

pub enum Msg {
    Ignore,
    Drag(DragEvent),
    Drop(DragEvent),
    DragOver(DragEvent),
    DragLeave(DragEvent),
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub users: Vec<User>,
}

pub struct DragList {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for DragList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
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
                        false
                    }
                    Some(data) => {
                        data.set_drop_effect("move");
                        let id = data.get_data("item-id").unwrap();
                        let d = document();
                        let dragging_element = d.get_element_by_id(&id).unwrap();
                        let dragging_node = dragging_element.dyn_ref::<Node>().unwrap();

                        let dropped_element = e.target().unwrap();
                        let dropped_node = dropped_element.dyn_ref::<Node>();

                        let item_list = d.get_element_by_id("item-list");
                        item_list.unwrap().insert_before(&dragging_node, dropped_node);

                        let class_name = Array::new();
                        class_name.set(0, JsValue::from(DRAG_OVER_CLASS_NAME));

                        let drag_over_target = e.current_target().unwrap();
                        let drag_over_element = drag_over_target.dyn_ref::<Element>().unwrap();
                        drag_over_element.class_list().remove(&class_name);
                        true
                    }
                }
            }
            Msg::DragOver(e) => {
                e.prevent_default();
                let class_name = Array::new();
                class_name.set(0, JsValue::from(DRAG_OVER_CLASS_NAME));

                let drag_over_target = e.current_target().unwrap();
                let drag_over_element = drag_over_target.dyn_ref::<Element>().unwrap();
                drag_over_element.class_list().add(&class_name);
                true
            },
            Msg::DragLeave(e) => {
                let class_name = Array::new();
                class_name.set(0, JsValue::from(DRAG_OVER_CLASS_NAME));

                let drag_over_target = e.current_target().unwrap();
                let drag_over_element = drag_over_target.dyn_ref::<Element>().unwrap();
                drag_over_element.class_list().remove(&class_name);
                true
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <ul id="item-list" class="collection">
                { for self.props.users.iter().map(|user| {
                    html! {
                        <li id={ format!("item-{:?}", user.id) }
                            class="collection-item"
                            draggable=true
                            ondrop=self.link.callback(|e: DragEvent| Msg::Drop(e))
                            ondragover=self.link.callback(|e: DragEvent| Msg::DragOver(e))
                            ondragleave=self.link.callback(|e: DragEvent| Msg::DragLeave(e))
                            ondragstart=self.link.callback(|e: DragEvent| Msg::Drag(e))>
                            { user.username.to_string() }
                        </li>
                    }
                })}
            </ul>
        }
    }
}
