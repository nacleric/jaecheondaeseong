// TODO: Remove this later for moy dynamic resizing, Width and Height will mostly be in the buffer type
use crate::consts::{HEIGHT, WIDTH};

pub trait ViewBuffer {
    fn clear(&mut self);
}
