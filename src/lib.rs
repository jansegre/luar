// Copyright (c) 2014 Jan Segre
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*! A rustic Rust API that embeds the C Lua interpreter.

TODO: introduction, examples, tests, design choices

*/

#![experimental]
#![feature(link_args)]
#![feature(macro_rules)]
#![feature(overloaded_calls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate "lua-sys" as raw;

use std::hash::Hash;
use std::collections::HashMap;
use std::io::{stdio, IoResult};
//use std::ops::Index;
use std::ptr;
use std::str::from_utf8;
use std::kinds::marker::NoSync;
use libc::{c_void, c_char, c_int, size_t};

macro_rules! c_str(
    ($t: expr) => ({let t = $t.to_c_str(); t.as_ptr()});
)

/// Main type of the API, this is an abstraction over Lua's `lua_State`.
// TODO: consider using std::kinds::marker (http://doc.rust-lang.org/std/kinds/marker/) for
// properly marking types
pub struct State {
    L: *mut raw::lua_State,
    owned: bool,
    no_sync: NoSync,
}

impl State {
    #[inline]
    pub fn new() -> State {
        match State::new_checked(true) {
            Ok(s) => s,
            Err(e) => panic!(e),
        }
    }

    #[inline]
    pub fn new_checked(with_libs: bool) -> Result<State, LuarError> {
        unsafe {
            match raw::luaL_newstate() {
                L if L.is_not_null() => {
                    let mut state = State{ L: L, owned: true, no_sync: NoSync };
                    if with_libs { state.open_libs(); }
                    Ok(state)
                },
                _ => Err(MemoryAllocationError),
            }
        }
    }

    #[inline]
    fn new_raw_tmp(L: *mut raw::lua_State) -> State {
        State{ L: L, owned: false, no_sync: NoSync }
    }

    #[inline]
    fn open_libs(&mut self) {
        unsafe {
            raw::luaL_checkversion(self.L);
            // stop gc during initialization
            raw::lua_gc(self.L, raw::LUA_GCSTOP, 0);
            raw::luaL_openlibs(self.L);
            raw::lua_gc(self.L, raw::LUA_GCRESTART, 0);
        }
    }

