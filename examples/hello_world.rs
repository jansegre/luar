extern crate luar;

fn main() {
    let mut state = luar::State::new();

    // insert some variables
    state.insert("a", "Hello ");
    state.insert("b", "World!");

    // load and run some code
    state.eval("print(a .. b)").unwrap();
}
