use yew::{prelude::*, Properties};

use crate::appstate::app_state::AppState;
use crate::components::
    {core_component::CoreView,
     side_screen::SideScreen};

pub struct MainComponent {
    current_state: AppState,
}

pub enum Message {
    ChangeToSide,
    ChangeToCore,
}

impl Component for MainComponent {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        MainComponent { 
            current_state: AppState::Core,
         }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ChangeToSide => {
                self.current_state = AppState::Side;
                true
            }
            Message::ChangeToCore => {
                self.current_state = AppState::Core;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.current_state {
            AppState::Core => {
                html! {
                    <CoreView on_click={ctx.link().callback(|_| Message::ChangeToSide)} />
                }
            }
            AppState::Side => {
                html! {
                    <SideScreen on_return={ctx.link().callback(|_| Message::ChangeToCore)}  />
                }
            }
        }
    }
}