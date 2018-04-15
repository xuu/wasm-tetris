#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
// use stdweb::Value;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
enum Brick {
    Black,
    Gray,
}

use Brick::*;

#[derive(Debug, Copy, Clone, PartialEq)]
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

fn gen_random_drop() -> BrickDrop {
    let brick_drops = [I, J, L, O, S, T, Z];
    let r = js!(return Math.floor(Math.random() * 7));
    let rr: usize = r.try_into().unwrap();
    // js! {
    //     console.log(@{format!("{:?}", rr)});
    // }
    brick_drops[rr]
}

type DropCoords = [(i32, i32); 4];

fn get_drop_coords(d: BrickDrop, init_x: i32) -> DropCoords {
    match d {
        I => [(init_x, -4), (init_x, -3), (init_x, -2), (init_x, -1)],
        J => [
            (init_x + 1, -3),
            (init_x + 1, -2),
            (init_x + 1, -1),
            (init_x, -1),
        ],
        L => [(init_x, -3), (init_x, -2), (init_x, -1), (init_x + 1, -1)],
        O => [
            (init_x, -2),
            (init_x + 1, -2),
            (init_x, -1),
            (init_x + 1, -1),
        ],
        S => [
            (init_x + 2, -2),
            (init_x + 1, -2),
            (init_x + 1, -1),
            (init_x, -1),
        ],
        T => [
            (init_x, -2),
            (init_x + 1, -2),
            (init_x + 2, -2),
            (init_x + 1, -1),
        ],
        Z => [
            (init_x, -2),
            (init_x + 1, -2),
            (init_x + 1, -1),
            (init_x + 2, -1),
        ],
    }
}

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
        for y in 0..height {
            for x in 0..width {
                bricks.push((x, y, Gray))
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

#[derive(Debug)]
struct Store {
    wall: Wall,
    current_drop: BrickDrop,
    current_drop_coords: DropCoords,
    next_drop: BrickDrop,
    scores: u32,
    init_x: i32,
}

impl Store {
    fn new(wall: Wall) -> Store {
        let current_drop = gen_random_drop();
        // let current_drop = I;
        let init_x = wall.width as i32 / 2 - 1;
        // js! {
        //     console.log(@{format!("{:?}", get_drop_coords(current_drop, init_x))})
        // }
        Store {
            wall,
            current_drop,
            current_drop_coords: get_drop_coords(current_drop, init_x),
            next_drop: gen_random_drop(),
            scores: 0,
            init_x,
        }
    }
}

impl Store {
    fn will_crash(&self, next_drop_coords: DropCoords) -> bool {
        next_drop_coords
            .iter()
            .any(|c| c.0 < 0 || c.0 > self.wall.width as i32 - 1 || c.1 >= self.wall.height as i32) ||
        next_drop_coords.iter().any(|c| {
            let index = c.1 * self.wall.width as i32 + c.0;
            let len = self.wall.bricks.len() as i32;
            index >= 0 && index < len && self.wall.bricks[index as usize].2 == Black
        })
    }

    fn move_down(&mut self) {
        let width = self.wall.width;
        let mut next_drop_coords = self.current_drop_coords.clone();
        for (_, y) in next_drop_coords.iter_mut() {
            *y += 1;
        }
        // js! {
        //     console.log(@{format!("{:?}", self.will_crash(next_drop_coords))});
        // }
        if self.will_crash(next_drop_coords) {
            js! {
                console.log("crash");
            }
            for (x, y) in self.current_drop_coords.iter() {
                let index = *x + *y * width as i32;
                if index >= 0 && index < self.wall.bricks.len() as i32 {
                    self.wall.bricks[index as usize].2 = Black
                }
            }
            self.current_drop = self.next_drop;
            self.current_drop_coords = get_drop_coords(self.current_drop, self.init_x);
            self.next_drop = gen_random_drop();
        } else {
            self.current_drop_coords = next_drop_coords;
        }
    }

    fn move_left(&mut self) {
        let mut next_drop_coords = self.current_drop_coords.clone();
        for (x, _) in next_drop_coords.iter_mut() {
            *x -= 1;
        }
        if !self.will_crash(next_drop_coords) {
            self.current_drop_coords = next_drop_coords;
        }
    }

    fn move_right(&mut self) {
        let mut next_drop_coords = self.current_drop_coords.clone();
        for (x, _) in next_drop_coords.iter_mut() {
            *x += 1;
        }
        if !self.will_crash(next_drop_coords) {
            self.current_drop_coords = next_drop_coords;
        }
    }

    fn rotate(&mut self) {
        if self.current_drop == O {
            return;
        }
        let mut next_coords = self.current_drop_coords.clone();
        // default rotate origin
        let (mut x0, y0) = next_coords[1];
        // adjust origin horizontally
        let index = y0 * self.wall.width as i32 + x0;
        let adjust_dir = (
            y0 >= 0 && x0 > 1 && self.wall.bricks[(index - 2) as usize].2 == Gray,
            y0 >= 0 && x0 > 0 && self.wall.bricks[(index - 1) as usize].2 == Gray,
            y0 >= 0 && x0 < (self.wall.width - 1) as i32
                && self.wall.bricks[(index + 1) as usize].2 == Gray,
            y0 >= 0 && x0 < (self.wall.width - 1) as i32
                && self.wall.bricks[(index + 2) as usize].2 == Gray,
        );
        if let (_, false, false, _) = adjust_dir {
            return;
        }
        let dx = match adjust_dir {
            (false, true, true, _) if self.current_drop == I => 1,
            (false, false, true, true) if self.current_drop == I => 2,
            (true, true, false, _) if self.current_drop == I => -2,
            (_, true, true, false) if self.current_drop == I => -1,
            (_, false, true, _) if self.current_drop != I => 1,
            (_, true, false, _) if self.current_drop != I => -1,
            _ => 0,
        };
        x0 += dx;
        for c in next_coords.iter_mut() {
            c.0 += dx; // drop coords also adjusted
            *c = (x0 + y0 - c.1, y0 + c.0 - x0);
        }
        if !self.will_crash(next_coords) {
            self.current_drop_coords = next_coords;
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

        // window().request_animation_frame(|t| {
        //     self.play(canvas, t);
        // });
    }

    // fn pause() {}

    // fn resume() {}
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
        let brick_width = self.store.wall.brick_width as f64;
        let dist = brick_width + 1.0;
        let mut bricks = self.store.wall.bricks.clone();
        // .into_iter().map(|b| {
        //     if self.store
        //         .current_drop_coords
        //         .iter()
        //         .any(|c| c.0 == b.0 as i32 && c.1 == b.1 as i32)
        //     {
        //         (b.0, b.1, Black)
        //     } else {
        //         b
        //     }
        // });
        let width = self.store.wall.width;
        for (x, y) in self.store.current_drop_coords.iter() {
            let index = *x + *y * width as i32;
            if index >= 0 && index < bricks.len() as i32 {
                bricks[index as usize].2 = Black
            }
        }

        for (x, y, b) in bricks {
            let color = match b {
                Gray => "#eee",
                Black => "#333",
            };
            self.context.set_fill_style_color(color);
            self.context
                .fill_rect(x as f64 * dist, y as f64 * dist, brick_width, brick_width);
        }
    }
}

fn setup_action(canvas: Rc<RefCell<Canvas>>) {
    window().add_event_listener(move |e: KeyDownEvent| {
        // js! {
        //     console.log(@{format!("{:?}", e.key())})
        // }
        let mut c = canvas.borrow_mut();
        match e.key().as_str() {
            "ArrowUp" => c.store.rotate(),
            "ArrowRight" => c.store.move_right(),
            "ArrowLeft" => c.store.move_left(),
            "ArrowDown" => c.store.move_down(),
            &_ => (),
        }
        c.paint();
    });
}

fn main() {
    let wall = Wall::new(20, 30, 10);
    let store = Store::new(wall);
    let canvas = Rc::new(RefCell::new(Canvas::new("canvas", store)));
    let canvas_animation = canvas.clone();
    let canvas_action = canvas.clone();
    let animation = Animation::new();

    setup_action(canvas_action);
    window().request_animation_frame(move |time| {
        animation.play(canvas_animation, time);
    });
}
