use std::io::{self, Write};

pub struct Input;
use crate::utils::color_text::ColorText;

impl Input {
    /// Ambil input biasa
    pub fn text(prompt: &str) -> String {
        print!("{}", ColorText::yellow(prompt));
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Gagal membaca input");

        buffer.trim().to_string()
    }

    /// Ambil input wajib (tidak boleh kosong)
    pub fn required(prompt: &str) -> String {
        loop {
            let input = Self::text(prompt);

            if !input.is_empty() {
                return input;
            }

            println!("Input tidak boleh kosong!");
        }
    }

    /// Ambil input angka (u32)
pub fn number(prompt: &str) -> usize {
    loop {
        let input = Self::text(prompt);

        match input.parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Masukkan angka yang valid!"),
        }
    }
}
}