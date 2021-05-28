#[derive(Clone)]
pub(crate) struct Position {
    x: u64,
    y: u64,
}
impl Position {
    pub(in crate::tokenizer) fn change_from_char(&mut self, c: char) -> () {
        match c {
            '\n' => self.y += 1,
            _ => self.x += 1,
        }
    }
    pub fn new() -> Self {
        Self { x: 1, y: 1 }
    }
}
