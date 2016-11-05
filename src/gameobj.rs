extern crate tcod;
use tcod::console::*;
use tcod::colors::*;

use map::Map;

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

    pub fn move_by (&mut self, map: &Map, dx: i32, dy: i32) {
        let tx = self.x + dx;
        let ty = self.y + dy;

        if tx < 0 || tx >= map.len() as i32 
            || ty < 0 || ty >= map[0].len() as i32
            || map[tx as usize][ty as usize].blocked {
            return
        }

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