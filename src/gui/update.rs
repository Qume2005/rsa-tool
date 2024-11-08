use iced::Task;
use super::{message::{priv_key_to_pub_key_and_fill, save_decrypt_result, save_encrypt_result, text_to_priv_key, text_to_pub_key, Message}, state::State, subscription::on_event};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::OnEvent(event) => {
            on_event(state, event)
        }
        Message::OnPrivateKeyAction(action) => {
            state.perform_priv_key_text_content(action);
            text_to_priv_key(state)
        }
        Message::SetPrivateKey(private_key) => {
            state.set_private_key(Some(private_key));
            priv_key_to_pub_key_and_fill(state)
        }
        Message::FillPrivateKey(private_key) => {
            state.fill_private_key(private_key);
            priv_key_to_pub_key_and_fill(state)
        }
        Message::CleanPrivateKey => {
            state.set_private_key(None);
            Task::none()
        }
        Message::OnPublicKeyAction(action) => {
            state.perform_pub_key_text_content(action);
            text_to_pub_key(state)
        }
        Message::SetPublicKey(public_key) => {
            state.set_public_key(Some(public_key));
            Task::none()
        }
        Message::FillPublicKey(public_key) => {
            state.fill_public_key(public_key);
            Task::none()
        }
        Message::CleanPublicKey => {
            state.set_public_key(None);
            Task::none()
        }
        Message::SetFilePath(path) => {
            state.set_file_path(Some(path));
            Task::none()
        }
        Message::SaveEncryptResult(data) => save_encrypt_result(state.get_file_name(), data),
        Message::SaveDecryptResult(data) => save_decrypt_result(state.get_file_name(), data),
        _ => Task::none() 
    }
}