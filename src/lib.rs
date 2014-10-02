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

use std::hash::Hash;
use std::collections::HashMap;
use std::io::{stdio, IoResult};
//use std::ops::Index;
use std::ptr;
use std::str::from_utf8;
use libc::{c_void, c_char, c_int, c_uint, c_double, size_t, ptrdiff_t};

macro_rules! c_str(
    ($t: expr) => ({let t = $t.to_c_str(); t.as_ptr()});
)

// lua version
static LUA_VERSION_NUM: c_int = 502;

// lua constants
static LUA_MINSTACK:    c_int = 20;
//static LUAI_BITSINT: c_int = 32;
//static LUAI_BITSINT: c_int = 16;
//static LUAI_MAXSTACK: c_int;
//if LUAI_BITSINT >= 32 {
//    static LUAI_MAXSTACK: c_int = 1_000_000;
//} else {
//    static LUAI_MAXSTACK: c_int = 15_000;
//}
//static LUAI_MAXSTACK: c_int = if LUAI_BITSINT >= 32 { 1_000_000 } else { 15000 };
static LUAI_MAXSTACK: c_int = 1_000_000;
static LUAI_FIRSTPSEUDOIDX: c_int = -LUAI_MAXSTACK - 1000;
static LUA_REGISTRYINDEX: c_int = LUAI_FIRSTPSEUDOIDX;

// lua prompt
static LUA_PROMPT:  &'static str = "> ";
static LUA_PROMPT2: &'static str = ">> ";

// lua error results
static LUA_MULTRET:   c_int = -1;
static LUA_OK:        c_int = 0;
static LUA_YIELD:     c_int = 1;
static LUA_ERRRUN:    c_int = 2;
static LUA_ERRSYNTAX: c_int = 3;
static LUA_ERRMEM:    c_int = 4;
static LUA_ERRGCMM:   c_int = 5;
static LUA_ERRERR:    c_int = 6;
static LUA_ERRFILE:   c_int = 7;

// lua gc commands
static LUA_GCSTOP:        c_int =  0;
static LUA_GCRESTART:     c_int =  1;
static LUA_GCCOLLECT:     c_int =  2;
//static LUA_GCCOUNT:       c_int =  3;
//static LUA_GCCOUNTB:      c_int =  4;
//static LUA_GCSTEP:        c_int =  5;
//static LUA_GCSETPAUSE:    c_int =  6;
//static LUA_GCSETSTEPMUL:  c_int =  7;
//static LUA_GCSETMAJORINC: c_int =  8;
//static LUA_GCISRUNNING:   c_int =  9;
//static LUA_GCGEN:         c_int = 10;
//static LUA_GCINC:         c_int = 11;

// lua types
//static LUA_TNONE:          c_int = -1;
static LUA_TNIL:           c_int = 0;
//static LUA_TBOOLEAN:       c_int = 1;
//static LUA_TLIGHTUSERDATA: c_int = 2;
//static LUA_TNUMBER:        c_int = 3;
//static LUA_TSTRING:        c_int = 4;
//static LUA_TTABLE:         c_int = 5;
//static LUA_TFUNCTION:      c_int = 6;
//static LUA_TUSERDATA:      c_int = 7;
//static LUA_TTHREAD:        c_int = 8;

type lua_State = c_void;
type lua_Alloc = Option<extern fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void>;
type lua_Number = c_double;
type lua_Integer = ptrdiff_t;
type lua_Unsigned = c_uint;
type lua_CFunction = Option<extern fn(L: *mut lua_State) -> c_int>;

