#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;

pub mod widgets;

mod app;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() { yew::start_app::<app::Model>(); }
