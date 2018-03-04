extern crate text_ui;
use text_ui::*;

fn main() {
    let greetings : Vec<Pane> = vec!("Hi", "Sup", "Hello").into_iter().map(|g| txt(g.to_owned())).collect();
    let g = vbox(greetings);
    let s = txt("Hello World".to_owned());
    let s2 = vbox(vec!(s.clone(), s.clone()));
    let s3 = hbox(vec!(g.clone(), s2.clone(), g.clone()));
    println!("{}", render_string(&s3));
}