    /// Use this to load source code. Possible sources are strings (loaded directly) and paths
    /// (file content is loaded).
    pub fn load<L: Loader>(&mut self, source: L) -> Result<(), LuarError> {
        match LuarError::from_lua(Loader::load_to(self, source)) {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    #[inline]
    pub fn push<V: ToLua>(&mut self, val: V) {
        ToLua::push_to(self, val)
    }

    #[inline]
    pub fn pop<V: FromLua>(&mut self) -> Option<V> {
        //let top = self.top();
        //let opt_val = self.read(top);
        let opt_val = self.read(-1);
        self.remove_top();
        opt_val
    }

    #[inline]
    pub fn read<V: FromLua>(&mut self, idx: int) -> Option<V> {
        FromLua::read_from(self, idx as c_int)
    }

    #[inline]
    fn get_global<V: FromLua>(&mut self, var: &str) -> Option<V> {
        unsafe { raw::lua_getglobal(self.L, c_str!(var)); }
        self.read(-1)
    }

    #[inline]
    pub fn insert<V: ToLua>(&mut self, name: &str, val: V) {
        self.push(val);
        unsafe { raw::lua_setglobal(self.L, c_str!(name)); }
    }

    #[inline]
    pub fn get<V: FromLua>(&mut self, name: &str) -> Option<V> {
        unsafe { raw::lua_getglobal(self.L, c_str!(name)); }
        self.pop()
    }

    #[inline]
    pub fn get_or<V: FromLua>(&mut self, name: &str, def: V) -> V {
        self.get(name).unwrap_or(def)
    }

    #[inline]
    fn move_to(&mut self, index: int) {
        unsafe { raw::lua_insert(self.L, index as c_int) }
    }

    #[inline]
    fn remove(&mut self, idx: int) {
        unsafe { raw::lua_remove(self.L, idx as c_int) }
    }

    #[inline]
    fn top(&mut self) -> int {
        unsafe { raw::lua_gettop(self.L) as int }
    }

    #[inline]
    fn set_top(&mut self, index: int) {
        unsafe { raw::lua_settop(self.L, index as c_int) }
    }

    #[inline]
    fn remove_top(&mut self) {
        unsafe { raw::lua_settop(self.L, -2) }
    }

    fn load_slice(&mut self, slice: &str) -> c_int {
        let cstr = slice.to_c_str();
        unsafe { raw::luaL_loadbuffer(self.L, cstr.as_ptr(), cstr.len() as size_t, c_str!("=stdin")) }
    }

    /// Will print the prompt from lua vars `_PROMPT` or `_PROMPT2` if they're defined
    /// or use the static versions. Read a single line, and push it into lua.
    fn push_line(&mut self, first: bool) -> IoResult<()> {

        // print the prompt if defined in lua or the default
        match self.get_global::<&str>(if first { "_PROMPT" } else { "_PROMPT2" }) {
            Some(s) => print!("{:s}", s),
            None    => print!("{:s}", if first { raw::LUA_PROMPT } else { raw::LUA_PROMPT2 }),
        }
        self.remove_top(); // pop the result from getglobal
        stdio::flush();

        // read a line, could be improved to use GNU Readline
        let line = try!(stdio::stdin().read_line());
        let line = line.as_slice();
        if first && line.starts_with("=") {
            self.push(format!("return {}", line.slice_from(1)));
        } else {
            self.push(line);
        }
        Ok(())
    }

    fn load_line(&mut self) -> (bool, c_int) {
        self.set_top(0);

        let mut status: c_int = -1;
        match self.push_line(true) { Ok(()) => (), Err(_) => return (false, status) }

        loop {
            let line: &str = self.read(1).unwrap();
            status = self.load_slice(line);
            match status {
                raw::LUA_ERRSYNTAX => {
                    match self.read::<&str>(-1) {
                        Some(s) if s.ends_with("<eof>") => self.remove_top(),
                        _ => break,
                    }
                },
                _ => break,
            }

            match self.push_line(false) { Ok(()) => (), Err(_) => return (false, status) }

            // add a new line between the two lines:
            self.push("\n");
            self.move_to(-2);
            unsafe { raw::lua_concat(self.L, 3); }
        }

        // TODO: save line in history
        unsafe { raw::lua_remove(self.L, 1); } // remove line
        (true, status)
    }

    pub fn shell(&mut self) {
        loop {
            let (ok, mut status) = self.load_line();
            if !ok { break; }

            if status == raw::LUA_OK {
                status = self.call(0, raw::LUA_MULTRET);
            }
            // report
            if status != raw::LUA_OK && unsafe { raw::lua_isnil(self.L, -1) != 1 } {
                let msg: &str = match self.read(-1) {
                    Some(s) => s,
                    None => "(error object is not a string)",
                };
                println!("{}", msg);
                self.remove_top();
                // force a complete garbage collection in case of errors
                unsafe { raw::lua_gc(self.L, raw::LUA_GCCOLLECT, 0); }
            }
            if status == raw::LUA_OK && self.top() > 0 { // any result to print
                unsafe {
                    raw::luaL_checkstack(self.L, raw::LUA_MINSTACK, c_str!("too many results to print"));
                    raw::lua_getglobal(self.L, c_str!("print"));
                    raw::lua_insert(self.L, 1);
                    if raw::lua_pcall(self.L, raw::lua_gettop(self.L) - 1, 0, 0) != raw::LUA_OK {
                        println!("error calling print({:s})", self.read::<&str>(-1).unwrap());
                    }
                }
            }
        }
        self.set_top(0); // clear stack
        println!("");
    }

    fn call(&mut self, nargs: c_int, nret: c_int) -> c_int {
        extern fn traceback(L: *mut raw::lua_State) -> c_int {
            // TODO: use State::new_raw_tmp(L)
            unsafe {
                let msg = raw::lua_tostring(L, 1);
                if msg.is_not_null() {
                    raw::luaL_traceback(L, L, msg, 1);
                } else if raw::lua_isnoneornil(L, 1) != 1 { // is there an error object?
                    if raw::luaL_callmeta(L, 1, c_str!("__tostring")) == 0 { // try its 'tostring' metamethod
                        raw::lua_pushstring(L, c_str!("(no error message)"));
                    }
                }
            }
            1
        }
        let base = self.top(); // function index
        self.push(traceback); // push traceback function
        self.move_to(base); // put it under chunk and args
        // TODO: treat SIGINT
        let status = unsafe { raw::lua_pcall(self.L, nargs, nret, 1) };
        self.remove(base); // remove traceback function
        status
    }

    pub fn eval<L: Loader>(&mut self, source: L) -> Result<(), LuarError> {
        match self.load(source) {
            Ok(()) => (),
            Err(e) => return Err(e),
        }
        match LuarError::from_lua(self.call(0, raw::LUA_MULTRET)) {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }
}

//impl<'a, V: ToLua> Index<&'a str, Option<V>> for State {
//    fn index<'a>(&'a self, index: & &str) -> &'a Option<V> {
//        let cstr = index.to_c_str();
//        unsafe { lua_getglobal(self.L, cstr.as_ptr()); }
//        &self.read::<V>(-1)
//    }
//}

impl<L: Loader> FnMut<(L,), Result<(), LuarError>> for State {
    /// Experimental shorthand for eval
    extern "rust-call" fn call_mut(&mut self, (source,): (L,)) -> Result<(), LuarError> {
        self.eval(source)
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if self.owned {
            unsafe { raw::lua_close(self.L); }
            self.L = ptr::null_mut();
        }
    }
}

// TODO: nicer name for these traits
pub trait ToLua {
    fn push_to(s: &mut State, val: Self);
    fn push_as_return(s: &mut State, val: Self) -> c_int { ToLua::push_to(s, val); 1 }
}
pub trait FromLua {
    fn read_from(s: &mut State, idx: c_int) -> Option<Self>;
}
pub trait Loader {
    fn load_to(s: &mut State, source: Self) -> c_int;
}

impl<'a> Loader for &'a Path {
    fn load_to(s: &mut State, source: &'a Path) -> c_int {
        // TODO: check whether luaL_loadbuffer or lua_load with a Reader is better
        let path = source.to_c_str();
        unsafe { raw::luaL_loadfile(s.L, path.as_ptr()) }
    }
}
// TODO: test this

impl<'a> Loader for &'a str {
    fn load_to(s: &mut State, source: &str) -> c_int {
        unsafe { raw::luaL_loadbuffer(s.L, source.as_ptr() as *const c_char, source.len() as size_t, c_str!("=stdin")) }
    }
}
#[test]
fn test_loader_str() {
    let mut state = State::new();

    state("return 5 + 6").unwrap();
    assert_eq!(Some(11i), state.pop());

    state("x = 2").unwrap();
    state("y = 5").unwrap();
    state("return math.pow(x, y)").unwrap();
    assert_eq!(Some(32i), state.pop());
}

impl Loader for String {
    fn load_to(s: &mut State, source: String) -> c_int {
        Loader::load_to(s, source.as_slice())
    }
}

#[test]
fn test_loader_string() {
    let mut state = State::new();

    state(format!("return {} + {}", 20i, 21i)).unwrap();
    assert_eq!(Some(41i), state.pop());
}

impl ToLua for () {
    fn push_to(s: &mut State, _: ()) {
        unsafe { raw::lua_pushnil(s.L) }
    }
    fn push_as_return(_s: &mut State, _: ()) -> c_int { 0 }
}

//impl<V: ToLua> ToLua for Option<V> {
//    fn push_to(s: &mut State, val: Option<V>) {
//        match val {
//            Some(p) => ToLua::push_to(s, p),
//            None => ToLua::push_to(s, ()),
//        }
//    }
//}

impl ToLua for bool {
    fn push_to(s: &mut State, val: bool) {
        unsafe { raw::lua_pushboolean(s.L, if val { 1 } else { 0 }) }
    }
}
impl FromLua for bool {
    fn read_from(s: &mut State, idx: c_int) -> Option<bool> {
        match unsafe { raw::lua_toboolean(s.L, idx) } {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}
#[test]
fn test_lua_bool() {
    let mut state = State::new();
    state.push(true);
    state.push(true);
    state.push(false);
    assert_eq!(state.pop(), Some(false));
    assert_eq!(state.pop(), Some(true));
    assert_eq!(state.pop(), Some(true));
    // caveat: nil is read as false:
    assert_eq!(state.pop(), Some(false));
}

//impl<V0: ToLua, V1: ToLua> ToLua for (V0, V1) {
//    fn push_to(s: &mut State, (v0, v1): (V0, V1)) {
//        ToLua::push_to(s, v0);
//        ToLua::push_to(s, v1);
//    }
//    fn push_as_return(s: &mut State, val: (V0, V1)) -> c_int { ToLua::push_to(s, val); 2 }
//}
//impl<V0: FromLua, V1: FromLua> FromLua for (V0, V1) {
//    fn read_from(s: &mut State, idx: c_int) -> Option<(V0, V1)> {
//        let mut i = 0;
//        let v1 = match FromLua::read_from(s, idx + i) { Some(v) => v, None => return None }; i += 1;
//        let v0 = match FromLua::read_from(s, idx + i) { Some(v) => v, None => return None };
//        Some((v0, v1))
//    }
//}
//#[test]
//fn test_tuple_push_to() {
//    let mut state = State::new();
//    state.push((1i, "2"));
//    let (a, b): (int, &str) = state.pop().unwrap();
//    assert_eq!(1i, a);
//    assert_eq!("2", b);
//}

macro_rules! impl_number(
    ($t:ty|$lua_type:ident $lua_push:ident|$lua_to:ident) => (
        impl ToLua for $t {
            fn push_to(s: &mut State, val: $t) {
                unsafe { raw::$lua_push(s.L, val as raw::$lua_type) }
            }
        }
        impl FromLua for $t {
            fn read_from(s: &mut State, idx: c_int) -> Option<$t> {
                let mut isnum: c_int = 0;
                let num = unsafe { raw::$lua_to(s.L, idx, &mut isnum) };
                match isnum {
                    1 => Some(num as $t),
                    _ => None,
                }
            }
        }
    );
)
// floats
impl_number!(f32|lua_Number lua_pushnumber|lua_tonumberx)
impl_number!(f64|lua_Number lua_pushnumber|lua_tonumberx)
// ints
impl_number!(int|lua_Integer lua_pushinteger|lua_tointegerx)
impl_number!(i8 |lua_Integer lua_pushinteger|lua_tointegerx)
impl_number!(i16|lua_Integer lua_pushinteger|lua_tointegerx)
impl_number!(i32|lua_Integer lua_pushinteger|lua_tointegerx)
impl_number!(i64|lua_Integer lua_pushinteger|lua_tointegerx)
// uints
impl_number!(uint|lua_Unsigned lua_pushunsigned|lua_tounsignedx)
impl_number!(u8  |lua_Unsigned lua_pushunsigned|lua_tounsignedx)
impl_number!(u16 |lua_Unsigned lua_pushunsigned|lua_tounsignedx)
impl_number!(u32 |lua_Unsigned lua_pushunsigned|lua_tounsignedx)
impl_number!(u64 |lua_Unsigned lua_pushunsigned|lua_tounsignedx)

impl<'a> ToLua for &'a str {
    fn push_to(s: &mut State, val: &str) {
        unsafe { raw::lua_pushlstring(s.L, val.as_ptr() as *const c_char, val.len() as size_t); }
    }
}
impl<'a> FromLua for &'a str {
    fn read_from<'a>(s: &mut State, idx: c_int) -> Option<&'a str> {
        let mut len: size_t = 0;
        let ptr = unsafe { raw::lua_tolstring(s.L, idx, &mut len) };
        let slice = unsafe { std::mem::transmute(std::raw::Slice { data: ptr, len: len as uint}) };
        from_utf8(slice)
    }
}

impl ToLua for String {
    fn push_to(s: &mut State, val: String) {
        ToLua::push_to(s, val.as_slice())
    }
}
impl FromLua for String {
    fn read_from(s: &mut State, idx: c_int) -> Option<String> {
        match FromLua::read_from(s, idx) {
            Some(s) => Some(String::from_str(s)),
            None => None,
        }
    }
}

// handy implementation for copyable types
impl<'a, V: ToLua + Copy> ToLua for &'a V {
    fn push_to(s: &mut State, val: &V) {
        ToLua::push_to(s, *val);
    }
}

// this works because the struct State is a *mut lua_State so they can be transmuted
type CFunction = extern fn(L: *mut raw::lua_State) -> c_int;
//type lua_CFunction = Option<extern fn(L: *mut raw::lua_State) -> c_int>;
//type CFunction = unsafe extern fn(*mut raw::lua_State) -> c_int;
impl ToLua for CFunction {
    fn push_to(s: &mut State, val: CFunction) {
        unsafe { raw::lua_pushcclosure(s.L, Some(val), 0) }
    }
}
impl FromLua for CFunction {
    fn read_from(s: &mut State, idx: c_int) -> Option<CFunction> {
        unsafe { raw::lua_tocfunction(s.L, idx) }
    }
}

macro_rules! impl_tolua_for_cl(
    ($($A:ident)*) => (
        #[inline]
        impl<'a, $($A: FromLua,)* R: ToLua> ToLua for |$($A),*|: 'a -> R {
            fn push_to(s: &mut State, val: |$($A),*| -> R) {
                #[inline(never)]
                extern fn dispatch<$($A: FromLua,)* R: ToLua>(L: *mut raw::lua_State) -> c_int {
                    let mut s = State::new_raw_tmp(L);
                    let fun: |$($A),*| -> R = unsafe { std::mem::transmute(
                            (raw::lua_touserdata(s.L, raw::lua_upvalueindex(1)), raw::lua_touserdata(s.L, raw::lua_upvalueindex(2)))
                    ) };
                    // prefix it with _ to supress warnings for when $($A)* is empty
                    let mut _i = 0i;
                    let r = fun($(
                        // it would be awesome if a macro had a counter
                        match s.read::<$A>({_i += 1; _i}) { Some(a) => a, None => return 0 }
                    ),*);
                    ToLua::push_as_return(&mut s, r)
                }
                unsafe {
                    let (low, high): (*mut c_void, *mut c_void) = std::mem::transmute(val);
                    raw::lua_pushlightuserdata(s.L, low);
                    raw::lua_pushlightuserdata(s.L, high);
                    raw::lua_pushcclosure(s.L, Some(dispatch::<$($A,)* R>), 2);
                }
            }
        }
    );
)
// recursive impl_tolua_for_cl!
macro_rules! rec_impl_tolua_for_cl(
    () => (impl_tolua_for_cl!());
    ($i:ident $($i_rest:ident)*) => (impl_tolua_for_cl!($i $($i_rest)*) rec_impl_tolua_for_cl!($($i_rest)*));
)
rec_impl_tolua_for_cl!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15)

macro_rules! impl_tolua_for_fn(
    ($($A:ident)*) => (
        #[inline]
        impl<'a, $($A: FromLua,)* R: ToLua> ToLua for fn($($A),*) -> R {
            fn push_to(s: &mut State, val: fn($($A),*) -> R) {
                #[inline(never)]
                extern fn dispatch<$($A: FromLua,)* R: ToLua>(L: *mut raw::lua_State) -> c_int {
                    let mut s = State::new_raw_tmp(L);
                    let fun: fn($($A),*) -> R = unsafe { std::mem::transmute(
                            raw::lua_touserdata(s.L, raw::lua_upvalueindex(1))
                    ) };
                    // prefix it with _ to supress warnings for when $($A)* is empty
                    let mut _i = 0i;
                    let r = fun($(
                        // it would be awesome if a macro had a counter
                        match s.read::<$A>({_i += 1; _i}) { Some(a) => a, None => return 0 }
                    ),*);
                    ToLua::push_as_return(&mut s, r)
                }
                unsafe {
                    raw::lua_pushlightuserdata(s.L, std::mem::transmute(val));
                    raw::lua_pushcclosure(s.L, Some(dispatch::<$($A,)* R>), 1);
                }
            }
        }
    );
)
// recursive impl_tolua_for_cl!
macro_rules! rec_impl_tolua_for_fn(
    () => (impl_tolua_for_fn!());
    ($i:ident $($i_rest:ident)*) => (impl_tolua_for_fn!($i $($i_rest)*) rec_impl_tolua_for_fn!($($i_rest)*));
)
rec_impl_tolua_for_fn!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15)

#[test]
fn test_fn0_push_to() {
    let mut state = State::new();
    fn my_fun() -> int { 456 }
    state.insert("my_fun", my_fun);
    state.eval("x = my_fun()").unwrap();
    assert_eq!(Some(456i), state.get("x"));
}

#[test]
fn test_fn1_push_to() {
    let mut state = State::new();
    fn my_fun(x: int) -> int { x + 4 }
    state.insert("my_fun", my_fun);
    state.eval("x = my_fun(7)").unwrap();
    assert_eq!(Some(11i), state.get("x"));
}

#[test]
fn test_fn2_push_to() {
    let mut state = State::new();
    fn my_fun(x: int, y: int) -> int { x * y + 6 }
    state.insert("my_fun", my_fun);
    state.eval("x = my_fun(5, 7)").unwrap();
    assert_eq!(Some(41i), state.get("x"));
}

#[test]
fn test_closure_push_to() {
    let mut state = State::new();
    let mut z = 4i;
    state.insert("my_fun", || { z = 5 } );
    state.eval("my_fun()").unwrap();
    assert_eq!(5i, z);
}

#[test]
fn test_closure0_push_to() {
    let mut state = State::new();
    state.insert("my_fun", || 6i );
    state.eval("x = my_fun()").unwrap();
    assert_eq!(Some(6i), state.get("x"));
}

#[test]
fn test_closure1_push_to() {
    let mut state = State::new();
    let z = 4i;
    state.insert("my_fun", |x: int| x + z );
    state.eval("x = my_fun(7)").unwrap();
    assert_eq!(Some(11i), state.get("x"));
}

#[test]
fn test_closure2_push_to() {
    let mut state = State::new();
    let mut z = 4;
    state.insert("my_fun", |x: int, y: int| x * y + z);
    z = 6;
    state.eval("x = my_fun(5, 7)").unwrap();
    assert_eq!(Some(5i * 7 + z), state.get("x"));
}

#[test]
fn test_closure_push_to_longer_life() {
    let z = 5i;
    let fun = |_: int, _: int| z;
    {
        let mut state = State::new();
        state.insert("my_fun", fun);
        state.eval("x = my_fun(1, 2)").unwrap();
        assert_eq!(Some(5i), state.get("x"));
    }
}

// The following should not be allowed, it may be hard to impose that though
#[test]
fn test_closure_push_to_shorter_life() {
    let mut state = State::new();
    {
        let z = 5i;
        let fun = |_: int, _: int| z;
        state.insert("my_fun", fun);
    }
    state.eval("x = my_fun(1, 2)").unwrap();
    assert_eq!(Some(5i), state.get("x"));
}

impl<K: ToLua + Hash + Eq + Copy, V: ToLua + Copy> ToLua for HashMap<K, V> {
    fn push_to(s: &mut State, val: HashMap<K, V>) {
        unsafe { raw::lua_createtable(s.L, val.len() as c_int, 0) };
        for (key, value) in val.iter() {
            ToLua::push_to(s, key);
            ToLua::push_to(s, value);
            unsafe { raw::lua_rawset(s.L, -3) };
        }
    }
}
// TODO: impl FromLua for HashMap, in order to read tables

#[deriving(Show)]
pub enum LuarError {
    UnkownError,
    CallError,
    GcMetamethodError,
    LoadFileError,
    RuntimeError,
    SyntaxError,
    MemoryAllocationError,
    MessageHandlerError,
    NotANumberError,
    SuspendedError,
}

impl LuarError {
    /// Returns None in case of success, otherwise returns the proper enum
    fn from_lua(status: c_int) -> Option<LuarError> {
        match status {
            raw::LUA_OK        => None,
            raw::LUA_YIELD     => Some(SuspendedError),
            raw::LUA_ERRRUN    => Some(RuntimeError),
            raw::LUA_ERRSYNTAX => Some(SyntaxError),
            raw::LUA_ERRMEM    => Some(MemoryAllocationError),
            raw::LUA_ERRGCMM   => Some(GcMetamethodError),
            raw::LUA_ERRERR    => Some(MessageHandlerError),
            raw::LUA_ERRFILE   => Some(LoadFileError),
            _             => Some(UnkownError),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn state_get_or() {
        let mut state = ::State::new();

        state.insert("x", 5i);
        assert_eq!(state.get_or("x", 6i), 5i);

        state.insert("y", ());
        assert_eq!(state.get_or("y", 7i), 7i);

        state.insert("z", "42");
        assert_eq!(state.get_or("z", 66i), 42i);

        state.insert("w", "foo");
        assert_eq!(state.get_or("w", 55i), 55i);

        assert_eq!(state.get_or("n", 77i), 77i);
    }
}
