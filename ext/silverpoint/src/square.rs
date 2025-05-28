// Copyright (c) 2023 Lily Lyons
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use magnus::method;
use magnus::Module;

use crate::enums;

#[magnus::wrap(class = "Silverpoint::Square", size, free_immediately)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square(chess_engine::Square);

impl From<chess_engine::Square> for Square {
    fn from(value: chess_engine::Square) -> Self {
        Self(value)
    }
}

impl From<Square> for chess_engine::Square {
    fn from(value: Square) -> Self {
        value.0
    }
}

impl Square {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get_piece(&self) -> Option<enums::Piece> {
        self.0.get_piece().map(Into::into)
    }

    fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}

pub fn bind(ruby: &magnus::Ruby, module: impl magnus::Module) -> Result<(), magnus::Error> {
    let class = module.define_class("Square", ruby.class_object())?;
    class.define_method("empty?", method!(Square::is_empty, 0))?;
    class.define_method("piece", method!(Square::get_piece, 0))?;

    class.define_method("to_s", method!(Square::to_string, 0))?;
    class.define_method("==", method!(Square::eq, 1))?;
    class.define_method("!=", method!(Square::ne, 1))?;

    class.const_set("Empty", Square(chess_engine::EMPTY_SQUARE))?;

    Ok(())
}
