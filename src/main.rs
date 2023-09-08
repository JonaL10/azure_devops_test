use yew::prelude::*;
pub mod components;
pub mod appstate;

use appstate::app_state;
use components::{core_component::*, main_component::MainComponent};


fn main() {
    yew::Renderer::<MainComponent>::new().render();

}
