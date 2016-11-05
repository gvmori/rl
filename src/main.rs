extern crate tcod;
use tcod::console::*;
use tcod::colors;

mod gameobj;
mod config;
mod map;

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("rl")
        .init();

    tcod::system::set_fps(config::LIMIT_FPS);

    let player = gameobj::Object::new(config::SCREEN_WIDTH/2, config::SCREEN_HEIGHT/2, '@', colors::WHITE);
    let mut objects = [player];

    let mut console = Offscreen::new(config::SCREEN_WIDTH, config::SCREEN_HEIGHT);

    let mut map = map::gen_map(config::MAP_WIDTH as usize, config::MAP_HEIGHT as usize);

    while !root.window_closed() {
        draw_map(&mut console, &map);
        for obj in &objects {
            obj.draw(&mut console);
        }
        blit(&mut console, (0,0), (config::SCREEN_WIDTH, config::SCREEN_HEIGHT), &mut root, (0,0), 1.0, 1.0);
        root.flush();
        for obj in &objects {
            obj.clear(&mut console);
        }

        if handle_keys(&mut root, &map, &mut objects[0]) {
            break
        }
    }
}

fn draw_map(console: &mut Console, map: &map::Map) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y].block_sight {
                console.set_char_background(x as i32, y as i32, config::COLOR_DARK_WALL, BackgroundFlag::Set);
            }
            else {
                console.set_char_background(x as i32, y as i32, config::COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
}

fn handle_keys(root: &mut Root, map: &map::Map, player: &mut gameobj::Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            let is_fullscreen = root.is_fullscreen();
            root.set_fullscreen(!is_fullscreen);
        },
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => player.move_by(map, 0, -1),
        Key { code: Down, .. } => player.move_by(map, 0, 1),
        Key { code: Left, .. } => player.move_by(map, -1, 0),
        Key { code: Right, .. } => player.move_by(map, 1, 0),
        _ => {},
    }

    false
}