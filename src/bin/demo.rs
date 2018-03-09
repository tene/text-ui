extern crate text_ui;
use text_ui::widget::{demo_widget};
use text_ui::backend::{Widget, TextGrid, TextBuilder};

fn main() {
    let w = demo_widget();
    let mut bldr = TextBuilder::new();
    let tg: TextGrid = w.build_with(&mut bldr);
    println!("{}", tg.to_string());
}