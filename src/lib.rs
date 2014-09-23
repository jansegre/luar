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
static LUA_MINSTACK:    c_int = 20;

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
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);
    fn lua_pushboolean(L: *mut lua_State, b: c_int);
    fn lua_pushcclosure(L: *mut lua_State, fun: lua_CFunction, n: c_int);
    fn lua_pushlstring(L: *mut lua_State, s: *const c_char, l: size_t) -> *const c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const c_char) -> *const c_char;
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

/// Main type of the API, this is an abstraction over Lua's `lua_State`.
pub struct State {
    // There is a huge advantage on having solely this field:
    // being able to transmute back and forth into it.
    L: *mut lua_State,
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
                    let mut state = State{ L: L };
                    if with_libs { state.open_libs(); }
                    Ok(state)
                },
                _ => Err(MemoryAllocationError),
            }
        }
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
        match LuarError::from_lua(unsafe { Loader::load_to(self.L, source) }) {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    /// Shorthand for load followed by call_mut
    #[inline]
    pub fn eval<L: Loader>(&mut self, source: L) -> Result<(), LuarError> {
        match self.load(source) {
            Ok(()) => match LuarError::from_lua(self()) {
                None => Ok(()),
                Some(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    // should this be made public?
    #[inline]
    fn push<V: ToLua>(&mut self, val: V) {
        unsafe { ToLua::push_to(self.L, val) }
    }

    #[inline]
    fn get<V: FromLua>(&mut self, idx: int) -> Option<V> {
        unsafe { FromLua::get_from(self.L, idx as c_int) }
    }

    #[inline]
    fn get_global<V: FromLua>(&mut self, var: &str) -> Option<V> {
        unsafe { lua_getglobal(self.L, c_str!(var)); }
        self.get(-1)
    }

    #[inline]
    pub fn insert<V: ToLua>(&mut self, name: &str, val: V) {
        self.push(val);
        unsafe { lua_setglobal(self.L, c_str!(name)); }
    }

    #[inline]
    fn move(&mut self, index: int) {
        unsafe { lua_insert(self.L, index as c_int) }
    }

    #[inline]
    pub fn pop<V: FromLua>(&mut self) -> Option<V> {
        let top = self.top();
        let opt_val = self.get(top);
        self.remove_top();
        opt_val
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
            let line: &str = self.get(1).unwrap();
            status = self.load_slice(line);
            match status {
                LUA_ERRSYNTAX => {
                    match self.get::<&str>(-1) {
                        Some(s) if s.ends_with("<eof>") => self.remove_top(),
                        _ => break,
                    }
                },
                _ => break,
            }

            match self.push_line(false) { Ok(()) => (), Err(_) => return (false, status) }

            // add a new line between the two lines:
            self.push("\n");
            self.move(-2);
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
                status = self();
            }
            // report
            if status != LUA_OK && unsafe { lua_isnil(self.L, -1) != 1 } {
                let msg: &str = match self.get(-1) {
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
                        println!("error calling print({:s})", self.get::<&str>(-1).unwrap());
                    }
                }
            }
        }
        self.set_top(0); // clear stack
        println!("");
    }
}

//impl<'a, V: ToLua> Index<&'a str, Option<V>> for State {
//    fn index<'a>(&'a self, index: & &str) -> &'a Option<V> {
//        let cstr = index.to_c_str();
//        unsafe { lua_getglobal(self.L, cstr.as_ptr()); }
//        &self.get::<V>(-1)
//    }
//}

impl FnMut<(), c_int> for State {
    #[rust_call_abi_hack]
    fn call_mut(&mut self, _args: ()) -> c_int {
        //extern fn traceback(L: *mut lua_State) -> c_int {
        extern fn traceback(state: State) -> c_int {
            unsafe {
                let msg = lua_tostring(state.L, 1);
                if msg.is_not_null() {
                    luaL_traceback(state.L, state.L, msg, 1);
                } else if lua_isnoneornil(state.L, 1) != 1 { // is there an error object?
                    if luaL_callmeta(state.L, 1, c_str!("__tostring")) == 0 { // try its 'tostring' metamethod
                        lua_pushstring(state.L, c_str!("(no error message)"));
                    }
                }
                1
            }
        }
        let base = self.top(); // function index
        self.push(traceback); // push traceback function
        self.move(base); // put it under chunk and args
        // TODO: treat SIGINT
        let status = unsafe { lua_pcall(self.L, 0, LUA_MULTRET, base as c_int) };
        self.remove(base); // remove traceback function
        status
    }
}

impl Drop for State {
    fn drop(&mut self) { unsafe { lua_close(self.L) } }
}

trait ToLua {
    unsafe fn push_to(L: *mut lua_State, val: Self);
}
trait FromLua {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<Self>;
}
trait Loader {
    unsafe fn load_to(L: *mut lua_State, source: Self) -> c_int;
}

impl<'a> Loader for &'a Path {
    unsafe fn load_to(L: *mut lua_State, source: &'a Path) -> c_int {
        // TODO: check whether luaL_loadbuffer or lua_load with a Reader is better
        let path = source.to_c_str();
        luaL_loadfile(L, path.as_ptr())
    }
}

impl<'a> Loader for &'a str {
    unsafe fn load_to(L: *mut lua_State, source: &str) -> c_int {
        luaL_loadbuffer(L, source.as_ptr() as *const c_char, source.len() as size_t, c_str!("=stdin"))
    }
}

impl<'a> Loader for &'a String {
    unsafe fn load_to(L: *mut lua_State, source: &'a String) -> c_int {
        Loader::load_to(L, source.as_slice())
    }
}

impl ToLua for () {
    unsafe fn push_to(L: *mut lua_State, _: ()) {
        lua_pushnil(L)
    }
}

impl<V: ToLua> ToLua for Option<V> {
    unsafe fn push_to(L: *mut lua_State, val: Option<V>) {
        match val {
            Some(p) => ToLua::push_to(L, p),
            None => ToLua::push_to(L, ()),
        }
    }
}

impl ToLua for bool {
    unsafe fn push_to(L: *mut lua_State, val: bool) {
        lua_pushboolean(L, if val { 1 } else { 0 })
    }
}
impl FromLua for bool {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<bool> {
        match lua_toboolean(L, idx) {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl ToLua for f64 {
    unsafe fn push_to(L: *mut lua_State, val: f64) {
        lua_pushnumber(L, val as lua_Number)
    }
}
impl FromLua for f64 {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<f64> {
        let mut isnum: c_int = 0;
        let num = lua_tonumberx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as f64),
            _ => None,
        }
    }
}

impl ToLua for f32 {
    unsafe fn push_to(L: *mut lua_State, val: f32) {
        lua_pushnumber(L, val as lua_Number)
    }
}
impl FromLua for f32 {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<f32> {
        let mut isnum: c_int = 0;
        let num = lua_tonumberx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as f32),
            _ => None,
        }
    }
}