#[link(name = "lua", kind = "static")]
extern {
    fn luaL_newstate() -> *mut lua_State;
    fn luaL_openlibs(L: *mut lua_State);
    //fn luaL_loadbuffer(L: *mut lua_State, buff: *const c_char, sz: size_t, name: *const c_char) -> c_int;
    //fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);
    fn lua_pcallk(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: c_int, k: lua_CFunction) -> c_int;
    fn lua_settop(L: *mut lua_State, idx: c_int);
    fn lua_close(L: *mut lua_State);
    fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    fn luaL_loadfilex(L: *mut lua_State, filename: *const c_char, mode: *const c_char) -> c_int;
    fn lua_rawset(L: *mut lua_State, idx: c_int);
    //fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    //fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
    fn lua_setglobal(L: *mut lua_State, var: *const c_char);
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number);
    fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
    fn lua_getglobal(L: *mut lua_State, var: *const c_char);
    fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;
    fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
    fn lua_tointegerx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Integer;
    fn lua_tounsignedx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Unsigned;
    fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> lua_CFunction;
    fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);
    fn lua_pushboolean(L: *mut lua_State, b: c_int);
    fn lua_pushcclosure(L: *mut lua_State, fun: lua_CFunction, n: c_int);
    fn lua_pushlstring(L: *mut lua_State, s: *const c_char, l: size_t) -> *const c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const c_char) -> *const c_char;
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    //fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, ...) -> *const c_char;
    fn lua_remove(L: *mut lua_State, idx: c_int);
    fn lua_insert(L: *mut lua_State, idx: c_int);
    fn lua_concat(L: *mut lua_State, n: c_int);
    fn luaL_loadbufferx(L: *mut lua_State, buff: *const c_char, sz: size_t, name: *const c_char, mode: *const c_char) -> c_int;
    fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const c_char, level: c_int);
    fn lua_gettop(L: *mut lua_State) -> c_int;
    fn luaL_checkstack(L: *mut lua_State, sz: c_int, msg: *const c_char);
    fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    fn luaL_callmeta(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int;
}

//#[inline]
//unsafe fn lua_pop(L: *mut lua_State, n: c_int) { lua_settop(L, -n - 1) }
//#[inline]
//unsafe fn lua_newtable(L: *mut lua_State) { lua_createtable(L, 0, 0) }
#[inline]
unsafe fn luaL_loadfile(L: *mut lua_State, f: *const c_char) -> c_int { luaL_loadfilex(L, f, ptr::null()) }
#[inline]
unsafe fn lua_pcall(L: *mut lua_State, n: c_int, r: c_int, f: c_int) -> c_int { lua_pcallk(L, n, r, f, 0, None) }
#[inline]
unsafe fn luaL_checkversion(L: *mut lua_State) { luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number) }
#[inline]
unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char { lua_tolstring(L, i, ptr::null_mut()) }
//#[inline]
//unsafe fn lua_pushliteral(L: *mut lua_State, s: *const c_char) -> *const c_char { lua_pushlstring(L, s, (size_of??? / size_of<c_char>()) - 1); }
#[inline]
unsafe fn luaL_loadbuffer(L: *mut lua_State, s: *const c_char, sz: size_t, n: *const c_char) -> c_int { luaL_loadbufferx(L, s, sz, n, ptr::null()) }
#[inline]
unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> c_int { (lua_type(L, n) <= 0) as c_int }
#[inline]
unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> c_int { (lua_type(L, n) == LUA_TNIL) as c_int }
//#[inline]
//unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) { lua_pushcclosure(L, f, 0) }
//#[inline]
//unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) { lua_pushcfunction(L, f); lua_setglobal(L, n) }
#[inline]
unsafe fn lua_upvalueindex(i: c_int) -> c_int { LUA_REGISTRYINDEX - i }

/// Main type of the API, this is an abstraction over Lua's `lua_State`.
// TODO: consider using std::kinds::marker (http://doc.rust-lang.org/std/kinds/marker/) for
// properly marking types
pub struct State {
    L: *mut lua_State,
    owned: bool,
}

impl State {
    #[inline]
    pub fn new() -> State {
        match State::new_checked(true) {
            Ok(s) => s,
            Err(e) => fail!(e),
        }
    }

