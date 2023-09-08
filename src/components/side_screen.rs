use yew::{prelude::*, Properties};

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SideScreen {
    pub on_return: Callback<()>,
}

pub enum Message{
    GoBack,
}

impl Component for SideScreen {
    type Message = Message;
    type Properties = Self;

    fn create(ctx: &Context<Self>) -> Self {
        SideScreen {
            on_return: ctx.props().on_return.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Message::GoBack => {
                self.on_return.emit(());
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button class="button is-dark" onclick={ctx.link().callback(|_| Message::GoBack)}>
                {"Go Back"}
            </button>
        }
    }

}