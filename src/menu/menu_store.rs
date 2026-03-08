use crate::utils::input::Input;
use crate::utils::color_text::ColorText;
use crate::feature::download_store;


pub fn show() {
    println!("{}", ColorText::green("\n[1] DOWNLOAD STORE DB 1 by 1"));
    println!("{}", ColorText::red("[2] DOWNLOAD STORE DB MULTI"));

    
    let choice = Input::number("Pilih : ");

    match choice {
        1 => download_store::download_session(),
        2 => println!("Pilihan tidak valid"),
        // 3 => feature::register::auto_api_web2(),
        // 4 => feature::register::auto_modem(),
        _ => println!("Pilihan tidak valid"),
    }

}