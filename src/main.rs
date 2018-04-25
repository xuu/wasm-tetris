#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::TextAlign::*;
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

struct Wall {
    width: usize,
    height: usize,
    brick_width: u32,
    bricks: Vec<(i32, i32)>,
}

impl Wall {
    fn new(width: usize, height: usize, brick_width: u32) -> Wall {
        Wall {
            width,
            height,
            brick_width,
            bricks: Vec::with_capacity(width * height),
        }
    }

    fn check_brick_existing(&self, check_brick: (i32, i32)) -> bool {
        self.bricks.iter().any(|&b| b == check_brick)
    }
}

fn derived_level(score: u32) -> u32 {
    match score {
        0...5_000 => 1,
        5_000...8_000 => 2,
        8_000...10_000 => 3,
        _ => 4,
    }
}

fn derived_speed(level: u32) -> f64 {
    match level {
        0 | 1 => 300.0,
        2 => 200.0,
        3 => 100.0,
        _ => 50.0,
    }
}

struct Store {
    wall: Wall,
    current_drop: BrickDrop,
    current_drop_coords: DropCoords,
    next_drop: BrickDrop,
    score: u32,
    level: u32,
    init_x: i32,
    speed: f64,
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
            score: 0,
            level: 1,
            init_x,
            speed: 300.0,
            playing: false,
            game_over: false,
        }
    }
}

impl Store {
    fn will_crash(&self, new_drop_coords: DropCoords) -> bool {
        new_drop_coords
            .iter()
            .any(|c| c.0 < 0 || c.0 >= self.wall.width as i32 || c.1 >= self.wall.height as i32)
            || new_drop_coords
                .iter()
                .any(|&d| self.wall.check_brick_existing(d))
    }
}

impl Store {
    fn build_wall(&mut self) {
        let width = self.wall.width as i32;
        self.wall.bricks.extend(self.current_drop_coords.iter());
        self.wall
            .bricks
            .sort_by(|a, b| (a.1 * width + a.0).cmp(&(b.1 * width + b.0)));
        let (mut new_bricks, mut temp, rows, _, game_over) = self.wall.bricks.iter().fold(
            (
                Vec::<(i32, i32)>::new(),
                Vec::<(i32, i32)>::new(),
                Vec::<i32>::new(),
                0,
                false,
            ),
            |(mut n, mut temp, mut rows, prev_y, _), &(x, y)| {
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
                (n, temp, rows, y, y < 0)
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
            self.score += 100 * 2u32.pow(rows.len() as u32 - 1);
            self.level = derived_level(self.score);
            self.speed = derived_speed(self.level);
        }
    }
}

impl Store {
    fn move_down(&mut self) -> bool {
        if !self.playing {
            return false;
        }
        let mut new_drop_coords = self.current_drop_coords.clone();
        for (_, y) in new_drop_coords.iter_mut() {
            *y += 1;
        }
        let crash = self.will_crash(new_drop_coords);
        if crash {
            self.build_wall();
            self.current_drop = self.next_drop;
            self.current_drop_coords = get_drop_coords(self.current_drop, self.init_x);
            self.next_drop = gen_random_drop();
        } else {
            self.current_drop_coords = new_drop_coords;
        }
        !crash
    }
}

impl Store {
    fn drop_down(&mut self) {
        loop {
            if !self.move_down() {
                break;
            }
        }
    }
}

impl Store {
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
}

impl Store {
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
}

impl Store {
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
        // https://en.wikipedia.org/wiki/Rotation_of_axes
        // rotate 90 degree
        for c in next_coords.iter_mut() {
            c.0 += dx;
            *c = (x0 + y0 - c.1, y0 + c.0 - x0);
        }
        if !self.will_crash(next_coords) {
            self.current_drop_coords = next_coords;
        }
    }
}

impl Store {
    fn restart(&mut self) {
        self.wall.bricks.clear();
        self.current_drop = gen_random_drop();
        self.next_drop = gen_random_drop();
        self.current_drop_coords = get_drop_coords(self.current_drop, self.init_x);
        self.score = 0;
        self.game_over = false;
        self.speed = 300.0
    }

    fn pause_toggle(&mut self) {
        self.playing = !self.playing;
    }
}

