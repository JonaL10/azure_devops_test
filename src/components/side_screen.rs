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
            <>
                <button class="button is-dark" onclick={ctx.link().callback(|_| Message::GoBack)}>
                    {"Go Back"}
                </button>
                <div id="wrapper-ios" class="is-flex is-justify-content-center is-align-items-center">
                    <div  class="columns wrapper-child is-multiline">
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-black has-background-white p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-link p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-info p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-success p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-black has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-danger p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-warning p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-black has-background-light p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-light has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-warning has-background-black-bis	 p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                        <div class="column is-full has-text-white has-background-primary p-3" style="border-radius: 8px;">{"Column"}</div>
                    </div>
                </div>
            </>
        }
    }

}