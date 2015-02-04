#![allow(unstable)]

extern crate luar;

use std::collections::HashMap;
use luar::State;

// rewrite of http://lua-users.org/wiki/SimpleLuaApiExample
fn main() {
    let mut state = State::new();

    let mut table = HashMap::new();
    for i in range(1, 6) {
        table.insert(i, i * 2);
    }

    state.insert("foo", table);

    // evaluate a script
    state.eval(&Path::new("./script.lua")).unwrap();

    // get its return either one will do:
    let sum: f64 = state.pop().unwrap();
    //let sum = state.pop::<f64>().unwrap();

    println!("Script returned {:0.}", sum);
}
