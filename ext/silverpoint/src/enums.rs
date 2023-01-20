// Copyright (c) 2023 Lily Lyons
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use magnus::{function, method, Module, Object};

use crate::{board::Board, position::Position};

#[magnus::wrap(class = "Silverpoint::Color", size, free_immediately)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Color(chess_engine::Color);

impl From<chess_engine::Color> for Color {
    fn from(value: chess_engine::Color) -> Self {
        Self(value)
    }
}

impl From<Color> for chess_engine::Color {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl Color {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    fn inspect(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[magnus::wrap(class = "Silverpoint::Piece", size, free_immediately)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Piece(chess_engine::Piece);

impl From<chess_engine::Piece> for Piece {
    fn from(value: chess_engine::Piece) -> Self {
        Self(value)
    }
}

impl From<Piece> for chess_engine::Piece {
    fn from(value: Piece) -> Self {
        value.0
    }
}

impl Piece {
    fn get_name(&self) -> String {
        self.0.get_name().to_string()
    }

    fn get_material_value(&self) -> i32 {
        self.0.get_material_value()
    }

    fn with_color(&self, &color: &Color) -> Self {
        self.0.with_color(color.0).into()
    }

    fn get_color(&self) -> Color {
        self.0.get_color().into()
    }

    fn get_pos(&self) -> Position {
        self.0.get_pos().into()
    }

    fn is_king(&self) -> bool {
        self.0.is_king()
    }

    fn is_queen(&self) -> bool {
        self.0.is_queen()
    }

    fn is_rook(&self) -> bool {
        self.0.is_rook()
    }

    fn is_bishop(&self) -> bool {
        self.0.is_bishop()
    }

    fn is_knight(&self) -> bool {
        self.0.is_knight()
    }

    fn is_pawn(&self) -> bool {
        self.0.is_pawn()
    }

    fn is_starting_pawn(&self) -> bool {
        self.0.is_starting_pawn()
    }

    fn is_queenside_rook(&self) -> bool {
        self.0.is_queenside_rook()
    }

    fn is_kingside_rook(&self) -> bool {
        self.0.is_kingside_rook()
    }

    fn move_to(&self, &position: &Position) -> Self {
        self.0.move_to(position.into()).into()
    }

    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    fn inspect(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[magnus::wrap(class = "Silverpoint::Move", size, free_immediately)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Move(chess_engine::Move);

impl From<chess_engine::Move> for Move {
    fn from(value: chess_engine::Move) -> Self {
        Self(value)
    }
}

impl From<Move> for chess_engine::Move {
    fn from(value: Move) -> Self {
        value.0
    }
}

impl Move {
    fn new_queenside_castle() -> Self {
        Self(chess_engine::Move::QueenSideCastle)
    }

    fn new_kingside_castle() -> Self {
        Self(chess_engine::Move::KingSideCastle)
    }

    fn new_piece(&pos1: &Position, &pos2: &Position) -> Self {
        Self(chess_engine::Move::Piece(pos1.into(), pos2.into()))
    }

    fn new_resign() -> Self {
        Self(chess_engine::Move::Resign)
    }

    fn parse(str: String) -> Result<Self, magnus::Error> {
        chess_engine::Move::parse(str)
            .map(Into::into)
            .map_err(magnus::Error::runtime_error)
    }

    fn is_queenside_castle(&self) -> bool {
        matches!(self.0, chess_engine::Move::QueenSideCastle)
    }

    fn is_kingside_castle(&self) -> bool {
        matches!(self.0, chess_engine::Move::KingSideCastle)
    }

    fn is_piece(&self) -> bool {
        matches!(self.0, chess_engine::Move::Piece(..))
    }

    fn is_resign(&self) -> bool {
        matches!(self.0, chess_engine::Move::Resign)
    }

    fn piece_positions(&self) -> Option<(Position, Position)> {
        if let chess_engine::Move::Piece(pos1, pos2) = self.0 {
            Some((pos1.into(), pos2.into()))
        } else {
            None
        }
    }

    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    fn inspect(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[magnus::wrap(class = "Silverpoint::GameResult", size, free_immediately)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct GameResult(chess_engine::GameResult);

impl From<chess_engine::GameResult> for GameResult {
    fn from(value: chess_engine::GameResult) -> Self {
        Self(value)
    }
}

impl From<GameResult> for chess_engine::GameResult {
    fn from(value: GameResult) -> Self {
        value.0
    }
}

impl GameResult {
    fn is_continuing(&self) -> bool {
        matches!(self.0, chess_engine::GameResult::Continuing(..))
    }

    fn is_victory(&self) -> bool {
        matches!(self.0, chess_engine::GameResult::Victory(..))
    }

    fn is_stalemate(&self) -> bool {
        matches!(self.0, chess_engine::GameResult::Stalemate)
    }

    fn is_illegal_move(&self) -> bool {
        matches!(self.0, chess_engine::GameResult::IllegalMove(..))
    }

    fn next_board(&self) -> Option<Board> {
        if let chess_engine::GameResult::Continuing(board) = self.0 {
            Some(board.into())
        } else {
            None
        }
    }

    fn winning_color(&self) -> Option<Color> {
        if let chess_engine::GameResult::Victory(color) = self.0 {
            Some(color.into())
        } else {
            None
        }
    }

    fn illegal_move(&self) -> Option<Move> {
        if let chess_engine::GameResult::IllegalMove(move_) = self.0 {
            Some(move_.into())
        } else {
            None
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    let class = module.define_class("Color", Default::default())?;
    class.const_set("White", Color(chess_engine::Color::White))?;
    class.const_set("Black", Color(chess_engine::Color::Black))?;

    class.define_method("inspect", method!(Color::inspect, 0))?;
    class.define_method("to_s", method!(Color::to_string, 0))?;
    class.define_method("==", method!(Color::eq, 1))?;
    class.define_method("!=", method!(Color::ne, 1))?;

    let class = module.define_class("Piece", Default::default())?;
    class.define_method("name", method!(Piece::get_name, 0))?;
    class.define_method("material_value", method!(Piece::get_material_value, 0))?;
    class.define_method("with_color", method!(Piece::with_color, 1))?;
    class.define_method("color", method!(Piece::get_color, 0))?;
    class.define_method("pos", method!(Piece::get_pos, 0))?;
    class.define_method("king?", method!(Piece::is_king, 0))?;
    class.define_method("queen?", method!(Piece::is_queen, 0))?;
    class.define_method("rook?", method!(Piece::is_rook, 0))?;
    class.define_method("bishop?", method!(Piece::is_bishop, 0))?;
    class.define_method("knight?", method!(Piece::is_knight, 0))?;
    class.define_method("pawn?", method!(Piece::is_pawn, 0))?;
    class.define_method("starting_pawn?", method!(Piece::is_starting_pawn, 0))?;
    class.define_method("queenside_rook?", method!(Piece::is_queenside_rook, 0))?;
    class.define_method("kingside_rook?", method!(Piece::is_kingside_rook, 0))?;
    class.define_method("move", method!(Piece::move_to, 1))?;

    class.define_method("inspect", method!(Piece::inspect, 0))?;
    class.define_method("to_s", method!(Piece::to_string, 0))?;
    class.define_method("==", method!(Piece::eq, 1))?;
    class.define_method("!=", method!(Piece::ne, 1))?;

    let class = module.define_class("Move", Default::default())?;
    class.define_singleton_method(
        "new_queenside_castle",
        function!(Move::new_queenside_castle, 0),
    )?;
    class.define_singleton_method(
        "new_kingside_castle",
        function!(Move::new_kingside_castle, 0),
    )?;
    class.define_singleton_method("new_resign", function!(Move::new_resign, 0))?;
    class.define_singleton_method("new_piece", function!(Move::new_piece, 2))?;
    class.define_singleton_method("parse", function!(Move::parse, 1))?;

    class.define_method("queenside_castle?", method!(Move::is_queenside_castle, 0))?;
    class.define_method("kingside_castle?", method!(Move::is_kingside_castle, 0))?;
    class.define_method("piece?", method!(Move::is_piece, 0))?;
    class.define_method("resign?", method!(Move::is_resign, 0))?;
    class.define_method("piece_positions", method!(Move::piece_positions, 0))?;

    class.define_method("inspect", method!(Move::inspect, 0))?;
    class.define_method("to_s", method!(Move::to_string, 0))?;
    class.define_method("==", method!(Move::eq, 1))?;
    class.define_method("!=", method!(Move::ne, 1))?;

    let class = module.define_class("GameResult", Default::default())?;
    class.define_method("continuing?", method!(GameResult::is_continuing, 0))?;
    class.define_method("victory?", method!(GameResult::is_victory, 0))?;
    class.define_method("stalemate?", method!(GameResult::is_stalemate, 0))?;
    class.define_method("illegal_move?", method!(GameResult::is_illegal_move, 0))?;
    class.define_method("next_board", method!(GameResult::next_board, 0))?;
    class.define_method("winning_color", method!(GameResult::winning_color, 0))?;
    class.define_method("illegal_move", method!(GameResult::illegal_move, 0))?;

    class.define_method("inspect", method!(GameResult::to_string, 0))?;
    class.define_method("to_s", method!(GameResult::to_string, 0))?;
    class.define_method("==", method!(GameResult::eq, 1))?;
    class.define_method("!=", method!(GameResult::ne, 1))?;

    Ok(())
}
