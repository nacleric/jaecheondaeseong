#[cfg(test)]
mod unit_tests;

pub mod mock_terminal; // TODO: change this back to private
pub mod termion_adapter;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{TtyControl, View, ViewBuffer},
};
use std::io::{stdin, stdout, Stdin, Stdout, Write};

#[derive(Clone, Debug)]
pub struct TerminalBuffer {
    cells: Vec<Vec<Option<char>>>, // TODO: Might need to be generic?
    width: usize,
    height: usize,
}

impl TerminalBuffer {
    pub fn new() -> Self {
        Self {
            cells: vec![vec![None]],
            width: WIDTH, // TODO: width and height will need to be dynamic, placeholders for now
            height: HEIGHT,
        }
    }

    fn get_data(&self) -> &Vec<Vec<Option<char>>> {
        &self.cells
    }

    fn set_data(&mut self, cells: Vec<Vec<Option<char>>>) {
        self.cells = cells;
    }
}

// Responsible for ANSI control
impl ViewBuffer for TerminalBuffer {
    fn clear(&mut self) {
        unimplemented!()
    }

    // TODO: Bring out Cursor Position
    fn write(&mut self, graphemes: &[char]) {
        let mut view_buffer = self.view_buffer();
    }
}

pub struct Terminal<VB, TC: TtyControl> {
    view_buffer: VB,
    tty_control: TC,
    input: Stdin,
    output: Stdout,
}

impl<VB, TC: TtyControl> Terminal<VB, TC>
where
    VB: ViewBuffer + Clone,
{
    pub fn new(view: VB, tty_control: TC) -> Self {
        let stdin = stdin();
        let stdout = stdout();
        Self {
            view_buffer: view,
            tty_control,
            input: stdin,
            output: stdout,
        }
    }

    pub fn view_buffer(&self) -> &VB {
        &self.view_buffer
    }
}

// TODO: ask Brad to review this
impl<VB: ViewBuffer, TC: TtyControl> View for Terminal<VB, TC>
where
    VB: ViewBuffer + Clone,
{
    fn clear(&mut self) -> Result<(), std::io::Error> {
        match write!(self.output, "{}", termion::clear::All) {
            Ok(_) => self.output.flush(),
            Err(e) => panic!("Problem writing to screen: {:?}", e),
        }
    }

    fn print_to_screen(&mut self, grapheme: char) {
        // self.write_to_buffer(grapheme);
        write!(self.output, "{}", grapheme).expect("words on screen");
    }
}
