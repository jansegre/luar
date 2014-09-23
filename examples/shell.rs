extern crate luar;

fn main() {
    let mut state = luar::State::new();
    state.shell();
}
