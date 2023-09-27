use yew::{prelude::*, Properties};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;



#[wasm_bindgen(module = "/src/functions.js")]
extern "C" {
    #[wasm_bindgen(js_name = "addScroll")]
    fn add_scroll_js();
}

pub fn add_scroll(){
    #[allow(unused_unsafee)]
    unsafe{
        add_scroll_js()
    }
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct CoreView {
    pub on_click: Callback<()>,
}

pub enum Message {
    GoToSide,
}

impl Component for CoreView {
    type Message = Message;
    type Properties = Self;

    fn create(ctx: &Context<Self>) -> Self {
        CoreView {
            on_click: ctx.props().on_click.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg:Self::Message) -> bool {
        match msg{
            Message::GoToSide => {
                self.on_click.emit(());
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <button class="button is-dark" onclick={ctx.link().callback(|_| Message::GoToSide)}>
                {"Go to ios styles"}
            </button>
                <div class="is-flex is-justify-content-center is-align-items-center contenedor">
                    <div id="wrapper" class="columns ">
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-black has-background-white p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-link p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-info p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-success p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-black has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-danger p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-warning p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-black has-background-light p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-light has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-warning has-background-black-bis	 p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                    </div>
                
                </div>
            </>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render{
            add_scroll()
        }
    }
}