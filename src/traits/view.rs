use crate::Result;

pub trait View {
    fn clear(&mut self) -> Result<()>;
    fn show(&self) -> Result<()>;
}
