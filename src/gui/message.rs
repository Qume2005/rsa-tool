use std::path::PathBuf;

use anyhow::Result;
use iced::{widget::text_editor, Event, Task};
use rfd::FileHandle;
use rsa::{pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey}, RsaPrivateKey, RsaPublicKey};

use crate::core::{async_decrypt, async_encrypt, async_generate_priv_key_from_bits, async_get_data, async_pick_file, async_priv_key_to_pub_key, async_save_decrypted_file, async_save_encrypted_file, async_save_private_pem_file, async_save_public_pem_file, async_to_priv_key, async_to_pub_key};

use super::state::State;

#[derive(Debug, Clone)]
pub enum Message {
    OnEvent(Event),
    OnPrivateKeyAction(text_editor::Action),
    OnPublicKeyAction(text_editor::Action),
    FillPrivateKey(RsaPrivateKey),
    CleanPrivateKey,
    SetPrivateKey(RsaPrivateKey),
    FillPublicKey(RsaPublicKey),
    CleanPublicKey,
    SetPublicKey(RsaPublicKey),
    SetFilePath(PathBuf),
    SaveEncryptResult(Option<Vec<u8>>),
    SaveDecryptResult(Option<Vec<u8>>),
    NoThingToDo,
}


pub fn generate_priv_key_from_bits(bit: usize) -> Task<Message> {
    let f =|result| match result {
        Ok(private_key) => Message::FillPrivateKey(private_key),
        Err(_) => Message::NoThingToDo,
    };
    Task::perform(async_generate_priv_key_from_bits(bit), f)
}

pub fn text_to_priv_key(state: &mut State) -> Task<Message> {
    let f =|result| match result {
        Ok(private_key) => Message::SetPrivateKey(private_key),
        Err(_) => Message::CleanPrivateKey,
    };
    Task::perform(async_to_priv_key(state.get_priv_key_text_content().text()), f)
}

pub fn save_private_key(state: &State) -> Task<Message> {
    let future = async_save_private_pem_file(state.get_priv_key_text_content().text());
    Task::perform(future, |_| Message::NoThingToDo)
}

pub fn text_to_pub_key(state: &mut State) -> Task<Message> {
    let f =|result| match result {
        Ok(public_key) => Message::SetPublicKey(public_key),
        Err(_) => Message::CleanPublicKey,
    };
    Task::perform(async_to_pub_key(state.get_pub_key_text_content().text()), f)
}

pub fn save_public_key(state: &State) -> Task<Message> {
    let future = async_save_public_pem_file(state.get_pub_key_text_content().text());
    Task::perform(future, |_| Message::NoThingToDo)
}

pub fn priv_key_to_pub_key_and_fill(state: &mut State) -> Task<Message> {
    let future = async_priv_key_to_pub_key(state.get_private_key().unwrap());
    let f = |result: Result<RsaPublicKey>| Message::FillPublicKey(result.unwrap());
    Task::perform(future, f)
}

pub fn get_key(path: PathBuf) -> Task<Message> {
    let future = async_get_data(path);
    let f = |result| {
        match result {
            Ok(data) => {
                match String::from_utf8(data) {
                    Ok(text) => match (RsaPrivateKey::from_pkcs1_pem(&text), RsaPublicKey::from_pkcs1_pem(&text)) {
                        (Ok(private_key), _) => Message::FillPrivateKey(private_key),
                        (_, Ok(public_key)) => Message::FillPublicKey(public_key),
                        _ => Message::NoThingToDo,
                    }
                    Err(_) => Message::NoThingToDo,
                }
            }
            Err(_) => Message::NoThingToDo
        }
    };
    Task::perform(future, f)
}

pub fn encrypt(state: &mut State) -> Task<Message> {
    match state.get_public_key() {
        Some(public_key) => {
            let future = async_encrypt(public_key, state.get_file_path().unwrap());
            let f = |encrypt_result| {
                match encrypt_result {
                    Ok(data) => Message::SaveEncryptResult(Some(data)),
                    _ => {
                        Message::SaveEncryptResult(Some("Failed! File too big or bit too low".as_bytes().to_vec()))
                    },
                }
            };
            Task::perform(future, f)
        }
        None => {
            state.set_encrypt_result(None);
            Task::none()
        }
    }
}

pub fn decrypt(state: &mut State) -> Task<Message> {
    match state.get_private_key() {
        Some(private_key) => {
            let future = async_decrypt(private_key, state.get_file_path().unwrap());
            let f = |decrypt_result| {
                match decrypt_result {
                    Ok(data) => Message::SaveDecryptResult(Some(data)),
                    Err(_) => {
                        Message::SaveDecryptResult(Some("Failed! File too big or bit too low".as_bytes().to_vec()))
                    },
                }
            };
            Task::perform(future, f)
        }
        None => {
            state.set_decrypt_result(None);
            Task::none()
        }
    }
}

pub fn save_encrypt_result(file_name: String, data: Option<Vec<u8>>) -> Task<Message> {
    let future = async_save_encrypted_file(file_name, data);
    let f = |_| Message::NoThingToDo;
    Task::perform(future, f)
}

pub fn save_decrypt_result(file_name: String, data: Option<Vec<u8>>) -> Task<Message> {
    let future = async_save_decrypted_file(file_name.replace(".encrypted", ""), data);
    let f = |_| Message::NoThingToDo;
    Task::perform(future, f)
}

pub fn pick_file() -> Task<Message> {
    let future = async_pick_file();
    let f = |file_handle: Result<FileHandle>| {
        match file_handle {
            Ok(file_handle) => Message::SetFilePath(file_handle.path().to_path_buf()),
            _ => Message::NoThingToDo,
        }
    };
    Task::perform(future, f)
}