#[derive(Clone, Debug)]
pub(crate) struct Position {
    pub(in crate::tokenizer) x: u64,
    pub(in crate::tokenizer) y: u64,
}
impl Position {
    pub(in crate::tokenizer) fn change_from_char(&mut self, c: char) -> () {
        match c {
            '\n' => {
                self.y += 1;
                self.x = 1
            }
            _ => self.x += 1,
        }
    }
    pub fn new() -> Self {
        Self { x: 1, y: 1 }
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.y, self.x)
    }
    pub fn back(&mut self) -> () {
        self.x -= 1;
    }
}