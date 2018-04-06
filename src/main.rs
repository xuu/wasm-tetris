#[macro_use]
extern crate stdweb;

#[derive(Debug, Clone)]
enum Bric {
    // Black,
    Gray,
}

#[derive(Debug)]
struct Wall {
    brics: Vec<(usize, usize, Bric)>,
    width: usize,
    height: usize,
}

use Bric::*;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, CanvasRenderingContext2d};
use stdweb::web::html_element::CanvasElement;

impl Wall {
    fn new(width: usize, height: usize) -> Wall {
        let mut brics: Vec<(usize, usize, Bric)> = Vec::new();
        for i in 0..width {
            for j in 0..height {
                brics.push((i, j, Gray))
            }
        }
        Wall {
            brics,
            width,
            height,
        }
    }
}

fn paint(canvas: CanvasElement, wall: &Wall) {
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.set_fill_style_color("#eee");

    for (x, y, _b) in &wall.brics {
        context.fill_rect(*x as f64 * 11f64, *y as f64 * 11f64, 10f64, 10f64);
    }

    js! {
        console.log("d");
    }
}

fn main() {
    let w = Wall::new(30, 60);
    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    canvas.set_width(w.width as u32 * 11);
    canvas.set_height(w.height as u32 * 11);

    paint(canvas, &w);
}
