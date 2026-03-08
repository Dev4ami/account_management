use colored::*;

pub struct ColorText;

impl ColorText {
    /// Biru
    pub fn blue(text: &str) -> String {
        text.blue().to_string()
    }

    /// Kuning
    pub fn yellow(text: &str) -> String {
        text.yellow().to_string()
    }

    /// Merah
    pub fn red(text: &str) -> String {
        text.red().to_string()
    }

    /// Hijau
    pub fn green(text: &str) -> String {
        text.green().to_string()
    }

    /// Biru + bold
    pub fn info(text: &str) -> String {
        text.blue().bold().to_string()
    }

    /// Warning (Kuning + bold)
    pub fn warning(text: &str) -> String {
        format!("⚠ {}", text).yellow().bold().to_string()
    }

    /// Error (merah + bold)
    pub fn error(text: &str) -> String {
        format!("✖ {}", text).red().bold().to_string()
    }

    /// Success (hijau + bold)
    pub fn success(text: &str) -> String {
        format!("✔ {}", text).green().bold().to_string()
    }
}