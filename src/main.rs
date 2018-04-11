#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Wall {
    width: u32,
    height: u32,
    brick_width: u32,
    bricks: Vec<(u32, u32, Brick)>,
}

impl Wall {
    fn new(width: u32, height: u32, brick_width: u32) -> Wall {
        let mut bricks: Vec<(u32, u32, Brick)> = Vec::new();
        for i in 0..width {
            for j in 0..height {
                bricks.push((i, j, Gray))
            }
        }
        Wall {
            width,
            height,
            brick_width,
            bricks,
        }
    }
}

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

type DropCoords = [(i32, i32); 4];

fn get_drop_coords(d: BrickDrop, init_x: i32) -> DropCoords {
    match d {
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
    }
}

#[derive(Debug)]
struct Store {
    wall: Wall,
    current_drop: BrickDrop,
    next_drop: BrickDrop,
    current_drop_coords: DropCoords,
    scores: u32,
}

impl Store {
    fn new(wall: Wall) -> Store {
        let wall_width = wall.width;
        Store {
            wall,
            current_drop: O,
            next_drop: L,
            current_drop_coords: get_drop_coords(L, wall_width as i32 / 2 - 1),
            scores: 0,
        }
    }

    // fn get_random_brick_drop() {

    // }

    fn move_down(&mut self) {
        for (_, y) in self.current_drop_coords.iter_mut() {
            *y += 1;
            // if *y == self.max_y - 1 {
            //     self.end = true;
            // }
        }
    }

    fn move_left(&mut self) {
        if self.current_drop_coords.iter().any(|c| c.0 < 1) {
            return;
        }
        for (x, _) in self.current_drop_coords.iter_mut() {
            *x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.current_drop_coords
            .iter()
            .any(|c| c.0 > self.wall.width as i32 - 2)
        {
            return;
        }
        for (x, _) in self.current_drop_coords.iter_mut() {
            *x += 1;
        }
    }
}

struct Animation {
    time_stamp: f64,
}

impl Animation {
    fn new() -> Animation {
        Animation { time_stamp: 0.0 }
    }

    fn play(mut self, canvas: Rc<RefCell<Canvas>>, time: f64) {
        if time - self.time_stamp > 300.0 {
            self.time_stamp = time;
            let c = canvas.clone();
            let mut cc = c.borrow_mut();
            cc.store.move_down();
            cc.paint();
        }

        window().request_animation_frame(|t| {
            self.play(canvas, t);
        });
    }
}

#[derive(Debug)]
struct Canvas {
    context: CanvasRenderingContext2d,
    store: Store,
}

impl Canvas {
    fn new(selector: &str, store: Store) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(selector)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        canvas.set_width(store.wall.width as u32 * (store.wall.brick_width + 1));
        canvas.set_height(store.wall.height as u32 * (store.wall.brick_width + 1));

        Canvas {
            context: canvas.get_context().unwrap(),
            store,
        }
    }

    fn paint(&self) {
        let dist: f64 = self.store.wall.brick_width as f64 + 1.0;
        let bricks = self.store.wall.bricks.clone().into_iter().map(|b| {
            if self.store
                .current_drop_coords
                .iter()
                .any(|c| c.0 == b.0 as i32 && c.1 == b.1 as i32)
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
            self.context.set_fill_style_color(color);
            self.context.fill_rect(
                x as f64 * dist,
                y as f64 * dist,
                self.store.wall.brick_width as f64,
                self.store.wall.brick_width as f64,
            );
        }
    }
}

fn setup_action(canvas: Rc<RefCell<Canvas>>) {
    window().add_event_listener(move |e: KeyDownEvent| {
        js! {
            console.log(@{format!("{:?}", e.key())})
        }
        let mut c = canvas.borrow_mut();
        match e.key().as_str() {
            "ArrowRight" => c.store.move_right(),
            "ArrowLeft" => c.store.move_left(),
            &_ => (),
        }
        c.paint();
    });
}

fn main() {
    let wall = Wall::new(30, 50, 10);
    let store = Store::new(wall);
    let canvas = Rc::new(RefCell::new(Canvas::new("canvas", store)));
    let canvas_animate = canvas.clone();
    let canvas_action = canvas.clone();
    let animate = Animation::new();

    setup_action(canvas_action);
    window().request_animation_frame(move |time| {
        animate.play(canvas_animate, time);
    });
}