impl Store {
    fn get_bricks_snapshot(&self) -> Vec<(i32, i32)> {
        let mut bricks = self.wall.bricks.clone();
        bricks.extend(self.current_drop_coords.iter());
        bricks
    }
}

struct Canvas {
    canvas: CanvasElement,
    context: CanvasRenderingContext2d,
    store: Store,
    top_y: f64,
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
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let translate_y = 5f64 * (brick_width + 1) as f64;
        let dist = brick_width as f64 + 1.0;

        canvas.set_width(width as u32 * (brick_width + 1));
        canvas.set_height((height + 5) as u32 * (brick_width + 1));
        context.translate(0f64, translate_y);
        context.set_fill_style_color("#eee");
        for y in 0..height {
            for x in 0..width {
                context.fill_rect(
                    x as f64 * dist,
                    y as f64 * dist,
                    brick_width as f64,
                    brick_width as f64,
                );
            }
        }

        let x_center = canvas.width() as f64 / 2.0;
        context.set_fill_style_color("#333");
        context.set_font("14px consolas,courier,monospace");
        context.set_text_align(Center);
        context.fill_text("start: any", x_center, 20.0, None);
        context.fill_text("left: ← , j , a", x_center, 40.0, None);
        context.fill_text("right: → , l , d", x_center, 60.0, None);
        context.fill_text("rotate: ↑ , i , w", x_center, 80.0, None);
        context.fill_text("speed up: ↓ , k , s", x_center, 100.0, None);
        context.fill_text("drop: enter , space", x_center, 120.0, None);
        context.fill_text("pause: p", x_center, 140.0, None);
        context.fill_text("restart: r", x_center, 160.0, None);

        Canvas {
            canvas,
            context,
            store,
            top_y: -translate_y,
        }
    }
}

impl Canvas {
    fn draw(&mut self) {
        let brick_width = self.store.wall.brick_width as f64;
        let dist = brick_width + 1.0;
        let x_center = self.canvas.width() as f64 / 2.0;
        let score_level = String::from("Score/Level: ") + &self.store.score.to_string() + "/"
            + &self.store.level.to_string();

        self.context.clear_rect(
            0.0,
            self.top_y,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        self.context.set_fill_style_color("#eee");
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
        self.context.set_font("12px sans-serif");
        self.context
            .fill_text(&score_level, x_center, self.top_y + 10.0, None);

        if self.store.game_over {
            self.context.set_font("14px sans-serif");
            self.context.fill_text(
                "Game Over! Press `enter` restart.",
                x_center,
                self.top_y + 30.0,
                None,
            );
        } else {
            for &(x, y) in get_drop_coords(self.store.next_drop, self.store.init_x).iter() {
                self.context
                    .fill_rect(x as f64 * dist, y as f64 * dist, brick_width, brick_width);
            }
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
            let mut c = canvas_for_action.borrow_mut();
            match e.key().as_str() {
                "ArrowUp" | "w" | "i" => c.store.rotate(),
                "ArrowRight" | "d" | "l" => c.store.move_right(),
                "ArrowLeft" | "a" | "j" => c.store.move_left(),
                "ArrowDown" | "s" | "k" => {
                    c.store.move_down();
                }
                "p" => {
                    c.store.pause_toggle();
                    return;
                }
                "r" => c.store.restart(),
                " " => {
                    e.prevent_default();
                    c.store.drop_down();
                }
                "Enter" => if c.store.game_over {
                    c.store.restart();
                } else {
                    c.store.drop_down();
                },
                &_ => (),
            }
            if !c.store.playing {
                c.store.pause_toggle()
            }
            c.draw();
        });

        animation.play(400.0);
    }
}

impl Animation {
    fn play(mut self, time: f64) {
        if time - self.time_stamp > self.canvas.borrow().store.speed {
            self.time_stamp = time;
            let mut c = self.canvas.borrow_mut();
            if c.store.playing && !c.store.game_over {
                c.store.move_down();
                c.draw();
            }
        }

        window().request_animation_frame(|t| {
            self.play(t);
        });
    }
}

struct Tetris {}

impl Tetris {
    fn new(wall_width: usize, wall_height: usize, brick_width: u32) {
        let wall = Wall::new(wall_width, wall_height, brick_width);
        let store = Store::new(wall);
        let canvas = Canvas::new("canvas", store);
        Animation::new(canvas);
    }
}

fn main() {
    Tetris::new(20, 30, 10);
}
