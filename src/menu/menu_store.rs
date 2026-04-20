use crate::utils::input::Input;
use crate::utils::color_text::ColorText;
use crate::feature::download_store;
use crate::feature::download_store::download_session_by_file_fast;

pub fn show() {
    println!("{}", ColorText::green("\n[1] DOWNLOAD STORE DB USE FILE"));
    println!("{}", ColorText::green("[2] DOWNLOAD STORE DB USE NUMBER"));
    println!("{}", ColorText::green("[3] DOWNLOAD STORE DB USE FILE V2"));
    println!("{}", ColorText::red("[4] DOWNLOAD STORE DB MULTI"));

    
    let choice = Input::number("Pilih : ");

    match choice {
        1 => download_store::download_session_by_file(),
        2 => download_store::download_session_by_number(),
        3 => download_session_by_file_fast(),
        // 4 => feature::register::auto_modem(),
        _ => println!("Pilihan tidak valid"),
    }

}