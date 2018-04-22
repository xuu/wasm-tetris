#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use std::cell::RefCell;
use std::rc::Rc;

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
    width: usize,
    height: usize,
    brick_width: u32,
    bricks: Vec<(i32, i32)>,
}

impl Wall {
    fn new(width: usize, height: usize, brick_width: u32) -> Wall {
        let mut bricks = Vec::with_capacity(width * height);
        for y in (height - 3)..height {
            for x in 0..width {
                if !((y == height - 1) && x == 4 || x == 6) {
                    bricks.push((x as i32, y as i32))
                }
            }
        }
        Wall {
            width,
            height,
            brick_width,
            bricks
            // bricks: Vec::with_capacity(width * height),
        }
    }

    fn check_brick_existing(&self, check_brick: (i32, i32)) -> bool {
        self.bricks.iter().any(|&b| b == check_brick)
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
    playing: bool,
    game_over: bool,
}

impl Store {
    fn new(wall: Wall) -> Store {
        let current_drop = gen_random_drop();
        let init_x = wall.width as i32 / 2 - 1;
        Store {
            wall,
            current_drop,
            current_drop_coords: get_drop_coords(current_drop, init_x),
            next_drop: gen_random_drop(),
            scores: 0,
            init_x,
            playing: false,
            game_over: false,
        }
    }
}

impl Store {
    fn will_crash(&self, new_drop_coords: DropCoords) -> bool {
        new_drop_coords
            .iter()
            .any(|c| c.0 < 0 || c.0 > self.wall.width as i32 - 1 || c.1 >= self.wall.height as i32)
            || new_drop_coords
                .iter()
                .any(|d| self.wall.check_brick_existing(*d))
    }

    fn build_wall(&mut self) {
        let mut game_over = false;
        let width = self.wall.width as i32;
        self.wall.bricks.extend(self.current_drop_coords.iter());
        self.wall
            .bricks
            .sort_by(|a, b| (a.1 * width + a.0).cmp(&(b.1 * width + b.0)));
        let (mut new_bricks, rows, mut temp, _) = self.wall.bricks.iter().fold(
            (
                Vec::<(i32, i32)>::new(),
                Vec::<i32>::new(),
                Vec::<(i32, i32)>::new(),
                0,
            ),
            |(mut n, mut rows, mut temp, prev_y), &(x, y)| {
                if y < 0 {
                    game_over = true;
                }
                if y == prev_y || temp.len() == 0 {
                    temp.push((x, y));
                    if temp.len() == self.wall.width {
                        rows.push(y);
                        temp.clear();
                    }
                } else {
                    n.extend(temp.iter());
                    temp.clear();
                    temp.push((x, y));
                }
                (n, rows, temp, y)
            },
        );

        if game_over {
            self.game_over = true;
            return;
        }

        if temp.len() > 0 {
            new_bricks.append(&mut temp);
        }

        if rows.len() > 0 {
            self.wall.bricks = new_bricks
                .iter()
                .map(|&(x, y)| {
                    let dy = rows.iter()
                        .fold(0, |count, &r| if y < r { count + 1 } else { count });
                    (x, y + dy)
                })
                .collect();
            self.scores += 100 * 2u32.pow(rows.len() as u32 - 1);
        }
    }

    fn move_down(&mut self) {
        if !self.playing {
            return;
        }
        let mut new_drop_coords = self.current_drop_coords.clone();
        for (_, y) in new_drop_coords.iter_mut() {
            *y += 1;
        }
        if self.will_crash(new_drop_coords) {
            self.build_wall();
            self.current_drop = self.next_drop;
            self.current_drop_coords = get_drop_coords(self.current_drop, self.init_x);
            self.next_drop = gen_random_drop();
        } else {
            self.current_drop_coords = new_drop_coords;
        }
    }

    fn move_left(&mut self) {
        if !self.playing {
            return;
        }
        let mut new_drop_coords = self.current_drop_coords.clone();
        for (x, _) in new_drop_coords.iter_mut() {
            *x -= 1;
        }
        if !self.will_crash(new_drop_coords) {
            self.current_drop_coords = new_drop_coords;
        }
    }

    fn move_right(&mut self) {
        if !self.playing {
            return;
        }
        let mut new_drop_coords = self.current_drop_coords.clone();
        for (x, _) in new_drop_coords.iter_mut() {
            *x += 1;
        }
        if !self.will_crash(new_drop_coords) {
            self.current_drop_coords = new_drop_coords;
        }
    }

