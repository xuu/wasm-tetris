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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Brick {
    Empty,
    Full,
}
use self::Brick::*;

struct Wall {
    rows: usize,
    cols: usize,
    brick_width: u32,
    bricks: Vec<Vec<Brick>>,
    y0: usize,
}

impl Wall {
    fn new(rows: usize, cols: usize, brick_width: u32) -> Wall {
        let y0 = 4;
        let bricks = vec![vec![Empty; cols]; rows + y0];

        Wall {
            rows: rows + y0,
            cols,
            brick_width,
            bricks,
            y0,
        }
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        self.bricks[row][col] == Full
    }
}

type DropCoords = [(usize, usize); 4]; // (x, y) -> (col, row)

impl Wall {
    fn get_drop_coords(&self, d: BrickDrop) -> DropCoords {
        let x0 = self.cols / 2 - 1;

        match d {
            I => [(x0, 0), (x0, 1), (x0, 2), (x0, 3)],
            J => [(x0 + 1, 1), (x0 + 1, 2), (x0 + 1, 3), (x0, 3)],
            L => [(x0, 1), (x0, 2), (x0, 3), (x0 + 1, 3)],
            O => [(x0, 2), (x0 + 1, 2), (x0, 3), (x0 + 1, 3)],
            S => [(x0 + 2, 2), (x0 + 1, 2), (x0 + 1, 3), (x0, 3)],
            T => [(x0, 2), (x0 + 1, 2), (x0 + 2, 2), (x0 + 1, 3)],
            Z => [(x0, 2), (x0 + 1, 2), (x0 + 1, 3), (x0 + 2, 3)],
        }
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
    speed: f64,
    playing: bool,
    game_over: bool,
}

impl Store {
    fn new(rows: usize, cols: usize, brick_width: u32) -> Store {
        let wall = Wall::new(rows, cols, brick_width);
        let current_drop = gen_random_drop();
        let current_drop_coords = wall.get_drop_coords(current_drop);

        Store {
            wall,
            current_drop,
            current_drop_coords,
            next_drop: gen_random_drop(),
            score: 0,
            level: 1,
            speed: 300.0,
            playing: false,
            game_over: false,
        }
    }
}

impl Store {
    fn will_crash(&self, new_drop_coords: DropCoords) -> bool {
        new_drop_coords.iter().any(|c| c.1 >= (self.wall.rows))
            || new_drop_coords.iter().any(|&d| self.wall.is_full(d.1, d.0))
    }
}

impl Store {
    fn merge(&mut self) {
        for (x, y) in &self.current_drop_coords {
            if y < &self.wall.y0 {
                self.game_over = true;
            }
            self.wall.bricks[*y][*x] = Full;
        }

        if self.game_over {
            return;
        }

        self.wall
            .bricks
            .retain(|row| row.iter().any(|&b| b == Empty));

        let len = self.wall.bricks.len();
        if len < self.wall.rows {
            let rows_count = self.wall.rows - len;
            let mut new_rows = vec![vec![Empty; self.wall.cols]; rows_count];
            new_rows.append(&mut self.wall.bricks);
            self.wall.bricks = new_rows;

            self.score += 100 * 2u32.pow(rows_count as u32 - 1);
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
            self.current_drop_coords = self.wall.get_drop_coords(self.current_drop);
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
            if x < &mut 1 {
                return;
            }
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
            if x > &mut (self.wall.cols - 2) {
                return;
            }
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
        let mut new_coords = self.current_drop_coords.clone();
        // default rotate origin
        let (mut x0, y0) = new_coords[1];
        // adjust origin horizontally
        let adjust_dir = (
            x0 > 1 && !self.wall.is_full(y0, x0 - 2),
            x0 > 0 && !self.wall.is_full(y0, x0 - 1),
            x0 < (self.wall.cols - 1) && !self.wall.is_full(y0, x0 + 1),
            x0 < (self.wall.cols - 2) && !self.wall.is_full(y0, x0 + 2),
        );
        let dx: i64 = match adjust_dir {
            (false, true, true, true) if self.current_drop == I => 1,
            (_, false, true, true) if self.current_drop == I => 2,
            (true, true, false, _) if self.current_drop == I => -2,
            (_, true, true, false) if self.current_drop == I => -1,
            (_, false, true, _) if self.current_drop != I => 1,
            (_, true, false, _) if self.current_drop != I => -1,
            (_, false, false, _) => return,
            _ => 0,
        };
        let dx = dx as usize;
        x0 += dx;
        // https://en.wikipedia.org/wiki/Rotation_of_axes
        // rotate 90 degree
        for c in new_coords.iter_mut() {
            c.0 += dx;
            *c = (x0 + y0 - c.1, y0 + c.0 - x0);
        }
        if !self.will_crash(new_coords) {
            self.current_drop_coords = new_coords;
        }
    }
}

impl Store {
    fn restart(&mut self) {
        self.wall.bricks.clear();
        self.current_drop = gen_random_drop();
        self.next_drop = gen_random_drop();
        self.current_drop_coords = self.wall.get_drop_coords(self.current_drop);
        self.score = 0;
        self.game_over = false;
        self.speed = 300.0
    }

