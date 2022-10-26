use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

fn belongs_to_mandelbrot_set(x: f64, y: f64) -> bool {
    let maximum_iteration_limit = 10;
    let complex_number_threshold = 10.0;

    let mut real_component = x;
    let mut imaginary_component = y;

    for _i in 0..maximum_iteration_limit {
        let _real_component =
            real_component * real_component - imaginary_component * imaginary_component + x;
        let _imaginary_component = 2.0 * real_component * imaginary_component + y;

        real_component = _real_component;
        imaginary_component = _imaginary_component;
    }

    real_component * imaginary_component < complex_number_threshold
}

fn create_canvas() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    canvas.set_width(1400);
    canvas.set_height(800);
    (canvas, context)
}

static MAGNIFICATION_FACTOR: i32 = 200;
static PAN_X: f64 = 3.0;
static PAN_Y: f64 = 2.0;

#[wasm_bindgen]
pub fn draw() {
    let (canvas, context) = create_canvas();

    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            let belongs_to_set = belongs_to_mandelbrot_set(
                (x as f64) / (MAGNIFICATION_FACTOR as f64) - PAN_X,
                (y as f64) / (MAGNIFICATION_FACTOR as f64) - PAN_Y,
            );

            if belongs_to_set {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
// #[wasm_bindgen]
// pub fn draw2() -> Vec<(f64, f64)> {
//     let (canvas, context) = create_canvas();

//     let mut result = vec![];
//     for x in 0..canvas.width() {
//         for y in 0..canvas.height() {
//             let belongs_to_set = belongs_to_mandelbrot_set(
//                 (x as f64) / (MAGNIFICATION_FACTOR as f64) - PAN_X,
//                 (y as f64) / (MAGNIFICATION_FACTOR as f64) - PAN_Y,
//             );

//             if belongs_to_set {
//                 result.push((x, y))
//             }
//         }
//     }
//     result
// }

#[wasm_bindgen]
pub fn fib(n: i8) -> i32 {
    if n < 2 {
        return n as i32;
    }

    fib(n - 1) + fib(n - 2)
}
