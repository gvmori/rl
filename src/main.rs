extern crate tcod;
use tcod::console::*;
use tcod::colors;

mod gameobj;
mod config;

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

    while !root.window_closed() {
        for obj in &objects {
            obj.draw(&mut console);
        }
        blit(&mut console, (0,0), (config::SCREEN_WIDTH, config::SCREEN_HEIGHT), &mut root, (0,0), 1.0, 1.0);
        root.flush();
        for obj in &objects {
            obj.clear(&mut console);
        }

        if handle_keys(&mut root, &mut objects[0]) {
            break
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut gameobj::Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            let is_fullscreen = root.is_fullscreen();
            root.set_fullscreen(!is_fullscreen);
        },
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),
        _ => {},
    }

    false
}