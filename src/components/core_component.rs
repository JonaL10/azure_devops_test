use yew::{prelude::*, Properties};

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
            <button class="button is-dark" onclick={ctx.link().callback(|_| Message::GoToSide)}>
                {"Go to side screen"}
            </button>
        }
    }
}