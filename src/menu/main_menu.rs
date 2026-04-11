//use crate::feature;
use crate::utils::input::Input;
use crate::utils::color_text::ColorText;
use crate::feature::tgz_management;
use crate::menu::menu_store;

pub fn show() {
    loop {
        println!("{}", ColorText::green("\n[1] MANAGE FILE TGZ"));
        println!("{}", ColorText::green("[2] MANAGE store.db"));
        // println!("{}", ColorText::red("[3] Daily Check In"));
        // println!("{}", ColorText::red("[4] Check account"));

        let choice = Input::number("Pilih : ");

        match choice {
            1 => tgz_management::tgz_management(),
            2 => menu_store::show(),
            // 3 => feature::checkin::run(),
            // 4 => feature::account::run(),
            _ => println!("Pilihan tidak valid"),
        }       
    }
}