    fn pause_toggle(&mut self) {
        self.playing = !self.playing;
    }
}

impl Store {
    fn get_frame(&self) -> Vec<Vec<Brick>> {
        let mut bricks = self.wall.bricks.clone();
        for (x, y) in &self.current_drop_coords {
            if y >= &self.wall.y0 {
                bricks[*y][*x] = Full;
            }
        }
        for (x, y) in &self.wall.get_drop_coords(self.next_drop) {
            bricks[*y][*x] = Full;
        }
        bricks
    }
}

struct Tetris {
    store: Store,
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    header_height: f64,
    color_light: JsValue,
    color_dark: JsValue,
}

// TODO: rows >= 10. cols >= 10?
impl Tetris {
    fn new(
        canvas_element: HtmlCanvasElement,
        rows: usize,
        cols: usize,
        brick_width: u32,
    ) -> Rc<RefCell<Tetris>> {
        let store = Store::new(rows, cols, brick_width);
        let header_height = 60f64;
        let width = cols as u32 * (brick_width + 1);
        let height = header_height as u32 + store.wall.bricks.len() as u32 * (brick_width + 1);
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
        context.set_text_align("center");
        context.set_fill_style(&color_light);

        let tetris = Tetris {
            store,
            context,
            width,
            height,
            header_height,
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
        let x_center = self.width as f64 / 2.0;
        let score_level = String::from("Score/Level: ")
            + &self.store.score.to_string()
            + "/"
            + &self.store.level.to_string();

        self.context
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);

        self.context.set_fill_style(&self.color_dark);
        self.context.set_font("12px sans-serif");
        self.context
            .fill_text(&score_level, x_center, 10.0)
            .unwrap();

        if self.store.game_over {
            self.context.set_font("14px sans-serif");
            self.context
                .fill_text("Game Over!", x_center, 30.0)
                .unwrap();
            self.context
                .fill_text("Press `enter` restart.", x_center, 50.0)
                .unwrap();
        }

        let Wall {
            brick_width,
            rows,
            cols,
            y0,
            ..
        } = self.store.wall;
        let frame = self.store.get_frame();
        let dist = brick_width as f64 + 1.0;
        for row in 0..rows {
            for col in 0..cols {
                self.context.set_fill_style(if frame[row][col] == Full {
                    &self.color_dark
                } else {
                    &self.color_light
                });
                if (frame[row][col] == Full) || (row >= y0) {
                    self.context.fill_rect(
                        col as f64 * dist,
                        self.header_height + row as f64 * dist,
                        brick_width as f64,
                        brick_width as f64,
                    );
                }
            }
        }
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

// https://github.com/rustwasm/wasm-bindgen/blob/3d2f548ce2/examples/request-animation-frame/src/lib.rs
fn setup_animatoin_frame(tetris: Rc<RefCell<Tetris>>) {
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
