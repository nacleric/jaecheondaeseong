pub use error::{Error, Result};

use crate::{
    interfaces::View,
    view::{termion_adapter::TermionAdapter, Terminal, TerminalBuffer},
};

mod consts;
mod error;
mod model;
mod view;

pub mod interfaces;

// New
fn main() -> Result<(), std::io::Error> {
    let terminal_buffer = TerminalBuffer::new(); // TODO: This line is pretty useless, fix the api
    let termion = TermionAdapter::new();
    let mut terminal = Terminal::new(terminal_buffer, termion);
    terminal.clear()?;
    Ok(())
}
