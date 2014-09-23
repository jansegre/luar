#![feature(overloaded_calls)]
extern crate luar;

fn main() {
    let mut state = luar::State::new();

    // insert some variables
    state.insert("a", "Hello ");
    state.insert("b", "World!");

    // load some code
    state.load("print(a .. b)").unwrap();

    // run that code
    state();
}
