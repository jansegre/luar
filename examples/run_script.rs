#![feature(overloaded_calls)]
extern crate luar;

use std::collections::HashMap;
use luar::State;

// rewrite of http://lua-users.org/wiki/SimpleLuaApiExample
fn main() {
    let mut state = State::new();

    let mut table = HashMap::new();
    for i in range(1i, 6) {
        //table[i] = i * 2;
        table.insert(i, i * 2);
    }

    // this is seemingly hard to implement:
    //state["foo"] = table;
    // meanwhile this is the norm
    state.insert("foo", table);

    // load the script
    match state.load(&Path::new("./script.lua")) {
        Ok(()) => (),
        Err(e) => fail!(e),
    }

    // run the script
    state();

    // get its return
    // either one will do:
    let sum: f64 = state.pop().unwrap();
    //let sum = state.pop::<f64>().unwrap();
    println!("Script returned {:0.f}", sum);
}
