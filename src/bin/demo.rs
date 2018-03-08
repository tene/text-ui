extern crate text_ui;
use text_ui::widget::{Text, VBox, HBox};
use text_ui::backend::{Widget, TextGrid};

fn main() {
    let g1 = Text::from_string("Hi".to_owned());
    let g2 = Text::from_string("Hello".to_owned());
    let v1 = VBox::from_pair(Box::new(g1), Box::new(g2));
    let n1 = Text::from_string("Eve".to_owned());
    let n2 = Text::from_string("Chel".to_owned());
    let n3 = Text::from_string("Susan".to_owned());
    let mut v2 = VBox::from_pair(Box::new(n1), Box::new(n2));
    (&mut v2).append(Box::new(n3));
    let b  = HBox::from_pair(Box::new(v1), Box::new(v2));
    let tg: TextGrid = b.render();
    println!("{}", tg.to_string());
}