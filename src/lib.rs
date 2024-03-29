use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::f64;

#[wasm_bindgen]
pub fn get_user_input() {
    let document = web_sys::window().unwrap().document().unwrap();

    let input = document.get_element_by_id("user_input")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| ())
        .unwrap()
        .value();

    let output: web_sys::Element = document.get_element_by_id("rust_output")
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .map_err(|_| ())
        .unwrap();

    output.set_inner_html(&input);
}

#[wasm_bindgen]
pub fn smile() {
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

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

    let message: web_sys::Element = document.get_element_by_id("message")
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .map_err(|_| ())
        .unwrap();

    message.set_inner_html("Hi there!");
}