use cursive::views::{Dialog, TextView, LinearLayout};
use cursive::align::HAlign;
use cursive::view::{Scrollable};

mod assets;

const THEME_PATH: &str = "themes/dark.toml";
const TITLE: &str = "~ Ummon ~";
const UMMON_QUOTE: &str = "\nWe inhabit the In-between\n Stitching small singularities\n Like lattice crystals";
const BUTTON_OPTIONS: [&str; 2] = ["Cancel", "Fishstick!"];

fn main() {
    let mut siv = cursive::default();

    siv.load_theme_file(THEME_PATH).unwrap();

    let full_text: String = [UMMON_QUOTE, assets::ascii::ASCII_LOGO].join("");
    
    let text = TextView::new(full_text)
        .h_align(HAlign::Center);

    let layout = LinearLayout::vertical()
        .child(text)
        .scrollable();

    let dialog = Dialog::around(layout)
        .title(TITLE)
        .button(BUTTON_OPTIONS[0], |s| s.quit())
        .button(BUTTON_OPTIONS[1], |s| s.quit());
    
    siv.add_layer(dialog);

    siv.run();
}
