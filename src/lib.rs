extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use js_sys::Math;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, FocusEvent, HtmlCanvasElement, KeyboardEvent};

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
enum Block {
    Blank,
    Fill,
}

use self::Block::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

use self::Tetromino::*;

fn tetro_random() -> Tetromino {
    let r = Math::floor(Math::random() * 7.0) as usize;
    [I, J, L, O, S, T, Z][r]
}

fn derived_level(score: u32) -> u32 {
    match score {
        0..=1_000 => 1,
        1_000..=3_000 => 2,
        3_000..=5_000 => 3,
        5_000..=7_000 => 4,
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

// (x, y) -> (col, row)
type TetroCoords = [(i32, i32); 4];

struct Core {
    rows: usize,
    cols: usize,
    block_width: u32,
    matrix: Vec<Vec<Block>>,
    current_tetro: Tetromino,
    current_tetro_coords: TetroCoords,
    next_tetro: Tetromino,
    next_tetro_coords: TetroCoords,
    score: u32,
    level: u32,
    speed: f64,
    playing: bool,
    game_over: bool,
}

fn tetro_coords(d: Tetromino, cols: i32) -> TetroCoords {
    let x0 = cols / 2 - 1;

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

impl Core {
    fn new(rows: usize, cols: usize, block_width: u32) -> Core {
        let current_tetro = tetro_random();
        let next_tetro = tetro_random();
        let current_tetro_coords = tetro_coords(current_tetro, cols as i32);
        let next_tetro_coords = tetro_coords(next_tetro, cols as i32);

        Core {
            rows,
            cols,
            block_width,
            matrix: vec![vec![Blank; cols]; rows],
            current_tetro,
            current_tetro_coords,
            next_tetro,
            next_tetro_coords,
            score: 0,
            level: 1,
            speed: 300.0,
            playing: false,
            game_over: false,
        }
    }
}

impl Core {
    fn will_crash(&self, new_coords: TetroCoords) -> bool {
        new_coords.iter().any(|&(x, y)| {
            x < 0
                || x >= self.cols as i32
                || y >= self.rows as i32
                || (x >= 0 && y >= 0 && self.matrix[y as usize][x as usize] == Fill)
        })
    }
}

impl Core {
    fn fill_in(&mut self) {
        for &(x, y) in &self.current_tetro_coords {
            if y < 0 {
                self.game_over = true;
            } else {
                self.matrix[y as usize][x as usize] = Fill;
            }
        }

        self.current_tetro = self.next_tetro;
        self.current_tetro_coords = self.next_tetro_coords;
        self.next_tetro = tetro_random();
        self.next_tetro_coords = tetro_coords(self.next_tetro, self.cols as i32);
        self.matrix.retain(|row| row.iter().any(|&b| b == Blank));

        let rows_remain = self.matrix.len();
        if rows_remain < self.rows {
            let rows_missing = self.rows - rows_remain;
            let mut new_matrix = vec![vec![Blank; self.cols]; rows_missing];
            new_matrix.append(&mut self.matrix);
            self.matrix = new_matrix;
            self.score += 100 * 2u32.pow(rows_missing as u32 - 1);
            self.level = derived_level(self.score);
            self.speed = derived_speed(self.level);
        }
    }
}

impl Core {
    fn move_down(&mut self) -> bool {
        let mut new_coords = self.current_tetro_coords.clone();
        for c in new_coords.iter_mut() {
            c.1 += 1;
        }
        if self.will_crash(new_coords) {
            self.fill_in();
            false
        } else {
            self.current_tetro_coords = new_coords;
            true
        }
    }
}

impl Core {
    fn drop_down(&mut self) {
        while self.move_down() {}
    }
}

impl Core {
    fn move_left(&mut self) {
        let mut new_coords = self.current_tetro_coords.clone();
        for c in new_coords.iter_mut() {
            c.0 -= 1;
        }
        if !self.will_crash(new_coords) {
            self.current_tetro_coords = new_coords;
        }
    }
}

impl Core {
    fn move_right(&mut self) {
        let mut new_coords = self.current_tetro_coords.clone();
        for c in new_coords.iter_mut() {
            c.0 += 1;
        }
        if !self.will_crash(new_coords) {
            self.current_tetro_coords = new_coords;
        }
    }
}

impl Core {
    fn rotate(&mut self) {
        if self.current_tetro == O {
            return;
        }
        // use `dx` to adjust origin horizontally
        for dx in [0, -1, 1, -2, 2].iter() {
            let mut new_coords = self.current_tetro_coords.clone();
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
                self.current_tetro_coords = new_coords;
                break;
            }
        }
    }
}

impl Core {
    fn restart(&mut self) {
        self.matrix = vec![vec![Blank; self.cols]; self.rows];
        self.current_tetro = tetro_random();
        self.next_tetro = tetro_random();
        self.current_tetro_coords = tetro_coords(self.current_tetro, self.cols as i32);
        self.score = 0;
        self.game_over = false;
        self.speed = 300.0
    }

    fn pause(&mut self, b: bool) {
        self.playing = b
    }
}

struct Tetris {
    core: Core,
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    delta: u32,
    header_height: u32,
    color_light: JsValue,
    color_dark: JsValue,
}

impl Tetris {
    fn make(
        canvas: &HtmlCanvasElement,
        rows: usize,
        cols: usize,
        block_width: u32,
    ) -> Rc<RefCell<Tetris>> {
        if rows < 10 || cols < 12 || block_width < 5 {
            let error_str = "Required: rows >= 10 && cols >= 12 && block_width >= 5";
            error(error_str);
            panic!(error_str);
        }

        let core = Core::new(rows, cols, block_width);
        let delta = block_width + 1;
        let header_height = 6 * delta;
        let width = cols as u32 * delta;
        let height = header_height as u32 + rows as u32 * delta;
        let color_light = JsValue::from_str("#eee");
        let color_dark = JsValue::from_str("#333");
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        canvas.set_width(width);
        canvas.set_height(height);
        canvas.set_attribute("tabindex", "1").unwrap();
        context.set_text_align("center");
        context.set_fill_style(&color_light);

        for row in 0..(rows + 6) {
            for col in 0..cols {
                context.fill_rect(
                    col as f64 * delta as f64,
                    row as f64 * delta as f64,
                    block_width as f64,
                    block_width as f64,
                );
            }
        }

        let tetris = Rc::new(RefCell::new(Tetris {
            core,
            context,
            width,
            height,
            delta,
            header_height,
            color_light,
            color_dark,
        }));

        setup_events(canvas, tetris.clone());
        setup_animatoin_frame(tetris.clone());
        tetris
    }
}

impl Tetris {
    fn render(&mut self) {
        let x_center = self.width as f64 / 2.0;
        let score_level = String::from("Score/Level: ")
            + &self.core.score.to_string()
            + "/"
            + &self.core.level.to_string();

        self.context
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.context.set_fill_style(&self.color_dark);
        self.context.set_font("12px sans-serif");
        self.context
            .fill_text(&score_level, x_center, 10.0)
            .unwrap();

        let block_width = self.core.block_width as f64;
        let header_height = self.header_height as f64;
        let delta = self.delta as f64;
        if self.core.game_over {
            self.context.set_font("18px sans-serif");
            self.context
                .fill_text("Game Over!", x_center, 35.0)
                .unwrap();
            self.context
                .fill_text("Press `enter` restart.", x_center, 55.0)
                .unwrap();
        } else {
            for (x, y) in self.core.next_tetro_coords.iter() {
                self.context.fill_rect(
                    *x as f64 * delta,
                    header_height + *y as f64 * delta,
                    block_width,
                    block_width,
                );
            }
        }

        for row in 0..self.core.rows {
            for col in 0..self.core.cols {
                self.context
                    .set_fill_style(if self.core.matrix[row][col] == Fill {
                        &self.color_dark
                    } else {
                        &self.color_light
                    });
                self.context.fill_rect(
                    col as f64 * delta,
                    header_height + row as f64 * delta,
                    block_width,
                    block_width,
                );
            }
        }
        self.context.set_fill_style(&self.color_dark);
        for (x, y) in self.core.current_tetro_coords.iter() {
            self.context.fill_rect(
                *x as f64 * delta,
                header_height + *y as f64 * delta,
                block_width,
                block_width,
            );
        }
    }
}

fn setup_events(canvas: &HtmlCanvasElement, tetris: Rc<RefCell<Tetris>>) {
    let t1 = tetris.clone();
    let focus_handler = Closure::wrap(Box::new(move |e: FocusEvent| {
        t1.borrow_mut().core.pause(e.type_() == "focus")
    }) as Box<dyn FnMut(_)>);

    let t2 = tetris.clone();
    let keyboard_handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let mut t = t2.borrow_mut();
        if t.core.game_over {
            match e.key().as_str() {
                "r" | "Enter" => {
                    t.core.restart();
                    t.render();
                }
                _ => (),
            }
            return;
        }
        if t.core.playing {
            match e.key().as_str() {
                "ArrowUp" | "w" | "i" => t.core.rotate(),
                "ArrowRight" | "d" | "l" => t.core.move_right(),
                "ArrowLeft" | "a" | "j" => t.core.move_left(),
                "ArrowDown" | "s" | "k" => {
                    t.core.move_down();
                }
                "p" => t.core.pause(false),
                "r" => t.core.restart(),
                " " => {
                    e.prevent_default();
                    t.core.drop_down();
                }
                "Enter" => t.core.drop_down(),
                _ => (),
            }
        } else {
            t.core.pause(true)
        }
        t.render();
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_event_listener("blur", focus_handler.as_ref().unchecked_ref())
        .unwrap();
    canvas
        .add_event_listener_with_event_listener("focus", focus_handler.as_ref().unchecked_ref())
        .unwrap();
    canvas
        .add_event_listener_with_event_listener(
            "keydown",
            keyboard_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

    focus_handler.forget();
    keyboard_handler.forget();
}

fn window() -> web_sys::Window {
    web_sys::window().expect("global `window` should be OK.")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("`requestAnimationFrame` should be OK.");
}

// https://github.com/rustwasm/wasm-bindgen/blob/3d2f548ce2/examples/request-animation-frame/src/lib.rs
fn setup_animatoin_frame(tetris: Rc<RefCell<Tetris>>) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut time_stamp = 0.0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let time = window().performance().unwrap().now();
        if time - time_stamp > tetris.borrow().core.speed {
            time_stamp = time;
            let mut t = tetris.borrow_mut();
            if t.core.playing && !t.core.game_over {
                t.core.move_down();
                t.render();
            }
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[wasm_bindgen]
pub fn make_tetris(rows: usize, cols: usize, block_width: u32) -> HtmlCanvasElement {
    let canvas = window()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    Tetris::make(&canvas, rows, cols, block_width);
    canvas
}
