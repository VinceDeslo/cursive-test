use cursive::views::{Dialog, TextView};
use cursive::align::HAlign;

const THEME_PATH: &str = "themes/dark.toml";
const TITLE: &str = "~ Ummon ~";
const UMMON_QUOTE: &str = "\nWe inhabit the In-between\n Stitching small singularities\n Like lattice crystals";
const BUTTON_OPTIONS: [&str; 2] = ["Cancel", "Fishstick!"];

fn main() {
    let mut siv = cursive::default();

    siv.load_theme_file(THEME_PATH).unwrap();

    let text_view = TextView::new(UMMON_QUOTE)
        .h_align(HAlign::Center);
    
        let dialog = Dialog::around(text_view)
        .title(TITLE)
        .button(BUTTON_OPTIONS[0], |s| s.quit())
        .button(BUTTON_OPTIONS[1], |s| s.quit());

    siv.add_layer(dialog);

    siv.run();
}
