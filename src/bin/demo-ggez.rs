extern crate ggez;
extern crate text_ui;
use ggez::*;
use ggez::graphics::{Point2};
use text_ui::backend::ggez as backend;
use text_ui::backend::{Widget, GGBuilder, GGState};
use text_ui::widget::{VBox, HBox, Wrap};

pub fn main() {
    let c = conf::Conf::new();
    let mut ctx = Context::load_from_conf("super_simple", "ggez", c).unwrap();

    let mut events = event::Events::new(&ctx).unwrap();
    let mut continuing = true;

    while continuing {
        // Tell the timer stuff a frame has happened.
        // Without this the FPS timer functions and such won't work.
        //ctx.timer_context.tick();
        // Handle events
        for event in events.poll() {
            match event {
                event::Event::Quit { .. } |
                event::Event::KeyDown { keycode: Some(event::Keycode::Escape), .. } => {
                    println!("Quitting");
                    continuing = false
                }
                x => println!("Event fired: {:?}", x),
            }
        }
        let state = crap(&mut ctx);
        graphics::clear(&mut ctx);
        let _ = backend::draw(&mut ctx, Point2::new(0.0, 0.0), &state);
        graphics::present(&mut ctx);
        ggez::timer::yield_now();
    }
}

fn txt(ctx: &mut Context, s: &str) -> GGState {
    let font = ctx.default_font.clone();
    let t = graphics::Text::new(ctx, s, &font)
        .expect("Failed to render text");
    GGState::text(t)
}

fn crap(ctx: &mut Context) -> GGState {
    let mut bldr = GGBuilder::new();
    let g1 = Wrap::new(txt(ctx, "Hi"));
    let g2 = Wrap::new(txt(ctx, "Hello"));
    let v1: VBox<GGBuilder> = VBox::from_pair(Box::new(g1), Box::new(g2));
    let n1 = Wrap::new(txt(ctx, "Eve"));
    let n2 = Wrap::new(txt(ctx, "Chel"));
    let n3 = Wrap::new(txt(ctx, "Susan"));
    let mut v2 = VBox::from_pair(Box::new(n1), Box::new(n2));
    (&mut v2).append(Box::new(n3));
    let b  = HBox::from_pair(Box::new(v1), Box::new(v2));
    let blah = b.build_with(&mut bldr);
    blah
}