/* Tortoise: A rule-based fractal generator based on
 * turtle graphics & Lindenmayer systems.
 * Copyright (C) 2020 Tim Baccaert <timbaccaert@protonmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

mod canvas_engine;
mod vec_math;
mod turtle;

use wasm_bindgen::prelude::*;
use canvas_engine::{CanvasEngine, Canvas, Context};
use turtle::Turtle;
use std::f64::consts::*;

struct State {
    turtle: Turtle,
    amt: usize
}

fn update(state: &mut State, _: &mut Canvas, _: &mut Context) {
    if state.amt % 2 == 0 {
        state.turtle.turn(0.0);
    } else {
        state.turtle.turn(FRAC_PI_4);
    }

    state.turtle.forward(1.0);
    state.amt += 1;
}

fn render(state: &mut State, _: &mut Canvas, ctx: &mut Context) {
    ctx.set_line_width(25.0);
    ctx.set_stroke_style(&"rgb(255,0,0)".into());
    state.turtle.draw(ctx);
}

#[wasm_bindgen(start)]
pub fn start() {
    let state = State { turtle: Turtle::new(), amt: 0 };
    let engine = CanvasEngine::new("canvas", state, update, render);
    engine.start();
}
