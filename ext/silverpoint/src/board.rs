// Copyright (c) 2023 Lily Lyons
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chess_engine::Evaluate;
use magnus::{function, method, Module, Object};

use crate::{
    enums::{Color, GameResult, Move, Piece},
    position::Position,
};

unsafe fn call_without_gvl<Func, FuncReturn>(func: Func) -> FuncReturn
where
    Func: FnMut() -> FuncReturn,
{
    use std::ffi::c_void;

    unsafe extern "C" fn anon_func<Func, FuncReturn>(data: *mut c_void) -> *mut c_void
    where
        Func: FnMut() -> FuncReturn,
    {
        let mut func: Func = *Box::from_raw(data as *mut Func);

        Box::into_raw(Box::new(func())) as *mut _
    }

    //? SAFETY: We box the function and args to pass them over the FFI boundary.
    let boxed_args = Box::new(func);

    let result = rb_sys::rb_thread_call_without_gvl(
        Some(anon_func::<Func, FuncReturn>),
        Box::into_raw(boxed_args) as *mut _,
        None,
        std::ptr::null_mut(),
    );

    *Box::from_raw(result as _)
}

macro_rules! no_gvl {
    ($fun:expr) => {
        unsafe { call_without_gvl(|| $fun) }
    };
}

#[magnus::wrap(class = "Silverpoint::Board", size, free_immediately)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Board(chess_engine::Board);

impl From<chess_engine::Board> for Board {
    fn from(value: chess_engine::Board) -> Self {
        Self(value)
    }
}

impl From<Board> for chess_engine::Board {
    fn from(value: Board) -> Self {
        value.0
    }
}

impl Board {
    fn new() -> Self {
        Self(Default::default())
    }

    fn horde() -> Self {
        Self(chess_engine::Board::horde())
    }

    fn empty() -> Self {
        Self(chess_engine::Board::empty())
    }

    fn rating_bar(&self, len: usize) -> String {
        no_gvl!(self.0.rating_bar(len))
    }

    fn get_turn_color(&self) -> Color {
        self.0.get_turn_color().into()
    }

    fn get_en_passant(&self) -> Option<Position> {
        self.0.get_en_passant().map(Into::into)
    }

    fn remove_all(&self, &color: &Color) -> Self {
        self.0.remove_all(color.into()).into()
    }

    fn queen_all(&self, &color: &Color) -> Self {
        self.0.queen_all(color.into()).into()
    }

    fn set_turn(&self, &color: &Color) -> Self {
        self.0.set_turn(color.into()).into()
    }

    fn get_material_advantage(&self, &color: &Color) -> i32 {
        no_gvl!(self.0.get_material_advantage(color.into()))
    }

    fn get_piece(&self, &pos: &Position) -> Option<Piece> {
        self.0.get_piece(pos.into()).map(Into::into)
    }

    fn has_ally_piece(&self, &pos: &Position, &ally_color: &Color) -> bool {
        self.0.has_ally_piece(pos.into(), ally_color.into())
    }

    fn has_enemy_piece(&self, &pos: &Position, &ally_color: &Color) -> bool {
        self.0.has_enemy_piece(pos.into(), ally_color.into())
    }

    fn has_piece(&self, &pos: &Position) -> bool {
        self.0.has_piece(pos.into())
    }

    fn has_no_piece(&self, &pos: &Position) -> bool {
        self.0.has_no_piece(pos.into())
    }

    fn get_king_pos(&self, &color: &Color) -> Option<Position> {
        self.0.get_king_pos(color.into()).map(Into::into)
    }

    fn is_threatened(&self, &pos: &Position, &ally_color: &Color) -> bool {
        no_gvl!(self.0.is_threatened(pos.into(), ally_color.into()))
    }

    fn is_in_check(&self, &color: &Color) -> bool {
        no_gvl!(self.0.is_in_check(color.into()))
    }

    fn can_kingside_castle(&self, &color: &Color) -> bool {
        self.0.can_kingside_castle(color.into())
    }

    fn can_queenside_castle(&self, &color: &Color) -> bool {
        self.0.can_queenside_castle(color.into())
    }

    fn has_sufficient_material(&self, &color: &Color) -> bool {
        no_gvl!(self.0.has_sufficient_material(color.into()))
    }

    fn has_insufficient_material(&self, &color: &Color) -> bool {
        no_gvl!(self.0.has_insufficient_material(color.into()))
    }

    fn is_stalemate(&self) -> bool {
        no_gvl!(self.0.is_stalemate())
    }

    fn is_checkmate(&self) -> bool {
        no_gvl!(self.0.is_checkmate())
    }

    fn change_turn(&self) -> Self {
        self.0.change_turn().into()
    }

