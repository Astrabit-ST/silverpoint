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
pub fn init(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let module = magnus::define_module("Silverpoint")?;
    board::bind(ruby, module)?;
    square::bind(ruby, module)?;
    position::bind(ruby, module)?;
    enums::bind(ruby, module)?;

    Ok(())
}
