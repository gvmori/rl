extern crate tcod;
use tcod::console::*;
use tcod::colors::*;

pub struct Object {
    x: i32,
    y: i32,
    character: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, character: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            character: character,
            color: color,
        }
    }

    pub fn move_by (&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.character, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}