    #[inline]
    pub fn new_checked(with_libs: bool) -> Result<State, LuarError> {
        unsafe {
            match luaL_newstate() {
                L if L.is_not_null() => {
                    let mut state = State{ L: L, owned: true };
                    if with_libs { state.open_libs(); }
                    Ok(state)
                },
                _ => Err(MemoryAllocationError),
            }
        }
    }

    #[inline]
    fn new_raw_tmp(L: *mut lua_State) -> State {
        State{ L: L, owned: false }
    }

    #[inline]
    fn open_libs(&mut self) {
        unsafe {
            luaL_checkversion(self.L);
            // stop gc during initialization
            lua_gc(self.L, LUA_GCSTOP, 0);
            luaL_openlibs(self.L);
            lua_gc(self.L, LUA_GCRESTART, 0);
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
        unsafe { lua_getglobal(self.L, c_str!(var)); }
        self.read(-1)
    }

    #[inline]
    pub fn insert<V: ToLua>(&mut self, name: &str, val: V) {
        self.push(val);
        unsafe { lua_setglobal(self.L, c_str!(name)); }
    }

    #[inline]
    pub fn get<V: FromLua>(&mut self, name: &str) -> Option<V> {
        unsafe { lua_getglobal(self.L, c_str!(name)); }
        self.pop()
    }

    #[inline]
    pub fn get_or<V: FromLua>(&mut self, name: &str, def: V) -> V {
        self.get(name).unwrap_or(def)
    }

    #[inline]
    fn move_to(&mut self, index: int) {
        unsafe { lua_insert(self.L, index as c_int) }
    }

    #[inline]
    fn remove(&mut self, idx: int) {
        unsafe { lua_remove(self.L, idx as c_int) }
    }

    #[inline]
    fn top(&mut self) -> int {
        unsafe { lua_gettop(self.L) as int }
    }

    #[inline]
    fn set_top(&mut self, index: int) {
        unsafe { lua_settop(self.L, index as c_int) }
    }

    #[inline]
    fn remove_top(&mut self) {
        unsafe { lua_settop(self.L, -2) }
    }

    fn load_slice(&mut self, slice: &str) -> c_int {
        let cstr = slice.to_c_str();
        unsafe { luaL_loadbuffer(self.L, cstr.as_ptr(), cstr.len() as size_t, c_str!("=stdin")) }
    }

    /// Will print the prompt from lua vars `_PROMPT` or `_PROMPT2` if they're defined
    /// or use the static versions. Read a single line, and push it into lua.
    fn push_line(&mut self, first: bool) -> IoResult<()> {

        // print the prompt if defined in lua or the default
        match self.get_global::<&str>(if first { "_PROMPT" } else { "_PROMPT2" }) {
            Some(s) => print!("{:s}", s),
            None    => print!("{:s}", if first { LUA_PROMPT } else { LUA_PROMPT2 }),
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
                LUA_ERRSYNTAX => {
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
            unsafe { lua_concat(self.L, 3); }
        }

        // TODO: save line in history
        unsafe { lua_remove(self.L, 1); } // remove line
        (true, status)
    }

    pub fn shell(&mut self) {
        loop {
            let (ok, mut status) = self.load_line();
            if !ok { break; }

            if status == LUA_OK {
                status = self.call(0, LUA_MULTRET);
            }
            // report
            if status != LUA_OK && unsafe { lua_isnil(self.L, -1) != 1 } {
                let msg: &str = match self.read(-1) {
                    Some(s) => s,
                    None => "(error object is not a string)",
                };
                println!("{}", msg);
                self.remove_top();
                // force a complete garbage collection in case of errors
                unsafe { lua_gc(self.L, LUA_GCCOLLECT, 0); }
            }
            if status == LUA_OK && self.top() > 0 { // any result to print
                unsafe {
                    luaL_checkstack(self.L, LUA_MINSTACK, c_str!("too many results to print"));
                    lua_getglobal(self.L, c_str!("print"));
                    lua_insert(self.L, 1);
                    if lua_pcall(self.L, lua_gettop(self.L) - 1, 0, 0) != LUA_OK {
                        println!("error calling print({:s})", self.read::<&str>(-1).unwrap());
                    }
                }
            }
        }
        self.set_top(0); // clear stack
        println!("");
    }

    fn call(&mut self, nargs: c_int, nret: c_int) -> c_int {
        extern fn traceback(L: *mut lua_State) -> c_int {
            // TODO: use State::new_raw_tmp(L)
            unsafe {
                let msg = lua_tostring(L, 1);
                if msg.is_not_null() {
                    luaL_traceback(L, L, msg, 1);
                } else if lua_isnoneornil(L, 1) != 1 { // is there an error object?
                    if luaL_callmeta(L, 1, c_str!("__tostring")) == 0 { // try its 'tostring' metamethod
                        lua_pushstring(L, c_str!("(no error message)"));
                    }
                }
            }
            1
        }
        let base = self.top(); // function index
        self.push(traceback); // push traceback function
        self.move_to(base); // put it under chunk and args
        // TODO: treat SIGINT
        let status = unsafe { lua_pcall(self.L, nargs, nret, 1) };
        self.remove(base); // remove traceback function
        status
    }

    pub fn eval<L: Loader>(&mut self, source: L) -> Result<(), LuarError> {
        match self.load(source) {
            Ok(()) => (),
            Err(e) => return Err(e),
        }
        match LuarError::from_lua(self.call(0, LUA_MULTRET)) {
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
    #[rust_call_abi_hack]
    fn call_mut(&mut self, (source,): (L,)) -> Result<(), LuarError> {
        self.eval(source)
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if self.owned {
            unsafe { lua_close(self.L); }
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
        unsafe { luaL_loadfile(s.L, path.as_ptr()) }
    }
}
// TODO: test this

impl<'a> Loader for &'a str {
    fn load_to(s: &mut State, source: &str) -> c_int {
        unsafe { luaL_loadbuffer(s.L, source.as_ptr() as *const c_char, source.len() as size_t, c_str!("=stdin")) }
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
        unsafe { lua_pushnil(s.L) }
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
        unsafe { lua_pushboolean(s.L, if val { 1 } else { 0 }) }
    }
}
impl FromLua for bool {
    fn read_from(s: &mut State, idx: c_int) -> Option<bool> {
        match unsafe { lua_toboolean(s.L, idx) } {
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

impl ToLua for f64 {
    fn push_to(s: &mut State, val: f64) {
        unsafe { lua_pushnumber(s.L, val as lua_Number) }
    }
}
impl FromLua for f64 {
    fn read_from(s: &mut State, idx: c_int) -> Option<f64> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tonumberx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as f64),
            _ => None,
        }
    }
}

impl ToLua for f32 {
    fn push_to(s: &mut State, val: f32) {
        unsafe { lua_pushnumber(s.L, val as lua_Number) }
    }
}
impl FromLua for f32 {
    fn read_from(s: &mut State, idx: c_int) -> Option<f32> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tonumberx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as f32),
            _ => None,
        }
    }
}

impl ToLua for int {
    fn push_to(s: &mut State, val: int) {
        unsafe { lua_pushinteger(s.L, val as lua_Integer) }
    }
}
impl FromLua for int {
    fn read_from(s: &mut State, idx: c_int) -> Option<int> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tointegerx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as int),
            _ => None,
        }
    }
}

