use cursive::views::{Dialog, TextView};


fn main() {
    let mut siv = cursive::default();

    siv.load_theme_file("themes/dark.toml").unwrap();

    let text_view = TextView::new("Hellow Cursive");
    let dialog = Dialog::around(text_view)
        .title("Cursive").button("Quit", |s| s.quit());

    siv.add_layer(dialog);

    siv.run();
}
