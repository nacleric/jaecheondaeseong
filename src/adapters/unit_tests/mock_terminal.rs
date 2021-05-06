use crate::{
    consts::{HEIGHT, WIDTH},
    traits::ViewBuffer,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MockTerminalView {
    cells: [[Option<char>; WIDTH]; HEIGHT], // TODO: Might need to be generic?
}

impl MockTerminalView {
    pub fn new() -> Self {
        Self {
            cells: [[None; WIDTH]; HEIGHT],
        }
    }

    pub fn get_data(&self) -> [[Option<char>; WIDTH]; HEIGHT] {
        self.cells
    }

    pub fn set_data(&mut self, cells: [[Option<char>; WIDTH]; HEIGHT]) {
        self.cells = cells;
    }
}

impl ViewBuffer for MockTerminalView {
    fn clear(&mut self) {
        self.set_data([[None; WIDTH]; HEIGHT]);
    }
}