impl ToLua for i64 {
    fn push_to(s: &mut State, val: i64) {
        unsafe { lua_pushinteger(s.L, val as lua_Integer) }
    }
}
impl FromLua for i64 {
    fn read_from(s: &mut State, idx: c_int) -> Option<i64> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tointegerx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as i64),
            _ => None,
        }
    }
}

impl ToLua for i32 {
    fn push_to(s: &mut State, val: i32) {
        unsafe { lua_pushinteger(s.L, val as lua_Integer) }
    }
}
impl FromLua for i32 {
    fn read_from(s: &mut State, idx: c_int) -> Option<i32> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tointegerx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as i32),
            _ => None,
        }
    }
}

impl ToLua for uint {
    fn push_to(s: &mut State, val: uint) {
        unsafe { lua_pushunsigned(s.L, val as lua_Unsigned) }
    }
}
impl FromLua for uint {
    fn read_from(s: &mut State, idx: c_int) -> Option<uint> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tounsignedx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as uint),
            _ => None,
        }
    }
}

impl ToLua for u32 {
    fn push_to(s: &mut State, val: u32) {
        unsafe { lua_pushunsigned(s.L, val as lua_Unsigned) }
    }
}
impl FromLua for u32 {
    fn read_from(s: &mut State, idx: c_int) -> Option<u32> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tounsignedx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as u32),
            _ => None,
        }
    }
}

