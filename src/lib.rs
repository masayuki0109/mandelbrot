use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

#[wasm_bindgen]
pub fn draw() {
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

    let magnification_factor = 200;
    let pan_x = 3.0;
    let pan_y = 2.0;

    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            let belongs_to_set = belongs_to_mandelbrot_set(
                (x as f64) / (magnification_factor as f64) - pan_x,
                (y as f64) / (magnification_factor as f64) - pan_y,
            );

            if belongs_to_set {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}

#[wasm_bindgen]
pub fn fib(n: i8) -> i32 {
    if n < 2 {
        return n as i32;
    }

    fib(n - 1) + fib(n - 2)
}
