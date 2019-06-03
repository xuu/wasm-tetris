extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use js_sys::Math;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Brick {
    Empty,
    Fill,
}

use self::Brick::*;

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

fn random_drop() -> BrickDrop {
    let r = Math::floor(Math::random() * 7.0) as usize;
    [I, J, L, O, S, T, Z][r]
}

struct Wall {
    rows: usize,
    cols: usize,
    brick_width: u32,
    bricks: Vec<Vec<Brick>>,
}

impl Wall {
    fn new(rows: usize, cols: usize, brick_width: u32) -> Wall {
        Wall {
            rows,
            cols,
            brick_width,
            bricks: vec![vec![Empty; cols]; rows],
        }
    }
}

// BrickDrop use Wall.bricks as an xy-Cartesian coordinate system
// (x, y) -> (col, row)
type DropCoords = [(i32, i32); 4];

impl Wall {
    fn drop_coords(&self, d: BrickDrop) -> DropCoords {
        let x0 = self.cols as i32 / 2 - 1;

        match d {
            I => [(x0, -4), (x0, -3), (x0, -2), (x0, -1)],
            J => [(x0 + 1, -3), (x0 + 1, -2), (x0 + 1, -1), (x0, -1)],
            L => [(x0, -3), (x0, -2), (x0, -1), (x0 + 1, -1)],
            O => [(x0, -2), (x0 + 1, -2), (x0, -1), (x0 + 1, -1)],
            S => [(x0 + 2, -2), (x0 + 1, -2), (x0 + 1, -1), (x0, -1)],
            T => [(x0, -2), (x0 + 1, -2), (x0 + 2, -2), (x0 + 1, -1)],
            Z => [(x0, -2), (x0 + 1, -2), (x0 + 1, -1), (x0 + 2, -1)],
        }
    }
}

fn derived_level(score: u32) -> u32 {
    match score {
        0...1_000 => 1,
        1_000...3_000 => 2,
        3_000...5_000 => 3,
        5_000...7_000 => 4,
        _ => 5,
    }
}

fn derived_speed(level: u32) -> f64 {
    match level {
        0 | 1 => 300.0,
        2 => 200.0,
        3 => 100.0,
        4 => 50.0,
        _ => 20.0,
    }
}

struct Store {
    wall: Wall,
    current_drop: BrickDrop,
    current_drop_coords: DropCoords,
    next_drop: BrickDrop,
    next_drop_coords: DropCoords,
    score: u32,
    level: u32,
    speed: f64,
    playing: bool,
    game_over: bool,
}

impl Store {
    fn new(rows: usize, cols: usize, brick_width: u32) -> Store {
        let wall = Wall::new(rows, cols, brick_width);
        let current_drop = random_drop();
        let next_drop = random_drop();
        let current_drop_coords = wall.drop_coords(current_drop);
        let next_drop_coords = wall.drop_coords(next_drop);

        Store {
            wall,
            current_drop,
            current_drop_coords,
            next_drop,
            next_drop_coords,
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
        new_drop_coords
            .iter()
            .any(|&(x, y)| x < 0 || x >= self.wall.cols as i32 || y >= self.wall.rows as i32)
            || new_drop_coords
                .iter()
                .any(|&(x, y)| x >= 0 && y >= 0 && self.wall.bricks[y as usize][x as usize] == Fill)
    }
}

impl Store {
    fn fill_in(&mut self) {
        for &(x, y) in &self.current_drop_coords {
            if y < 0 {
                self.game_over = true;
            } else {
                self.wall.bricks[y as usize][x as usize] = Fill;
            }
        }

        self.wall
            .bricks
            .retain(|row| row.iter().any(|&b| b == Empty));
        self.current_drop = self.next_drop;
        self.current_drop_coords = self.next_drop_coords;
        self.next_drop = random_drop();
        self.next_drop_coords = self.wall.drop_coords(self.next_drop);

        let rows_remain = self.wall.bricks.len();
        if rows_remain < self.wall.rows {
            let rows_missing = self.wall.rows - rows_remain;
            let mut new_rows = vec![vec![Empty; self.wall.cols]; rows_missing];
            new_rows.append(&mut self.wall.bricks);
            self.wall.bricks = new_rows;
            self.score += 100 * 2u32.pow(rows_missing as u32 - 1);
            self.level = derived_level(self.score);
            self.speed = derived_speed(self.level);
        }
    }
}

impl Store {
    fn move_down(&mut self) -> bool {
        let mut new_drop_coords = self.current_drop_coords.clone();
        for c in new_drop_coords.iter_mut() {
            c.1 += 1;
        }
        if self.will_crash(new_drop_coords) {
            self.fill_in();
            false
        } else {
            self.current_drop_coords = new_drop_coords;
            true
        }
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
        let mut new_drop_coords = self.current_drop_coords.clone();
        for c in new_drop_coords.iter_mut() {
            c.0 -= 1;
        }
        if !self.will_crash(new_drop_coords) {
            self.current_drop_coords = new_drop_coords;
        }
    }
}

impl Store {
    fn move_right(&mut self) {
        let mut new_drop_coords = self.current_drop_coords.clone();
        for c in new_drop_coords.iter_mut() {
            c.0 += 1;
        }
        if !self.will_crash(new_drop_coords) {
            self.current_drop_coords = new_drop_coords;
        }
    }
}

impl Store {
    fn rotate(&mut self) {
        if self.current_drop == O {
            return;
        }
        // use `dx` to adjust origin horizontally
        'outer: for dx in [0, -1, 1, -2, 2].iter() {
            let mut new_coords = self.current_drop_coords.clone();
            // rotate origin
            let (mut x0, y0) = new_coords[1];
            x0 += dx;
            // rotate 90 degree
            // https://en.wikipedia.org/wiki/Rotation_of_axes
            for c in new_coords.iter_mut() {
                c.0 += dx;
                *c = (x0 + y0 - c.1, y0 + c.0 - x0);
            }
            if !self.will_crash(new_coords) {
                self.current_drop_coords = new_coords;
                break 'outer;
            }
        }
    }
}