impl ToLua for u64 {
    fn push_to(s: &mut State, val: u64) {
        unsafe { lua_pushunsigned(s.L, val as lua_Unsigned) }
    }
}
impl FromLua for u64 {
    fn read_from(s: &mut State, idx: c_int) -> Option<u64> {
        let mut isnum: c_int = 0;
        let num = unsafe { lua_tounsignedx(s.L, idx, &mut isnum) };
        match isnum {
            1 => Some(num as u64),
            _ => None,
        }
    }
}

impl<'a> ToLua for &'a str {
    fn push_to(s: &mut State, val: &str) {
        unsafe { lua_pushlstring(s.L, val.as_ptr() as *const c_char, val.len() as size_t); }
    }
}
impl<'a> FromLua for &'a str {
    fn read_from<'a>(s: &mut State, idx: c_int) -> Option<&'a str> {
        let mut len: size_t = 0;
        let ptr = unsafe { lua_tolstring(s.L, idx, &mut len) };
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
type CFunction = extern fn(L: *mut lua_State) -> c_int;
//type lua_CFunction = Option<extern fn(L: *mut lua_State) -> c_int>;
//type CFunction = unsafe extern fn(*mut lua_State) -> c_int;
impl ToLua for CFunction {
    fn push_to(s: &mut State, val: CFunction) {
        unsafe { lua_pushcclosure(s.L, Some(val), 0) }
    }
}
impl FromLua for CFunction {
    fn read_from(s: &mut State, idx: c_int) -> Option<CFunction> {
        unsafe { lua_tocfunction(s.L, idx) }
    }
}

struct Pair { low: *mut c_void, high: *mut c_void }
macro_rules! push_cl_dispatcher(
    ($L:expr, $disp:expr, $fun:expr) => (
        {
            let pair: Pair = std::mem::transmute($fun);
            lua_pushlightuserdata($L, pair.low);
            lua_pushlightuserdata($L, pair.high);
            lua_pushcclosure($L, Some($disp), 2);
        }
    );
)
macro_rules! get_dispatched_cl(
    ($L:expr) => (
        std::mem::transmute(Pair {
            low:  lua_touserdata($L, lua_upvalueindex(1)),
            high: lua_touserdata($L, lua_upvalueindex(2)),
        })
    );
)
macro_rules! impl_tolua_for_cl(
    () => (
        impl<'a, R: ToLua> ToLua for ||: 'a -> R {
            fn push_to(s: &mut State, val: || -> R) {
                extern fn dispatch<R: ToLua>(L: *mut lua_State) -> c_int {
                    let mut s = State::new_raw_tmp(L);
                    let fun: || -> R = unsafe { get_dispatched_cl!(s.L) };
                    ToLua::push_as_return(&mut s, fun())
                }
                unsafe { push_cl_dispatcher!(s.L, dispatch::<R>, val) }
            }
        }
    );
    ($($ArgType:ident)+) => (
        impl<'a, $($ArgType: FromLua),*, R: ToLua> ToLua for |$($ArgType),*|: 'a -> R {
            fn push_to(s: &mut State, val: |$($ArgType),*| -> R) {
                extern fn dispatch<$($ArgType: FromLua),*, R: ToLua>(L: *mut lua_State) -> c_int {
                    let mut s = State::new_raw_tmp(L);
                    let fun: |$($ArgType),*| -> R = unsafe { get_dispatched_cl!(s.L) };
                    let mut i = 0i;
                    let r = fun($(
                        // it would be awesome if a macro had a counter
                        match s.read::<$ArgType>({i += 1; i}) { Some(a) => a, None => return 0 }
                    ),*);
                    ToLua::push_as_return(&mut s, r)
                }
                unsafe { push_cl_dispatcher!(s.L, dispatch::<$($ArgType,)* R>, val) }
            }
        }
    );
)
macro_rules! mult_impl_tolua_for_cl(
    () => (impl_tolua_for_cl!());
    ($i:ident $($i_rest:ident)*) => (
        impl_tolua_for_cl!($i $($i_rest)*)
        mult_impl_tolua_for_cl!($($i_rest)*)
    );
)
mult_impl_tolua_for_cl!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15)

macro_rules! push_fn_dispatcher(
    ($L:expr, $disp:expr, $fun:expr) => (
        {
            lua_pushlightuserdata($L, std::mem::transmute($fun));
            lua_pushcclosure($L, Some($disp), 1);
        }
    );
)
macro_rules! get_dispatched_fn(
    ($L:expr) => (
        std::mem::transmute(lua_touserdata($L, lua_upvalueindex(1)))
    );
)
macro_rules! impl_tolua_for_fn(
    () => (
        impl<'a, R: ToLua> ToLua for fn() -> R {
            fn push_to(s: &mut State, val: fn() -> R) {
                extern fn dispatch<R: ToLua>(L: *mut lua_State) -> c_int {
                    let mut s = State::new_raw_tmp(L);
                    let fun: fn() -> R = unsafe { get_dispatched_fn!(s.L) };
                    ToLua::push_as_return(&mut s, fun())
                }
                unsafe { push_fn_dispatcher!(s.L, dispatch::<R>, val) }
            }
        }
    );
    ($($ArgType:ident)+) => (
        impl<'a, $($ArgType: FromLua),*, R: ToLua> ToLua for fn($($ArgType),*) -> R {
            fn push_to(s: &mut State, val: fn($($ArgType),*) -> R) {
                extern fn dispatch<$($ArgType: FromLua),*, R: ToLua>(L: *mut lua_State) -> c_int {
                    let mut s = State::new_raw_tmp(L);
                    let fun: fn($($ArgType),*) -> R = unsafe { get_dispatched_fn!(s.L) };
                    let mut i = 0i;
                    let r = fun($(
                        // it would be awesome if a macro had a counter
                        match s.read::<$ArgType>({i += 1; i}) { Some(a) => a, None => return 0 }
                    ),*);
                    ToLua::push_as_return(&mut s, r)
                }
                unsafe { push_fn_dispatcher!(s.L, dispatch::<$($ArgType,)* R>, val) }
            }
        }
    );
)
macro_rules! mult_impl_tolua_for_fn(
    () => (impl_tolua_for_fn!());
    ($i:ident $($i_rest:ident)*) => (
        impl_tolua_for_fn!($i $($i_rest)*)
        mult_impl_tolua_for_fn!($($i_rest)*)
    );
)
mult_impl_tolua_for_fn!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15)

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
        unsafe { lua_createtable(s.L, val.len() as c_int, 0) };
        for (key, value) in val.iter() {
            ToLua::push_to(s, key);
            ToLua::push_to(s, value);
            unsafe { lua_rawset(s.L, -3) };
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
            LUA_OK        => None,
            LUA_YIELD     => Some(SuspendedError),
            LUA_ERRRUN    => Some(RuntimeError),
            LUA_ERRSYNTAX => Some(SyntaxError),
            LUA_ERRMEM    => Some(MemoryAllocationError),
            LUA_ERRGCMM   => Some(GcMetamethodError),
            LUA_ERRERR    => Some(MessageHandlerError),
            LUA_ERRFILE   => Some(LoadFileError),
            _             => Some(UnkownError),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn _state_get_or() {
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
