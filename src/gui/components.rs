pub mod setting {
    use iced::{alignment::{Horizontal, Vertical}, widget::{container, text, text_editor, Column, Row, Scrollable}, Color, Element};
    use crate::gui::{message::Message, state::State};

    fn private_key_input(state: &State) -> Element<Message> {
        let title = text("Private key or RSA bit(default: 2048)");
        let text_editor = text_editor(state.get_priv_key_text_content())
            .on_action(Message::OnPrivateKeyAction)
            .size(16)
            .height(550);
        let private_key_box = Scrollable::new(text_editor).spacing(24);
        let status  = match state.get_private_key().is_some() {
            true => ("Private key is ready!", Color::from_rgb(0., 20., 0.)),
            false => ("Private key isn't ready...", Color::from_rgb(20., 0., 0.)),
        };
        let bottom_text = text(status.0).center().color(status.1).size(20);
        let column = Column::new()
            .align_x(Horizontal::Center)
            .push(title)
            .push(private_key_box)
            .push(bottom_text);
        column.into()
    }
    
    fn public_key_input(state: &State) -> Element<Message> {
        let title = text("Public key");
        let text_editor = text_editor(state.get_pub_key_text_content())
            .on_action(Message::OnPublicKeyAction)
            .size(16)
            .height(550);
        let public_key_box = Scrollable::new(text_editor).spacing(24);
        let status  = match state.get_public_key().is_some() {
            true => ("Public key is ready!", Color::from_rgb(0., 20., 0.)),
            false => ("Public key isn't ready...", Color::from_rgb(20., 0., 0.)),
        };
        let bottom_text = text(status.0).center().color(status.1).size(20);
        let column = Column::new()
            .align_x(Horizontal::Center)
            .push(title)
            .push(public_key_box)
            .push(bottom_text);
        column.into()
    }
    
    pub fn view(state: &State) -> Element<Message> {
        let layer = Row::new()
            .align_y(Vertical::Center)
            .push(
                private_key_input(state),
            )
            .push(
                public_key_input(state),
            );
        let bottom_text = text("\nCtrl+G -> Generate both keys\nCtrl+S -> Save available Keys").size(30);
        let layer = Column::new().push(layer).push(bottom_text).align_x(Horizontal::Center);
        let container = container(layer)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(10);
        container.into()
    }
}

pub mod calculate {
    use std::path::PathBuf;

    use iced::{alignment::Horizontal, widget::{container, text, Column}, Element, Length};
    use crate::gui::{message::Message, state::{State, ViewState}};

    pub fn view(state: &State) -> Element<Message> {
        let default_text = match state.get_view_state() {
            ViewState::Encrypting => "RSA tool - Encrypt",
            ViewState::Decrypting => "RSA tool - Decrypt",
            _ => panic!()
        };
        let f =|path: PathBuf| {
            path
                .to_str()
                .map(|text| text.to_string())
                .unwrap_or(default_text.to_string())
        };
        let center_text = state
            .get_file_path()
            .map(f)
            .unwrap_or(default_text.to_string());
        let content = Column::new()
            .spacing(10) 
            .align_x(Horizontal::Center)
            .push(
                text(center_text)
                    .size(48)
                    .shaping(text::Shaping::Advanced),
            );
        let container = container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(20);
    
        container.into()
    }
}