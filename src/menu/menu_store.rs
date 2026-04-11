use crate::utils::input::Input;
use crate::utils::color_text::ColorText;
use crate::feature::download_store;


pub fn show() {
    println!("{}", ColorText::green("\n[1] DOWNLOAD STORE DB USE FILE"));
    println!("{}", ColorText::green("[2] DOWNLOAD STORE DB USE NUMBER"));
    println!("{}", ColorText::red("[3] DOWNLOAD STORE DB MULTI"));

    
    let choice = Input::number("Pilih : ");

    match choice {
        1 => download_store::download_session_by_file(),
        2 => download_store::download_session_by_number(),
        // 3 => feature::register::auto_api_web2(),
        // 4 => feature::register::auto_modem(),
        _ => println!("Pilihan tidak valid"),
    }

}