extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use js_sys::Math;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

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

use self::BrickDrop::*;

fn gen_random_drop() -> BrickDrop {
    let brick_drops = [I, J, L, O, S, T, Z];
    let r = Math::floor(Math::random() * 7.0) as usize;
    brick_drops[r]
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
    rows: usize,
    cols: usize,
    brick_width: u32,
    bricks: Vec<(i32, i32)>,
}

impl Wall {
    fn new(rows: usize, cols: usize, brick_width: u32) -> Wall {
        Wall {
            rows,
            cols,
            brick_width,
            bricks: Vec::with_capacity(cols * rows),
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
    fn new(rows: usize, cols: usize, brick_width: u32) -> Store {
        let wall = Wall::new(rows, cols, brick_width);
        let current_drop = gen_random_drop();
        let init_x = wall.cols as i32 / 2 - 1;

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
            .any(|c| c.0 < 0 || c.0 >= self.wall.cols as i32 || c.1 >= self.wall.rows as i32)
            || new_drop_coords
                .iter()
                .any(|&d| self.wall.check_brick_existing(d))
    }
}

impl Store {
    fn merge(&mut self) {
        let cols = self.wall.cols as i32;
        self.wall.bricks.extend(self.current_drop_coords.iter());
        self.wall
            .bricks
            .sort_by(|a, b| (a.1 * cols + a.0).cmp(&(b.1 * cols + b.0)));
        let (mut new_bricks, mut temp, rows, game_over, _) = self.wall.bricks.iter().fold(
            (
                Vec::<(i32, i32)>::new(),
                Vec::<(i32, i32)>::new(),
                Vec::<i32>::new(),
                false,
                0,
            ),
            |(mut n, mut temp, mut rows, game_over, prev_y), &(x, y)| {
                if y == prev_y || temp.len() == 0 {
                    temp.push((x, y));
                    if temp.len() == self.wall.cols {
                        rows.push(y);
                        temp.clear();
                    }
                } else {
                    n.extend(temp.iter());
                    temp.clear();
                    temp.push((x, y));
                }
                (n, temp, rows, game_over || y < 0, y)
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
                    let dy = rows
                        .iter()
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
        if !self.playing || self.game_over {
            return false;
        }
        let mut new_drop_coords = self.current_drop_coords.clone();
        for (_, y) in new_drop_coords.iter_mut() {
            *y += 1;
        }
        let crash = self.will_crash(new_drop_coords);
        if crash {
            self.merge();
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
        if !self.playing || self.game_over {
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
        if !self.playing || self.game_over {
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
        if self.current_drop == O || !self.playing || self.game_over {
            return;
        }
        let mut next_coords = self.current_drop_coords.clone();
        // default rotate origin
        let (mut x0, y0) = next_coords[1];
        // adjust origin horizontally
        let adjust_dir = (
            x0 > 1 && !self.wall.check_brick_existing((x0 - 2, y0)),
            x0 > 0 && !self.wall.check_brick_existing((x0 - 1, y0)),
            x0 < (self.wall.cols - 1) as i32 && !self.wall.check_brick_existing((x0 + 1, y0)),
            x0 < (self.wall.cols - 2) as i32 && !self.wall.check_brick_existing((x0 + 2, y0)),
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
        bricks.extend(self.current_drop_coords.iter().filter(|c| c.1 >= 0));
        bricks
    }
}

macro_rules! brick_render {
    ($context:expr, $bricks:expr, $brick_width:expr) => {{
        let dist = $brick_width as f64 + 1.0;
        for &(x, y) in $bricks.iter() {
            $context.fill_rect(x as f64 * dist, y as f64 * dist, $brick_width, $brick_width);
        }
    }};
    ($context:expr, $cols:expr, $rows:expr, $brick_width:expr) => {{
        let dist = $brick_width as f64 + 1.0;
        for y in 0..$rows {
            for x in 0..$cols {
                $context.fill_rect(
                    x as f64 * dist,
                    y as f64 * dist,
                    $brick_width as f64,
                    $brick_width as f64,
                );
            }
        }
    }};
}

struct Tetris {
    store: Store,
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    y_translated: f64,
    color_light: JsValue,
    color_dark: JsValue,
}

impl Tetris {
    fn new(canvas_element: HtmlCanvasElement, rows: usize, cols: usize, brick_width: u32) -> Rc<RefCell<Tetris>> {
        let store = Store::new(rows, cols, brick_width);
        let y_translated = 8f64 * (brick_width + 1) as f64;
        let width = cols as u32 * (brick_width + 1);
        let height = (rows + 8) as u32 * (brick_width + 1);
        let color_light = JsValue::from_str("#eee");
        let color_dark = JsValue::from_str("#333");
        let context = canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        canvas_element.set_width(width);
        canvas_element.set_height(height);
        context.translate(0f64, y_translated).unwrap();
        context.set_text_align("center");
        context.set_fill_style(&color_light);
        brick_render!(context, cols, rows, brick_width);

        let tetris = Tetris {
            store,
            context,
            width,
            height,
            y_translated,
            color_light,
            color_dark,
        };
        let tetris = Rc::new(RefCell::new(tetris));

        setup_keyboard_event(tetris.clone());
        setup_animatoin_frame(tetris.clone());
        tetris
    }
}

impl Tetris {
    fn render(&mut self) {
        let y0 = -self.y_translated;
        let Wall {
            brick_width,
            rows,
            cols,
            ..
        } = self.store.wall;
        let x_center = self.width as f64 / 2.0;
        let score_level = String::from("Score/Level: ")
            + &self.store.score.to_string()
            + "/"
            + &self.store.level.to_string();

        self.context
            .clear_rect(0.0, y0, self.width as f64, self.height as f64);

        self.context.set_fill_style(&self.color_light);
        brick_render!(self.context, cols, rows, brick_width);

        self.context.set_fill_style(&self.color_dark);
        self.context.set_font("12px sans-serif");
        self.context
            .fill_text(&score_level, x_center, y0 + 10.0)
            .unwrap();

        if self.store.game_over {
            self.context.set_font("14px sans-serif");
            self.context
                .fill_text("Game Over!", x_center, y0 + 30.0)
                .unwrap();
            self.context
                .fill_text("Press `enter` restart.", x_center, y0 + 50.0)
                .unwrap();
        } else {
            brick_render!(
                self.context,
                get_drop_coords(self.store.next_drop, self.store.init_x),
                brick_width as f64
            );
        }

        brick_render!(
            self.context,
            self.store.get_bricks_snapshot(),
            brick_width as f64
        );
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("`window` not available")
}

fn setup_keyboard_event(tetris: Rc<RefCell<Tetris>>) {
    let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let mut t = tetris.borrow_mut();
        match e.key().as_str() {
            "ArrowUp" | "w" | "i" => t.store.rotate(),
            "ArrowRight" | "d" | "l" => t.store.move_right(),
            "ArrowLeft" | "a" | "j" => t.store.move_left(),
            "ArrowDown" | "s" | "k" => {
                t.store.move_down();
            }
            "p" => {
                t.store.pause_toggle();
                return;
            }
            "r" => t.store.restart(),
            " " => {
                // e.prevent_default();
                t.store.drop_down();
            }
            "Enter" => {
                if t.store.game_over {
                    t.store.restart();
                } else {
                    t.store.drop_down();
                }
            }
            &_ => (),
        }
        if !t.store.playing {
            t.store.pause_toggle()
        }
        t.render();
    }) as Box<FnMut(_)>);

    window()
        .add_event_listener_with_event_listener("keydown", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("`requestAnimationFrame` not available");
}

fn setup_animatoin_frame(tetris: Rc<RefCell<Tetris>>) {
    // https://github.com/rustwasm/wasm-bindgen/blob/3d2f548ce2/examples/request-animation-frame/src/lib.rs
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut time_stamp = 0.0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let time = window().performance().unwrap().now();
        if time - time_stamp > tetris.borrow().store.speed {
            time_stamp = time;
            let mut t = tetris.borrow_mut();
            if t.store.playing && !t.store.game_over {
                t.store.move_down();
                t.render();
            }
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[wasm_bindgen]
pub fn play(canvas_element: HtmlCanvasElement, rows: usize, cols: usize, brick_width: u32) {
    Tetris::new(canvas_element, rows, cols, brick_width);
}