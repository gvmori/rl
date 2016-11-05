use tcod::colors::Color;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 60;
pub const MAP_WIDTH: i32 = SCREEN_WIDTH;
pub const MAP_HEIGHT: i32 = SCREEN_HEIGHT;
pub const LIMIT_FPS: i32 = 30;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 100 };