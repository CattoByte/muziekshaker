use game_loop::game_loop;
use game_loop::winit::event_loop::EventLoop;
use game_loop::winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let state = pollster::block_on(muziekshaker::State::new(&window));

    game_loop(
        event_loop,
        window,
        state,
        240,
        0.1,
        |g| {
            g.game.update();
        },
        |g| {
            g.game.render().unwrap(); // Do some error handling later.
        },
        |g, event| {
            if !g.game.window_handler(event) {
                g.exit();
            }
        },
    );
}
