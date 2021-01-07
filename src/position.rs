#[cfg(test)]
mod unit_tests;
use crate::{
    error::{Error, Result},
    interfaces::MovementPolicy // Might not need this
};

#[derive(Debug, Eq, PartialEq)]
pub enum ColumnState {
    BeginningOfLine,
    EndOfLine,
    MiddleOfLine,
    InvalidPosition, // Might need to be an error type
}

#[derive(Debug, Eq, PartialEq)]
pub enum RowState{
    LowerBound,
    MiddleBound,
    UpperBound,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Position {
    col: usize,
    row: usize,
}

impl Position {
    // type_constructor
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.col(), self.row())
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row(&self) -> usize {
        self.row
    }

    // TODO: Movements will need policy injected (constructor dependency injection) maybe try composition
    pub fn move_down(&self) -> Position {
        let pos = Position::new(self.col(), self.row() + 1);
        pos
    }

    // TODO: homework
    pub fn move_left(&self) -> Position {
        let row_diff = (self.col() == 0) as usize;
        Position::new(self.col().saturating_sub(1), self.row().saturating_sub(row_diff))
    }

    pub fn move_right(&self) -> Position {
        let pos = Position::new(self.col() + 1, self.row());
        pos
    }

    pub fn move_up(&self) -> Position {
        let pos = Position::new(self.col(), self.row() - 1);
        pos
    }
}

// How default is derived
// impl Default for Position {
//     fn default() -> Self {
//         Self { usize::default(), usize::default() };
//     }
// }