    fn rotate(&mut self) {
        if self.current_drop == O || !self.playing {
            return;
        }
        let mut next_coords = self.current_drop_coords.clone();
        // default rotate origin
        let (mut x0, y0) = next_coords[1];
        // adjust origin horizontally
        let adjust_dir = (
            x0 > 1 && !self.wall.check_brick_existing((x0 - 2, y0)),
            x0 > 0 && !self.wall.check_brick_existing((x0 - 1, y0)),
            x0 < (self.wall.width - 1) as i32 && !self.wall.check_brick_existing((x0 + 1, y0)),
            x0 < (self.wall.width - 2) as i32 && !self.wall.check_brick_existing((x0 + 2, y0)),
        );
        let dx = match adjust_dir {
            (false, true, true, true) if self.current_drop == I => 1,
            (_, false, true, true) if self.current_drop == I => 2,
            (true, true, false, _) if self.current_drop == I => -2,
            (_, true, true, false) if self.current_drop == I => -1,
            (_, false, true, _) if self.current_drop != I => 1,
            (_, true, false, _) if self.current_drop != I => -1,
            (_, false, false, _) => return,
            _ => 0,
        };
        x0 += dx;
        for c in next_coords.iter_mut() {
            c.0 += dx;
            *c = (x0 + y0 - c.1, y0 + c.0 - x0);
        }
        if !self.will_crash(next_coords) {
            self.current_drop_coords = next_coords;
        }
    }

    fn get_bricks_snapshot(&self) -> Vec<(i32, i32)> {
        let mut bricks = self.wall.bricks.clone();
        bricks.extend(self.current_drop_coords.iter());
        bricks
    }

    fn playing_toggle(&mut self) {
        self.playing = !self.playing;
    }
}

#[derive(Debug)]
struct Canvas {
    canvas: CanvasElement,
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

        let Wall {
            width,
            height,
            brick_width,
            ..
        } = store.wall;

        // let dist = brick_width as f64 + 1.0;
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let translate_y = 4f64 * (brick_width + 1) as f64;
        canvas.set_width(width as u32 * (brick_width + 1));
        canvas.set_height((height + 4) as u32 * (brick_width + 1));
        context.translate(0f64, translate_y);
        context.set_font("12px serif");
        // context.set_fill_style_color("#eee");
        // for y in 0..height {
        //     for x in 0..width {
        //         context.fill_rect(
        //             x as f64 * dist,
        //             y as f64 * dist,
        //             brick_width as f64,
        //             brick_width as f64,
        //         );
        //     }
        // }

        Canvas {
            canvas,
            context,
            store,
        }
    }

    fn draw(&mut self) {
        let brick_width = self.store.wall.brick_width as f64;
        let dist = brick_width + 1.0;
        let translate_y = 4f64 * (brick_width + 1.0);
        let scores = String::from("scores: ") + &self.store.scores.to_string();
        self.context.clear_rect(
            0.0,
            -translate_y,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        self.context.set_fill_style_color("#333");
        self.context
            .fill_text(&scores, 0.0, -translate_y + 10.0, None);
        self.context.set_fill_style_color("#eee"); // TODO: background
        for y in 0..self.store.wall.height {
            for x in 0..self.store.wall.width {
                self.context.fill_rect(
                    x as f64 * dist,
                    y as f64 * dist,
                    brick_width as f64,
                    brick_width as f64,
                );
            }
        }

        self.context.set_fill_style_color("#333");
        for &(x, y) in get_drop_coords(self.store.next_drop, self.store.init_x).iter() {
            self.context
                .fill_rect(x as f64 * dist, y as f64 * dist, brick_width, brick_width);
        }

        for (x, y) in self.store.get_bricks_snapshot() {
            if y >= 0 {
                self.context
                    .fill_rect(x as f64 * dist, y as f64 * dist, brick_width, brick_width);
            }
        }
    }
}

struct Animation {
    canvas: Rc<RefCell<Canvas>>,
    time_stamp: f64,
}

impl Animation {
    fn new(canvas: Canvas) {
        let canvas_rc = Rc::new(RefCell::new(canvas));
        let animation = Animation {
            canvas: canvas_rc.clone(),
            time_stamp: 0.0,
        };

        let canvas_for_action = canvas_rc.clone();
        window().add_event_listener(move |e: KeyDownEvent| {
            // js! {
            //     console.log(@{format!("{:?}", e.key())})
            // }
            let mut c = canvas_for_action.borrow_mut();
            match e.key().as_str() {
                "ArrowUp" | "w" => c.store.rotate(),
                "ArrowRight" | "d" => c.store.move_right(),
                "ArrowLeft" | "a" => c.store.move_left(),
                "ArrowDown" | "s" => c.store.move_down(),
                " " => {
                    c.store.playing_toggle();
                    return;
                }
                &_ => (),
            }
            if !c.store.playing {
                c.store.playing_toggle()
            }
            c.draw();
        });

        animation.play(400.0);
    }

    fn play(mut self, time: f64) {
        if time - self.time_stamp > 300.0 {
            self.time_stamp = time;
            let mut c = self.canvas.borrow_mut();
            if c.store.playing {
                c.store.move_down();
                c.draw();
            }
        }
        if !self.canvas.borrow().store.game_over {
            window().request_animation_frame(|t| {
                self.play(t);
            });
        }
    }
}

fn main() {
    let wall = Wall::new(10, 20, 10);
    let store = Store::new(wall);
    let canvas = Canvas::new("canvas", store);
    Animation::new(canvas);
}
