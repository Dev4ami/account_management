use std::fs::{self, File};
use std::io::Write;
use reqwest::blocking::get;

pub fn download(phone: &str) -> Result<(), Box<dyn std::error::Error>> {

    let url = format!(
        "https://waweb.kryptonproject.my.id/api/download?phone={}",
        phone
    );

    let response = get(&url)?;

    let folder = format!("STORE/sessions_wa/{}", phone);

    fs::create_dir_all(&folder)?;

    let path = format!("{}/store.db", folder);

    let mut file = File::create(path)?;
    let bytes = response.bytes()?;

    file.write_all(&bytes)?;

    Ok(())
}