    fn play_move(&self, &m: &Move) -> GameResult {
        no_gvl!(self.0.play_move(m.into()).into())
    }

    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    fn inspect(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl Board {
    fn value_for(&self, &color: &Color) -> f64 {
        no_gvl!(self.0.value_for(color.into()))
    }

    fn get_current_player_color(&self) -> Color {
        self.0.get_current_player_color().into()
    }

    fn apply_eval_move(&self, &m: &Move) -> Self {
        self.0.apply_eval_move(m.into()).into()
    }

    fn get_legal_moves(&self) -> Vec<Move> {
        no_gvl! {
            self.0
            .get_legal_moves()
            .into_iter()
            .map(Into::into)
            .collect()
        }
    }

    fn get_best_next_move(&self, depth: i32) -> (Move, u64, f64) {
        let (m, u, f) = no_gvl!(self.0.get_best_next_move(depth));

        (m.into(), u, f)
    }

    fn get_worst_next_move(&self, depth: i32) -> (Move, u64, f64) {
        let (m, u, f) = no_gvl!(self.0.get_worst_next_move(depth));

        (m.into(), u, f)
    }

    fn minimax(
        &self,
        depth: i32,
        alpha: f64,
        beta: f64,
        is_maximizing: bool,
        &getting_move_for: &Color,
        mut board_count: u64,
    ) -> (f64, u64) {
        no_gvl! {
            (
                self.0.minimax(
                    depth,
                    alpha,
                    beta,
                    is_maximizing,
                    getting_move_for.into(),
                    &mut board_count,
                ),
                board_count,
            )
        }
    }
}

pub fn bind(module: impl Module) -> Result<(), magnus::Error> {
    let class = module.define_class("Board", Default::default())?;
    class.define_singleton_method("new", function!(Board::new, 0))?;
    class.define_singleton_method("horde", function!(Board::horde, 0))?;
    class.define_singleton_method("empty", function!(Board::empty, 0))?;

    class.define_method("rating_bar", method!(Board::rating_bar, 1))?;
    class.define_method("turn_color", method!(Board::get_turn_color, 0))?;
    class.define_method("en_passant", method!(Board::get_en_passant, 0))?;
    class.define_method("remove_all", method!(Board::remove_all, 1))?;
    class.define_method("queen_all", method!(Board::queen_all, 1))?;
    class.define_method("turn=", method!(Board::set_turn, 1))?;
    class.define_method(
        "material_advantage",
        method!(Board::get_material_advantage, 1),
    )?;
    class.define_method("piece", method!(Board::get_piece, 1))?;
    class.define_method("has_ally_piece?", method!(Board::has_ally_piece, 2))?;
    class.define_method("has_enemy_piece", method!(Board::has_enemy_piece, 2))?;
    class.define_method("has_piece?", method!(Board::has_piece, 1))?;
    class.define_method("has_no_piece?", method!(Board::has_no_piece, 1))?;
    class.define_method("king_pos", method!(Board::get_king_pos, 1))?;
    class.define_method("threatened?", method!(Board::is_threatened, 2))?;
    class.define_method("in_check?", method!(Board::is_in_check, 1))?;
    class.define_method(
        "can_kingside_castle?",
        method!(Board::can_kingside_castle, 1),
    )?;
    class.define_method(
        "can_queenside_castle?",
        method!(Board::can_queenside_castle, 1),
    )?;
    class.define_method(
        "has_sufficient_material?",
        method!(Board::has_sufficient_material, 1),
    )?;
    class.define_method(
        "has_insufficient_material?",
        method!(Board::has_insufficient_material, 1),
    )?;
    class.define_method("stalemate?", method!(Board::is_stalemate, 0))?;
    class.define_method("checkmate?", method!(Board::is_checkmate, 0))?;
    class.define_method("change_turn", method!(Board::change_turn, 0))?;
    class.define_method("play_move", method!(Board::play_move, 1))?;

    class.define_method("value_for", method!(Board::value_for, 1))?;
    class.define_method(
        "current_player_color",
        method!(Board::get_current_player_color, 0),
    )?;
    class.define_method("apply_eval_move", method!(Board::apply_eval_move, 1))?;
    class.define_method("legal_moves", method!(Board::get_legal_moves, 0))?;
    class.define_method("best_next_move", method!(Board::get_best_next_move, 1))?;
    class.define_method("worst_next_move", method!(Board::get_worst_next_move, 1))?;
    class.define_method("minimax", method!(Board::minimax, 6))?;

    class.define_method("inspect", method!(Board::inspect, 0))?;
    class.define_method("to_s", method!(Board::to_string, 0))?;
    class.define_method("==", method!(Board::eq, 1))?;
    class.define_method("!=", method!(Board::ne, 1))?;

    Ok(())
}
