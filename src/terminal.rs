#[cfg(test)]
mod unit_tests;

use crate::{
    adapters::termion_adapter,
    Result,
    traits::{Buffer, TtyControl, View},
    model::PositionBuffer
};

use std::io::{stdin, stdout, Stdin, Stdout, Write};

pub struct Terminal<B, TC> {
    buffer: B,
    tty_control: TC,
    input: Stdin,
    output: Stdout,
}

impl<B, TC> Terminal<B, TC>
where
    B: Buffer + PositionBuffer +  Clone,
    TC: TtyControl,
{
    pub fn new(buffer: B, tty_control: TC) -> Self {
        let stdin = stdin();
        let stdout = stdout();
        Self {
            buffer,
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
impl<B, TC> View for Terminal<B, TC>
where
    B: Buffer + Clone,
    TC: TtyControl,
{
    fn clear(&mut self) -> Result<(), std::io::Error> {
        write!(self.output, "{}", termion::clear::All)
    }

    fn show(&self) {
        todo!()
    }
}
