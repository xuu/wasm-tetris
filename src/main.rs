#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

#[derive(Debug, Clone)]
enum Brick {
    // Black,
    Gray,
}

use Brick::*;

#[derive(Debug)]
#[allow(dead_code)]
enum BrickDrop {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

use BrickDrop::*;

#[derive(Debug)]
struct BrickDroper {
    drop: BrickDrop,
    coord: [(i64, i64); 4],
}

impl BrickDroper {
    fn new(drop: BrickDrop, init_x: i64) -> BrickDroper {
        let coord = match drop {
            I => [(init_x, -3), (init_x, -2), (init_x, -1), (init_x, 0)],
            J => [
                (init_x + 1, -2),
                (init_x + 1, -1),
                (init_x + 1, 0),
                (init_x, 0),
            ],
            L => [(init_x, -2), (init_x, -1), (init_x, 0), (init_x + 1, 0)],
            O => [(init_x, -1), (init_x + 1, -1), (init_x, 0), (init_x + 1, 0)],
            S => [
                (init_x + 1, -1),
                (init_x + 2, -1),
                (init_x, 0),
                (init_x + 1, 0),
            ],
            T => [
                (init_x, -1),
                (init_x + 1, -1),
                (init_x + 2, -1),
                (init_x + 1, 0),
            ],
            Z => [
                (init_x, -1),
                (init_x + 1, -1),
                (init_x + 1, 0),
                (init_x + 2, 0),
            ],
        };
        BrickDroper { drop, coord }
    }

    fn move_down(&mut self) {
        for (_, y) in self.coord.iter_mut() {
            *y += 1;
        }
    }
}

#[derive(Debug)]
struct Wall {
    bricks: Vec<(usize, usize, Brick)>,
    width: usize,
    height: usize,
    brick_width: u32,
}

impl Wall {
    fn new(width: usize, height: usize, brick_width: u32) -> Wall {
        let mut bricks: Vec<(usize, usize, Brick)> = Vec::new();
        for i in 0..width {
            for j in 0..height {
                bricks.push((i, j, Gray))
            }
        }
        Wall {
            bricks,
            width,
            height,
            brick_width,
        }
    }
}

fn paint(canvas: &CanvasElement, wall: &Wall) {
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.set_fill_style_color("#eee");
    let dist: f64 = wall.brick_width as f64 + 1.0;

    for (x, y, _b) in &wall.bricks {
        context.fill_rect(
            *x as f64 * dist,
            *y as f64 * dist,
            wall.brick_width as f64,
            wall.brick_width as f64,
        );
    }
}

#[derive(Debug)]
struct Animate {
    canvas: CanvasElement,
    wall: Wall,
    droper: BrickDroper,
    time_stamp: f64,
}

impl Animate {
    fn new(canvas: CanvasElement, wall: Wall) -> Animate {
        Animate {
            canvas,
            wall,
            droper: BrickDroper::new(O, 10),
            time_stamp: 0.0,
        }
    }

    fn animate(mut self, time: f64) {
        if time - self.time_stamp > 1000.0 {
            self.droper.move_down();
            js! {
                console.log(@{format!("{:?}", self.droper)});
            }
            paint(&self.canvas, &self.wall);
            self.time_stamp = time;
        }

        window().request_animation_frame(|t| {
            self.animate(t);
        });
    }
}

fn setup_canvas(selector: &str, wall: &Wall) -> CanvasElement {
    let canvas: CanvasElement = document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    canvas.set_width(wall.width as u32 * (wall.brick_width + 1));
    canvas.set_height(wall.height as u32 * (wall.brick_width + 1));
    canvas
}

fn main() {
    let wall = Wall::new(30, 50, 10);
    let canvas = setup_canvas("canvas", &wall);
    let ani = Animate::new(canvas, wall);

    window().request_animation_frame(move |time| {
        ani.animate(time);
    });
}