impl Store {
    fn restart(&mut self) {
        self.wall = Wall::new(self.wall.rows, self.wall.cols, self.wall.brick_width);
        self.current_drop = random_drop();
        self.next_drop = random_drop();
        self.current_drop_coords = self.wall.drop_coords(self.current_drop);
        self.score = 0;
        self.game_over = false;
        self.speed = 300.0
    }

    fn pause_toggle(&mut self) {
        self.playing = !self.playing;
    }
}

impl Store {
    fn frame(&self) -> Vec<Vec<Brick>> {
        let mut bricks = self.wall.bricks.clone();
        for (x, y) in &self.current_drop_coords {
            if *y >= 0 {
                bricks[*y as usize][*x as usize] = Fill;
            }
        }
        bricks
    }
}

struct Tetris {
    store: Store,
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    delta: u32,
    header_height: u32,
    color_light: JsValue,
    color_dark: JsValue,
}

impl Tetris {
    fn new(
        canvas_element: HtmlCanvasElement,
        rows: usize,
        cols: usize,
        brick_width: u32,
    ) -> Rc<RefCell<Tetris>> {
        if rows < 10 || cols < 12 || brick_width < 5 {
            let error_str = "Required: rows >= 10 && cols >= 12 && brick_width >= 5";
            error(error_str);
            panic!(error_str);
        }

        let store = Store::new(rows, cols, brick_width);
        let delta = brick_width + 1;
        let header_height = 6 * delta;
        let width = cols as u32 * delta;
        let height = header_height as u32 + rows as u32 * delta;
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

        for row in 0..(rows + 6) {
            for col in 0..cols {
                context.fill_rect(
                    col as f64 * delta as f64,
                    row as f64 * delta as f64,
                    brick_width as f64,
                    brick_width as f64,
                );
            }
        }

        let tetris = Rc::new(RefCell::new(Tetris {
            store,
            context,
            width,
            height,
            delta,
            header_height,
            color_light,
            color_dark,
        }));

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

        let brick_width = self.store.wall.brick_width as f64;
        let header_height = self.header_height as f64;
        let delta = self.delta as f64;
        if self.store.game_over {
            self.context.set_font("18px sans-serif");
            self.context
                .fill_text("Game Over!", x_center, 35.0)
                .unwrap();
            self.context
                .fill_text("Press `enter` restart.", x_center, 55.0)
                .unwrap();
        } else {
            for (x, y) in self.store.next_drop_coords.iter() {
                self.context.fill_rect(
                    *x as f64 * delta,
                    header_height + *y as f64 * delta,
                    brick_width,
                    brick_width,
                );
            }
        }

        let frame = self.store.frame();
        for row in 0..self.store.wall.rows {
            for col in 0..self.store.wall.cols {
                self.context.set_fill_style(if frame[row][col] == Fill {
                    &self.color_dark
                } else {
                    &self.color_light
                });
                self.context.fill_rect(
                    col as f64 * delta,
                    header_height + row as f64 * delta,
                    brick_width,
                    brick_width,
                );
            }
        }
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("global `window` should be OK")
}

fn setup_keyboard_event(tetris: Rc<RefCell<Tetris>>) {
    let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let mut t = tetris.borrow_mut();
        let yep = t.store.playing && !t.store.game_over;
        match e.key().as_str() {
            "ArrowUp" | "w" | "i" if yep => t.store.rotate(),
            "ArrowRight" | "d" | "l" if yep => t.store.move_right(),
            "ArrowLeft" | "a" | "j" if yep => t.store.move_left(),
            "ArrowDown" | "s" | "k" if yep => {
                t.store.move_down();
            }
            "p" => {
                t.store.pause_toggle();
                return;
            }
            "r" => t.store.restart(),
            " " if yep => {
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
            _ => (),
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
        .expect("`requestAnimationFrame` should be OK");
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
