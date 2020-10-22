#[cfg(test)]
mod unit_tests;
use crate::{
    error::{Error, Result},
    interfaces::GlyphBuffer,
    Position,
};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    data: String,
    pos: Position,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: String::default(),
            pos: Position::default(),
        }
    }

    /// Removes data from the buffer but does not remove the entire buffer
    pub fn delete_glyphs(&mut self) -> &mut Self {
        self.data.clear();
        self
    }

    pub fn insert_glyphs<I: Iterator<Item = char>>(&mut self, glyphs: I) -> Result<&mut Self> {
        // `try_for_each` wants `Result<(), E>` from each iteration (wants `Ok(())` or `Err(E)`)
        glyphs
            .into_iter()
            .try_for_each(|c| self.insert_glyph(c).map(|_| ()))?;
        Ok(self)
    }
}

impl GlyphBuffer for Buffer {
    type Error = Error;

    fn contents(&self) -> &[u8] {
        self.data.as_bytes()
    }

    fn delete_glyph(&mut self) -> Result<&mut Self, Self::Error> {
        unimplemented!()
    }

    fn insert_glyph(&mut self, glyph: char) -> Result<&mut Self, Self::Error> {
        self.data.insert(self.pos().col(), glyph);
        self.move_right()?;

        Ok(self)
    }

    fn move_down(&mut self) -> Result<&mut Self, Self::Error> {
        self.pos = Position::new(self.pos.col(), self.pos.row().saturating_add(1));
        Ok(self)
    }

    fn move_left(&mut self) -> Result<&mut Self, Self::Error> {
        unimplemented!()
    }

    fn move_right(&mut self) -> Result<&mut Self, Self::Error> {
        unimplemented!()
    }

    fn move_up(&mut self) -> Result<&mut Self, Self::Error> {
        unimplemented!()
    }

    fn pos(&self) -> Position {
        unimplemented!()
    }

    fn set_pos(&mut self, pos: Position) -> Result<&mut Self, Self::Error> {
        unimplemented!()
    }
}

// TODO: Change the name of this later
struct BigBuffer {
    lines: Vec<Buffer>,
    pos: Position,
}

impl BigBuffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            pos: Position::default(),
        }
    }
}