impl ToLua for int {
    unsafe fn push_to(L: *mut lua_State, val: int) {
        lua_pushinteger(L, val as lua_Integer)
    }
}
impl FromLua for int {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<int> {
        let mut isnum: c_int = 0;
        let num = lua_tointegerx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as int),
            _ => None,
        }
    }
}

impl ToLua for i64 {
    unsafe fn push_to(L: *mut lua_State, val: i64) {
        lua_pushinteger(L, val as lua_Integer)
    }
}
impl FromLua for i64 {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<i64> {
        let mut isnum: c_int = 0;
        let num = lua_tointegerx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as i64),
            _ => None,
        }
    }
}

impl ToLua for i32 {
    unsafe fn push_to(L: *mut lua_State, val: i32) {
        lua_pushinteger(L, val as lua_Integer)
    }
}
impl FromLua for i32 {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<i32> {
        let mut isnum: c_int = 0;
        let num = lua_tointegerx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as i32),
            _ => None,
        }
    }
}

impl ToLua for uint {
    unsafe fn push_to(L: *mut lua_State, val: uint) {
        lua_pushunsigned(L, val as lua_Unsigned)
    }
}
impl FromLua for uint {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<uint> {
        let mut isnum: c_int = 0;
        let num = lua_tounsignedx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as uint),
            _ => None,
        }
    }
}

impl ToLua for u32 {
    unsafe fn push_to(L: *mut lua_State, val: u32) {
        lua_pushunsigned(L, val as lua_Unsigned)
    }
}
impl FromLua for u32 {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<u32> {
        let mut isnum: c_int = 0;
        let num = lua_tounsignedx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as u32),
            _ => None,
        }
    }
}

impl ToLua for u64 {
    unsafe fn push_to(L: *mut lua_State, val: u64) {
        lua_pushunsigned(L, val as lua_Unsigned)
    }
}
impl FromLua for u64 {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<u64> {
        let mut isnum: c_int = 0;
        let num = lua_tounsignedx(L, idx, &mut isnum);
        match isnum {
            1 => Some(num as u64),
            _ => None,
        }
    }
}

impl<'a> ToLua for &'a str {
    unsafe fn push_to(L: *mut lua_State, val: &str) {
        lua_pushlstring(L, val.as_ptr() as *const c_char, val.len() as size_t);
    }
}
impl<'a> FromLua for &'a str {
    unsafe fn get_from<'a>(L: *mut lua_State, idx: c_int) -> Option<&'a str> {
        let mut len: size_t = 0;
        let ptr = lua_tolstring(L, idx, &mut len);
        let slice = std::mem::transmute(std::raw::Slice { data: ptr, len: len as uint});
        from_utf8(slice)
    }
}

impl ToLua for String {
    unsafe fn push_to(L: *mut lua_State, val: String) {
        ToLua::push_to(L, val.as_slice())
    }
}
impl FromLua for String {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<String> {
        match FromLua::get_from(L, idx) {
            Some(s) => Some(String::from_str(s)),
            None => None,
        }
    }
}

// handy implementation for copyable types
impl<'a, V: ToLua + Copy> ToLua for &'a V {
    unsafe fn push_to(L: *mut lua_State, val: &V) {
        ToLua::push_to(L, *val);
    }
}

// this works because the struct State is a *mut lua_State so they can be transmuted
type Function = extern fn(State) -> c_int;
impl ToLua for Function {
    unsafe fn push_to(L: *mut lua_State, val: Function) {
        lua_pushcclosure(L, std::mem::transmute(Some(val)), 0)
    }
}
impl FromLua for Function {
    unsafe fn get_from(L: *mut lua_State, idx: c_int) -> Option<Function> {
        std::mem::transmute(lua_tocfunction(L, idx))
    }
}

impl<K: ToLua + Hash + Eq + Copy, V: ToLua + Copy> ToLua for HashMap<K, V> {
    unsafe fn push_to(L: *mut lua_State, val: HashMap<K, V>) {
        lua_createtable(L, val.len() as c_int, 0);
        for (key, value) in val.iter() {
            ToLua::push_to(L, key);
            ToLua::push_to(L, value);
            lua_rawset(L, -3);
        }
    }
}
// TODO: impl FromLua for HashMap, in order to get tables

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
