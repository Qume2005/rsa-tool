use std::path::PathBuf;
use iced::widget::text_editor::{self, Action, Content, Edit};
use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, RsaPrivateKey, RsaPublicKey};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Setting,
    Encrypting,
    Decrypting,
}

impl Default for ViewState {
    fn default() -> Self {
        Self::Setting
    }
}

#[derive(Default)]
pub struct State {
    view_state: ViewState,
    priv_key_text_content: text_editor::Content,
    pub_key_text_content: text_editor::Content,
    private_key: Option<RsaPrivateKey>,
    public_key: Option<RsaPublicKey>,
    file_path: Option<PathBuf>,
    encrypt_result: Option<Vec<u8>>,
    decrypt_result: Option<Vec<u8>>,
}

impl State {
    pub fn set_encrypt_result(&mut self, encrypt_result: Option<Vec<u8>>) {
        self.encrypt_result = encrypt_result;
    }
    pub fn set_decrypt_result(&mut self, decrypt_result: Option<Vec<u8>>) {
        self.decrypt_result = decrypt_result;
    }

    pub fn set_file_path(&mut self, path: Option<PathBuf>) {
        self.file_path = path;
    }

    pub fn get_file_path(&self) -> Option<PathBuf> {
        self.file_path.clone()
    }

    pub fn get_file_name(&self) -> String {
        match self.get_file_path() {
            Some(path) => path.file_name().map(|os_str| os_str.to_str().unwrap_or("")).unwrap_or("").to_string(),
            None => String::default()
        }
    }

    pub fn switch_view_state(&mut self) {
        if self.get_view_state() == ViewState::Setting && self.public_key.is_some() {
            self.view_state = ViewState::Encrypting;
            return;
        }
        if self.get_view_state() == ViewState::Encrypting && self.private_key.is_some() {
            self.view_state = ViewState::Decrypting;
            return;
        }
        if self.get_view_state() == ViewState::Setting && self.public_key.is_none() && self.private_key.is_some() {
            self.view_state = ViewState::Decrypting;
            return;
        }
        self.view_state = ViewState::Setting;
    }
    pub fn get_view_state(&self) -> ViewState {
        self.view_state
    }
    pub fn set_private_key(&mut self, private_key: Option<RsaPrivateKey>) {
        self.private_key = private_key;
    }

    pub fn get_private_key(&self) -> Option<RsaPrivateKey> {
        self.private_key.clone()
    }

    pub fn get_priv_key_text_content(&self) -> &text_editor::Content {
        &(self.priv_key_text_content)
    }

    pub fn perform_priv_key_text_content(&mut self, action: text_editor::Action) {
        self.priv_key_text_content.perform(action);
    }

    pub fn clean_priv_key_text_content(&mut self) {
        self.priv_key_text_content = Content::new();
    }

    pub fn fill_private_key(&mut self, private_key: RsaPrivateKey) {
        self.clean_priv_key_text_content();
        for c in format!("{}", private_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF).unwrap().to_string()).chars() {
            self.perform_priv_key_text_content(Action::Edit(Edit::Insert(c)));
        }
        self.private_key = Some(private_key);
    }

    pub fn set_public_key(&mut self, public_key: Option<RsaPublicKey>) {
        self.public_key = public_key;
    }

    pub fn get_public_key(&self) -> Option<RsaPublicKey> {
        self.public_key.clone()
    }

    pub fn get_pub_key_text_content(&self) -> &text_editor::Content {
        &(self.pub_key_text_content)
    }

    pub fn perform_pub_key_text_content(&mut self, action: text_editor::Action) {
        self.pub_key_text_content.perform(action);
    }

    pub fn clean_pub_key_text_content(&mut self) {
        self.pub_key_text_content = Content::new();
    }

    pub fn fill_public_key(&mut self, public_key: RsaPublicKey) {
        self.clean_pub_key_text_content();
        for c in format!("{}", public_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF).unwrap()).chars() {
            self.perform_pub_key_text_content(Action::Edit(Edit::Insert(c)));
        }
        self.public_key = Some(public_key);
    }
}