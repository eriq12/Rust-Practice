mod tutorial_window;

// using tutorial from https://sotrh.github.io/learn-wgpu/

fn main() {
    pollster::block_on(tutorial_window::run());
}
