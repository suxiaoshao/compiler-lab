#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Position {
    pub(in crate::tokenizer) x: usize,
    pub(in crate::tokenizer) y: usize,
}
impl Position {
    pub(in crate::tokenizer) fn change_from_char(&mut self, c: char) -> () {
        match c {
            '\n' => {
                self.y += 1;
                self.x = 1
            }
            '\r' => {}
            _ => self.x += 1,
        }
    }
    pub fn new() -> Self {
        Self { x: 1, y: 1 }
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.y, self.x)
    }
    pub fn new_(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
