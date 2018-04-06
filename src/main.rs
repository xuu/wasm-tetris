// #[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

#[derive(Debug)]
struct BrickDroper {
    drop: BrickDrop,
    coord: [(isize, isize); 4],
    limit_x: isize,
    limit_y: isize,
}

impl BrickDroper {
    fn new(drop: BrickDrop, limit_x: isize, limit_y: isize) -> BrickDroper {
        let init_x = limit_x / 2 - 1;
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
        BrickDroper { drop, coord, limit_x, limit_y }
    }

    fn move_down(&mut self) {
        for (_, y) in self.coord.iter_mut() {
            *y += 1;
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
        if self.coord.iter().any(|c| c.0 > self.limit_x - 2) {
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

#[derive(Debug)]
struct Animate {
    canvas: CanvasElement,
    wall: Wall,
    droper: BrickDroper,
    time_stamp: f64,
}

impl Animate {
    fn new(canvas: CanvasElement, wall: Wall) -> Animate {
        let limit_x = wall.width as isize;
        let limit_y = wall.height as isize;
        Animate {
            canvas,
            wall,
            droper: BrickDroper::new(T, limit_x, limit_y),
            time_stamp: 0.0,
        }
    }

    fn animate(mut self, time: f64) {
        if time - self.time_stamp > 1000.0 {
            self.droper.move_down();
            self.droper.move_left();
            self.paint();
            self.time_stamp = time;
        }

        window().request_animation_frame(|t| {
            self.animate(t);
        });
    }

    fn paint(&self) {
        let context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
        let dist: f64 = self.wall.brick_width as f64 + 1.0;
        let bricks = self.wall.bricks.clone().into_iter().map(|b| {
            if self.droper.coord.iter().any(|c| c.0 == b.0 as isize && c.1 == b.1 as isize) {
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
    let canvas = setup_canvas("canvas", &wall);
    let ani = Animate::new(canvas, wall);

    window().request_animation_frame(move |time| {
        ani.animate(time);
    });
}
