pub trait View {
    fn clear(&mut self) -> Result<(), std::io::Error>;
    fn print_to_screen(&mut self, graphemes: char);
    fn write_to_buffer(&mut self, graphemes: char);
}
