use std::path::PathBuf;

use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use anyhow::{Result, Error};
use rand::prelude::*;
use rfd::{AsyncFileDialog, FileHandle};
use rsa::{pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey}, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use tokio::task;

pub async fn async_encrypt(pub_key: RsaPublicKey, file_path: PathBuf) -> Result<Vec<u8>> {
    let data = async_get_data(file_path).await?;
    let task = move || {
        pub_key
            .encrypt(&mut thread_rng(), Pkcs1v15Encrypt, &data)
            .map_err(|err| Error::new(err))
    };
    task::spawn_blocking(task).await?
}

pub async fn async_decrypt(priv_key: RsaPrivateKey, file_path: PathBuf) -> Result<Vec<u8>> {
    let data = async_get_data(file_path).await?;
    let task = move || {
        priv_key
            .decrypt(Pkcs1v15Encrypt, &data)
            .map_err(|err| Error::new(err))
    };
    task::spawn_blocking(task).await?
}

pub async fn async_generate_priv_key_from_bits(bits: usize) -> Result<RsaPrivateKey> {
    task::spawn_blocking(move || {
        let mut rng = rand::thread_rng();
        let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
        Ok(priv_key)
    }).await?
}

pub async fn async_save_encrypted_file(file_name: String, data: Option<Vec<u8>>) -> Result<()> {
    let file_handle = AsyncFileDialog::new()
        .set_title("Save encrypted file")
        .add_filter("Encrypted file", &["encrypted"])
        .set_file_name(file_name)
        .save_file()
        .await
        .ok_or(Error::msg("Failed to save!"))?;
    let mut file = File::create(file_handle.path()).await?;
    file.write_all(&data.unwrap_or(Vec::default())).await?;
    Ok(())
}

pub async fn async_save_decrypted_file(file_name: String, data: Option<Vec<u8>>) -> Result<()> {
    let file_handle = AsyncFileDialog::new()
        .set_title("Save decrypted file")
        .set_file_name(file_name)
        .save_file()
        .await
        .ok_or(Error::msg("Failed to save!"))?;
    let mut file = File::create(file_handle.path()).await?;
    file.write_all(&data.unwrap_or(Vec::default())).await?;
    Ok(())
}

pub async fn async_save_private_pem_file(data: String) -> Result<()> {
    let file_handle = AsyncFileDialog::new()
        .set_title("Save private pem file")
        .add_filter("Private pem file", &["pem"])
        .set_file_name("private")
        .save_file()
        .await
        .ok_or(Error::msg("Failed to save!"))?;
    let mut file = File::create(file_handle.path()).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}

pub async fn async_save_public_pem_file(data: String) -> Result<()> {
    let file_handle = AsyncFileDialog::new()
        .set_title("Save public pem file")
        .add_filter("Public pem file", &["pem"])
        .set_file_name("public")
        .save_file()
        .await
        .ok_or(Error::msg("Failed to save!"))?;
    let mut file = File::create(file_handle.path()).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}

pub async fn async_pick_file() -> Result<FileHandle> {
    AsyncFileDialog::new()
        .set_title("Pick file")
        .pick_file()
        .await
        .ok_or(Error::msg("Failed to pick!"))
}

pub async fn async_get_data(path: PathBuf) -> Result<Vec<u8>> {
        let mut file = File::open(path).await?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;
        Ok(data)
}

pub async fn async_to_priv_key(text: String) -> Result<RsaPrivateKey> {
    task::spawn_blocking(move || RsaPrivateKey::from_pkcs1_pem(&text)).await?.map_err(|err| Error::new(err))
}

pub async fn async_to_pub_key(text: String) -> Result<RsaPublicKey> {
    task::spawn_blocking(move || RsaPublicKey::from_pkcs1_pem(&text)).await?.map_err(|err| Error::new(err))
}

pub async fn async_priv_key_to_pub_key(private_key: RsaPrivateKey) -> Result<RsaPublicKey> {
    task::spawn_blocking(move || private_key.to_public_key()).await.map_err(|err| Error::new(err))
}

