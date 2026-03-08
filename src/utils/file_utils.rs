use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;
use walkdir::WalkDir;

/// Membaca seluruh file menjadi String
pub fn read_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Membaca file dan mengembalikan Vec<String> per baris
pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;

    Ok(content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}

/// Menulis string ke file (overwrite)
pub fn write_string(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// Append text ke file
pub fn append_string(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

/// Membuat folder jika belum ada
pub fn ensure_dir(path: &str) -> io::Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

// MENGHITUNG JUMLAH FILE PADA SEBUAH FOLDER
pub fn count_files(path: &str) -> usize {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                .count()
        })
        .unwrap_or(0)
}


// MEAKUKAN MOVE FILE DARI FOLDER A KE B 
pub fn move_files(src: &str, dst: &str, limit: usize) -> std::io::Result<usize> {
    let mut moved = 0;

    for entry in fs::read_dir(src)? {
        if moved >= limit {
            break;
        }

        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap();
            let dest = Path::new(dst).join(filename);

            fs::rename(&path, dest)?;
            moved += 1;
        }
    }

    Ok(moved)
}

// MELAKUKAN GENERATE ZIP NAME
pub fn generate_zip_name(base: &str) -> String {
    let mut count = 1;

    loop {
        let filename = if count == 1 {
            format!("{}.zip", base)
        } else {
            format!("{}_{}.zip", base, count)
        };

        if !Path::new(&filename).exists() {
            return filename;
        }

        count += 1;
    }
}

// generator fodler name


pub fn generate_folder_name(base: &str) -> String {
    let mut count = 1;

    loop {
        let folder = if count == 1 {
            base.to_string()
        } else {
            format!("{}_{}", base, count)
        };

        if !Path::new(&folder).exists() {
            return folder;
        }

        count += 1;
    }
}



// MELAKUKAN ZIP FILE
pub fn zip_folder(src_dir: &str, dst_file: &str) -> std::io::Result<()> {
    let path = Path::new(dst_file);
    let file = File::create(path)?;

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    for entry in it.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(src_dir).unwrap();

        if path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;

            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}

// Membaca file txt
pub fn read_numbers(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap_or_default();

    content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}