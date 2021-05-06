pub use error::{Error, Result};

use crate::{
    traits::View,
    terminal::{termion_adapter::TermionAdapter, Terminal, TerminalBuffer},
};

mod adapters;
mod consts;
mod error;
mod model;
mod terminal;

pub mod traits;

// New
fn main() -> Result<(), std::io::Error> {
    let terminal_buffer = TerminalBuffer::new(); // TODO: This line is pretty useless, fix the api
    let termion = TermionAdapter::new();
    let mut terminal = Terminal::new(terminal_buffer, termion);
    terminal.clear()?;
    Ok(())
}
