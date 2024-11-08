use iced::Element;

use super::{components::{calculate, setting}, message::Message, state::{State, ViewState}};

pub fn view(state: &State) -> Element<Message> {
    match state.get_view_state() {
        ViewState::Setting => setting::view(state),
        _ => calculate::view(state),
    }
}