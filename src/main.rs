use crate::utils::logo::banner;

mod utils;
mod menu;
mod feature;


fn main() {
    banner();
    menu::main_menu::show();
}
