use crate::utils::{color_text::ColorText, file_utils, input::Input, download_store};
use std::{thread, time::Duration};


pub fn download_session_by_file() {

    let _ = file_utils::ensure_dir("STORE/sessions_wa");

    let file = Input::text("INPUT FILE NOMOR : ");
    let numbers = file_utils::read_numbers(&file);

    if numbers.is_empty() {
        println!("{}", ColorText::error("[!] TIDAK ADA NOMOR DI FILE"));
        return;
    }

    print!("{}", ColorText::info("[+] TOTAL NOMOR : "));
    println!("{}", ColorText::green(&format!("{}", numbers.len())));

    let mut i = 1;
    for phone in numbers {
        
        // print!("DOWNLOAD : {}", phone);
        print!("{}", ColorText::info(&format!("[{}] DOWNLOAD : {} ", i, phone)));

        match download_store::download(&phone) {
            Ok(_) => println!("{}", ColorText::success("SUCCESS")),
            Err(e) => println!("{}", ColorText::error(&format!("FAILED {} : {}", phone, e))),
        }

        thread::sleep(Duration::from_millis(500));
        i += 1;
    }
}


pub fn download_session_by_number() {
    loop {
        let _ = file_utils::ensure_dir("STORE/sessions_wa");

            let phone = Input::text("NOMOR : ");

            print!("{}", ColorText::info(&format!("[??] DOWNLOAD : {} ", &phone)));

            match download_store::download(&phone) {
                Ok(_) => println!("{}", ColorText::success("SUCCESS")),
                Err(e) => println!("{}", ColorText::error(&format!("FAILED {} : {}", phone, e))),
            }
    }

}