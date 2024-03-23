mod tutorial_window;

fn main() {
    pollster::block_on(tutorial_window::run());
}
