use tokio::fs;

use crate::utils::AnyError;

const LOCAL_FILES_DIR: &str = ".local";

pub async fn save_file(filename: String, data: &[u8]) -> Result<(), AnyError> {
    let path = format!("{}/{}", LOCAL_FILES_DIR, filename);

    fs::create_dir_all(LOCAL_FILES_DIR).await?;
    fs::write(path, data).await?;

    Ok(())
}

pub async fn get_file(filename: String) -> Result<Vec<u8>, AnyError> {
    let path = format!("{}/{}", LOCAL_FILES_DIR, filename);

    let v = fs::read(path).await?;

    Ok(v)
}

pub fn remove_file(filename: String) -> Result<(), AnyError> {
    todo!()
}

pub fn get_files_match(filename: String, regex: String) -> Vec<u8> {
    todo!()
}

pub fn read_file(filename: String) -> Result<Vec<u8>, AnyError> {
    todo!()
}
