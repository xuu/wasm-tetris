#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
enum Brick {
    Black,
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

type Coord = [(isize, isize); 4];

#[derive(Debug)]
struct BrickDroper {
    drop: BrickDrop,
    coord: Coord,
    max_x: isize,
    max_y: isize,
    end: bool,
}

impl BrickDroper {
    fn new(drop: BrickDrop, max_x: isize, max_y: isize) -> BrickDroper {
        let init_x = max_x / 2 - 1;
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
        BrickDroper {
            drop,
            coord,
            max_x,
            max_y,
            end: false,
        }
    }

    fn move_down(&mut self) {
        if self.end {
            return;
        }
        for (_, y) in self.coord.iter_mut() {
            *y += 1;
            if *y == self.max_y - 1 {
                self.end = true;
            }
        }
    }

    fn move_left(&mut self) {
        if self.coord.iter().any(|c| c.0 < 1) {
            return;
        }
        for (x, _) in self.coord.iter_mut() {
            *x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.coord.iter().any(|c| c.0 > self.max_x - 2) {
            return;
        }
        for (x, _) in self.coord.iter_mut() {
            *x += 1;
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

struct Animate {
    canvas: CanvasElement,
    wall: Wall,
    time_stamp: f64,
}

impl Animate {
    fn new(canvas: CanvasElement, wall: Wall) -> Animate {
        Animate {
            canvas,
            wall,
            time_stamp: 0.0,
        }
    }

    fn run(mut self, droper: Rc<RefCell<BrickDroper>>, time: f64) {
        if time - self.time_stamp > 1000.0 {
            self.time_stamp = time;
            let drc = droper.clone();
            self.paint(drc);
            // let will_callapse = self.droper
            //     .coord
            //     .iter()
            //     .map(|c| {
            //         self.wall
            //             .bricks
            //             .get(c.0 as usize * self.wall.width + (c.1 as usize + 1) * self.wall.height)
            //     })
            //     .any(|bbb| match bbb {
            //         Some((_, _, Black)) => true,
            //         _ => false,
            //     });

            // if will_callapse || self.droper.end {
            //     self.wall.bricks = new_bricks;
            // }
        }

        window().request_animation_frame(|t| {
            self.run(droper, t);
        });
    }

    fn paint(&self, droper: Rc<RefCell<BrickDroper>>) {
        let mut d = droper.borrow_mut();
        d.move_down();
        let context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
        let dist: f64 = self.wall.brick_width as f64 + 1.0;
        let bricks = self.wall.bricks.clone().into_iter().map(|b| {
            if d
                .coord
                .iter()
                .any(|c| c.0 == b.0 as isize && c.1 == b.1 as isize)
            {
                (b.0, b.1, Black)
            } else {
                b
            }
        });

        for (x, y, b) in bricks {
            let color = match b {
                Gray => "#eee",
                Black => "#333",
            };
            context.set_fill_style_color(color);
            context.fill_rect(
                x as f64 * dist,
                y as f64 * dist,
                self.wall.brick_width as f64,
                self.wall.brick_width as f64,
            );
        }
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
    let droper = Rc::new(RefCell::new(BrickDroper::new(T, 30, 50)));
    let droper_event = Rc::clone(&droper);
    let droper_down = Rc::clone(&droper);
    let canvas = setup_canvas("canvas", &wall);
    let a = Animate::new(canvas, wall);

    window().add_event_listener(move |e: KeyDownEvent| {
        js! {
            console.log(@{format!("{:?}", e.key())})
        }
        let mut d = droper_event.borrow_mut();
        match e.key().as_str() {
            "ArrowRight" => d.move_right(),
            "ArrowLeft" => d.move_left(),
            &_ => (),
        }
    });

    window().request_animation_frame(move |time| {
        a.run(droper_down, time);
    });
}
