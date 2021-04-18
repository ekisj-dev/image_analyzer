pub const BLACK: Color = Color {
    red: 0,
    green: 0,
    blue: 0
};

#[derive(Debug, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn green(&self) -> u8 {
        self.green
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
}