use anyhow::Result;
use iced::{application, event::listen_with};
use anyhow::Error;

use super::{message::Message, update::update, view::view};

pub fn run() -> Result<()> {
    application("RSA tool", update, view)
        .subscription(|_| listen_with(|event, _status, _id| Some(Message::OnEvent(event))))
        .run()
        .map_err(|err| Error::new(err))
}