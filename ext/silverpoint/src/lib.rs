// Copyright (c) 2023 Lily Lyons
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
#![warn(rust_2018_idioms, clippy::all, clippy::pedantic)]

mod board;
mod enums;
mod position;
mod square;

#[magnus::init]
pub fn init() -> Result<(), magnus::Error> {
    let module = magnus::define_module("Silverpoint")?;
    board::bind(module)?;
    square::bind(module)?;
    position::bind(module)?;
    enums::bind(module)?;

    Ok(())
}
