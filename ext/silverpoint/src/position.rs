// Copyright (c) 2023 Lily Lyons
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use magnus::function;
use magnus::method;
use magnus::Module;
use magnus::Object;

use crate::enums;
use crate::enums::Color;

#[magnus::wrap(class = "Silverpoint::Position", size, free_immediately)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Position(chess_engine::Position);

unsafe impl magnus::IntoValueFromNative for Position {}

impl From<chess_engine::Position> for Position {
    fn from(value: chess_engine::Position) -> Self {
        Self(value)
    }
}

impl From<Position> for chess_engine::Position {
    fn from(value: Position) -> Self {
        value.0
    }
}

impl Position {
    fn king_pos(&color: &Color) -> Self {
        chess_engine::Position::king_pos(color.into()).into()
    }

    fn queen_pos(&color: &Color) -> Self {
        chess_engine::Position::queen_pos(color.into()).into()
    }

    fn new(row: i32, col: i32) -> Self {
        chess_engine::Position::new(row, col).into()
    }

    fn pgn(str: String) -> Result<Self, magnus::Error> {
        chess_engine::Position::pgn(&str)
            .map(Into::into)
            .map_err(|err| {
                let ruby = magnus::Ruby::get().unwrap();
                magnus::Error::new(ruby.exception_runtime_error(), err)
            })
    }

    fn is_on_board(&self) -> bool {
        self.0.is_on_board()
    }

    fn is_off_board(&self) -> bool {
        self.0.is_off_board()
    }

    fn get_row(&self) -> i32 {
        self.0.get_row()
    }

    fn get_col(&self) -> i32 {
        self.0.get_col()
    }

    fn is_diagonal_to(&self, &other: &Self) -> bool {
        self.0.is_diagonal_to(other.0)
    }

    fn is_orthogonal_to(&self, &other: &Self) -> bool {
        self.0.is_orthogonal_to(other.0)
    }

    fn is_adjacent_to(&self, &other: &Self) -> bool {
        self.0.is_adjacent_to(other.0)
    }

    fn is_below(&self, &other: &Self) -> bool {
        self.0.is_below(other.0)
    }

    fn is_above(&self, &other: &Self) -> bool {
        self.0.is_above(other.0)
    }

    fn is_left_of(&self, &other: &Self) -> bool {
        self.0.is_left_of(other.0)
    }

    fn is_right_of(&self, &other: &Self) -> bool {
        self.0.is_right_of(other.0)
    }

    fn next_below(&self) -> Self {
        self.0.next_below().into()
    }

    fn next_above(&self) -> Self {
        self.0.next_above().into()
    }

    fn pawn_up(&self, &color: &Color) -> Self {
        self.0.pawn_up(color.into()).into()
    }

    fn pawn_back(&self, &color: &Color) -> Self {
        self.0.pawn_back(color.into()).into()
    }

    fn next_left(&self) -> Self {
        self.0.next_left().into()
    }

    fn next_right(&self) -> Self {
        self.0.next_right().into()
    }

    fn is_starting_pawn(&self, &color: &enums::Color) -> bool {
        self.0.is_starting_pawn(color.into())
    }

    fn is_kingside_rook(&self) -> bool {
        self.0.is_kingside_rook()
    }

    fn is_queenside_rook(&self) -> bool {
        self.0.is_queenside_rook()
    }

    fn diagonals_to(&self, &other: &Self) -> Vec<Position> {
        self.0
            .diagonals_to(other.0)
            .into_iter()
            .map(Into::into)
            .collect()
    }

    fn orthogonals_to(&self, &other: &Self) -> Vec<Position> {
        self.0
            .orthogonals_to(other.0)
            .into_iter()
            .map(Into::into)
            .collect()
    }

    fn is_knight_move(&self, &other: &Self) -> bool {
        self.0.is_knight_move(other.0)
    }

    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    fn inspect(&self) -> String {
        format!("{:?}", self.0)
    }
}

pub fn bind(ruby: &magnus::Ruby, module: impl Module) -> Result<(), magnus::Error> {
    let class = module.define_class("Position", ruby.class_object())?;

    class.define_singleton_method("king_pos", function!(Position::king_pos, 1))?;
    class.define_singleton_method("queen_pos", function!(Position::queen_pos, 1))?;
    class.define_singleton_method("new", function!(Position::new, 2))?;
    class.define_singleton_method("pgn", function!(Position::pgn, 1))?;

    class.define_method("on_board?", method!(Position::is_on_board, 0))?;
    class.define_method("off_board?", method!(Position::is_off_board, 0))?;
    class.define_method("row", method!(Position::get_row, 0))?;
    class.define_method("col", method!(Position::get_col, 0))?;
    class.define_method("diagonal_to?", method!(Position::is_diagonal_to, 1))?;
    class.define_method("orthogonal_to?", method!(Position::is_orthogonal_to, 1))?;
    class.define_method("adjacent_to?", method!(Position::is_adjacent_to, 1))?;
    class.define_method("below?", method!(Position::is_below, 1))?;
    class.define_method("above?", method!(Position::is_above, 1))?;
    class.define_method("left_of?", method!(Position::is_left_of, 1))?;
    class.define_method("right_of?", method!(Position::is_right_of, 1))?;
    class.define_method("next_below", method!(Position::next_below, 0))?;
    class.define_method("next_above", method!(Position::next_above, 0))?;
    class.define_method("pawn_up", method!(Position::pawn_up, 1))?;
    class.define_method("pawn_back", method!(Position::pawn_back, 1))?;
    class.define_method("next_left", method!(Position::next_left, 0))?;
    class.define_method("next_right", method!(Position::next_right, 0))?;
    class.define_method("starting_pawn?", method!(Position::is_starting_pawn, 1))?;
    class.define_method("kingside_rook?", method!(Position::is_kingside_rook, 0))?;
    class.define_method("queenside_rook?", method!(Position::is_queenside_rook, 0))?;
    class.define_method("diagonals_to", method!(Position::diagonals_to, 1))?;
    class.define_method("orthogonals_to", method!(Position::orthogonals_to, 1))?;
    class.define_method("knight_move?", method!(Position::is_knight_move, 1))?;

    class.define_method("to_s", method!(Position::to_string, 0))?;
    class.define_method("inspect", method!(Position::inspect, 0))?;
    class.define_method("==", method!(Position::eq, 1))?;
    class.define_method("!=", method!(Position::ne, 1))?;

    bind_constants(class)?;

    Ok(())
}

macro_rules! pos_const {
    ($($const:ident),+) => {
        fn bind_constants(class: impl Module) -> Result<(), magnus::Error> {
            $(
                class.const_set(stringify!($const), Position(chess_engine::$const))?;
            )+

            Ok(())
        }
    };
}

pos_const! {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8
}
