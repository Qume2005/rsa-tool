use iced::{Event, Task};

use super::{message::Message, state::State};

pub fn on_event(state: &mut State, event: Event) -> Task<Message> {
    match event {
        Event::Keyboard(event) => keyboard::on_event(state, event),
        Event::Window(event) => window::on_event(state, event),
        _ => Task::none(),
    }
}

mod keyboard {
    use iced::{keyboard::{key::Named, Event, Key, Modifiers}, Task};
    use crate::gui::{message::{decrypt, encrypt, generate_priv_key_from_bits, pick_file, save_private_key, save_public_key, Message}, state::{State, ViewState}};

    pub fn on_event(state: &mut State, event: Event) -> Task<Message> {
        match event {
            Event::KeyPressed { key, modifiers, ..} => on_key_pressed(state, key, modifiers),
            _ => Task::none(),
        }
    }
    fn on_key_pressed(state: &mut State, key: Key, modifiers: Modifiers) -> Task<Message> {
        match state.get_view_state() {
            ViewState::Setting => {
                match key {
                    Key::Character(character) => {
                        if !modifiers.control() && !modifiers.command() {
                            return Task::none();
                        }
                        match character.as_str() {
                            "g" | "G" => {
                                let bit = state
                                    .get_priv_key_text_content()
                                    .text()
                                    .trim()
                                    .trim_matches('g')
                                    .trim_matches('G')
                                    .parse()
                                    .unwrap_or(2048);
                                generate_priv_key_from_bits(bit)
                            },
                            "s" | "S" => {
                                match (state.get_private_key().is_some(), state.get_public_key().is_some()) {
                                    (true, true) => save_private_key(state).chain(save_public_key(state)),
                                    (true, false) => save_private_key(state),
                                    (false, true) => save_public_key(state),
                                    (false, false) => Task::none(),
                                }
                            }
                            _ => Task::none()
                        }
                    }
                    Key::Named(named) => {
                        match named {
                            Named::Tab => {
                                state.switch_view_state();
                                Task::none()
                            }
                            _ => Task::none(),
                        }
                    }
                    _ => Task::none()
                }
            }
            ViewState::Encrypting => {
                match key {
                    Key::Character(character) => {
                        if !modifiers.control() && !modifiers.command() {
                            return Task::none();
                        }
                        match character.as_str() {
                            "s" | "S" => {
                                encrypt(state)
                            }
                            "o" | "O" => {
                                pick_file()
                            }
                            _ => Task::none()
                        }
                    }
                    Key::Named(named) => {
                        match named {
                            Named::Tab => {
                                state.set_file_path(None);
                                state.switch_view_state();
                                Task::none()
                            }
                            Named::Enter => {
                                encrypt(state)
                            }
                            _ => Task::none(),
                        }
                    }
                    _ => Task::none()
                }
            }
            ViewState::Decrypting => {
                match key {
                    Key::Character(character) => {
                        if !modifiers.control() && !modifiers.command() {
                            return Task::none();
                        }
                        match character.as_str() {
                            "s" | "S" => {
                                decrypt(state)
                            }
                            "o" | "O" => {
                                pick_file()
                            }
                            _ => Task::none()
                        }
                    }
                    Key::Named(named) => {
                        match named {
                            Named::Tab => {
                                state.set_file_path(None);
                                state.switch_view_state();
                                Task::none()
                            }
                            Named::Enter => {
                                decrypt(state)
                            }
                            _ => Task::none(),
                        }
                    }
                    _ => Task::none()
                }
            }
        }
    }
}

mod window {
    use iced::{window::Event, Task};
    use crate::gui::{message::{get_key, Message}, state::{State, ViewState}};

    pub fn on_event(state: &mut State, event: Event) -> Task<Message> {
        match state.get_view_state() {
            ViewState::Setting => {
                match event {
                    Event::FileDropped(path) => get_key(path),
                    _ => Task::none(),
                }
            }
            ViewState::Encrypting => {
                match event {
                    Event::FileDropped(path) => {
                        if *&path.read_dir().is_err() {
                            state.set_file_path(Some(path.clone()));
                        }
                        Task::none()
                    },
                    _ => Task::none(),
                }
            }
            ViewState::Decrypting => {
                match event {
                    Event::FileDropped(path) => {
                        if *&path.read_dir().is_err() {
                            state.set_file_path(Some(path.clone()));
                        }
                        Task::none()
                    },
                    _ => Task::none(),
                }
            }
        }
    }
}