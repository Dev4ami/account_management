use crate::utils::{color_text::ColorText, file_utils, input::Input};
use chrono::Local;

pub fn tgz_management() {

    let _ = file_utils::ensure_dir("TGZ/MASTER");
    let _ = file_utils::ensure_dir("TGZ/QUEUE");
    let _ = file_utils::ensure_dir("TGZ/SUCCESS");
    let _ = file_utils::ensure_dir("TGZ/FAILED");
    let _ = file_utils::ensure_dir("TGZ/TEMP");
    let _ = file_utils::ensure_dir("TGZ/EXPORT");
    let _ = file_utils::ensure_dir("TGZ/BATCH");

    let total_in_file = file_utils::count_files("TGZ/MASTER");

    print!("{}", ColorText::green("\n[+] FOLDER MASTER : "));
    print!("{}", ColorText::blue("TGZ/MASTER"));
    println!(" {}", ColorText::green(&format!("[{}]", total_in_file)));

    if total_in_file == 0 {
        println!("{}", ColorText::error("[!] TIDAK ADA FILE DI MASTER"));
        return;
    }

    let total_to_manage = Input::number("TOTAL YANG AKAN DI AMBIL : ");

    let take = if total_to_manage > total_in_file {
        total_in_file
    } else {
        total_to_manage
    };

    let name = Input::text("NAME TO : ");
    let date = Local::now().format("%Y%m%d").to_string();

    // MASTER → TEMP
    let moved: usize = file_utils::move_files("TGZ/MASTER", "TGZ/TEMP", take)
        .unwrap_or(0);

    if moved == 0 {
        println!("{}", ColorText::error("[!] TIDAK ADA FILE YANG DIPINDAHKAN"));
        return;
    }

    let base_name = format!("{}_{}_{}", name, date, moved);

    // buat folder batch
    let batch_folder = file_utils::generate_folder_name(&format!("TGZ/BATCH/{}", base_name));
    let _ = file_utils::ensure_dir(&batch_folder);

    // zip dari TEMP
    let zip_name = file_utils::generate_zip_name(&format!("TGZ/EXPORT/{}", base_name));
    file_utils::zip_folder("TGZ/TEMP", &zip_name).ok();

    // TEMP → BATCH
    file_utils::move_files("TGZ/TEMP", &batch_folder, moved).ok();

    print!("{}", ColorText::info("[+] FILE ZIP CREATED :"));
    println!(" {}", ColorText::green(&zip_name));
}