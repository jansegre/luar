// Copyright (c) 2014 Jan Segre
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(macro_rules)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::ptr;
use libc::{c_void, c_char, c_int, c_uint, c_double, size_t, ptrdiff_t};

macro_rules! c_str(
    ($t: expr) => ({let t = $t.to_c_str(); t.as_ptr()});
)

// lua version
pub const LUA_VERSION_NUM: c_int = 502;

// lua constants
pub const LUA_MINSTACK:    c_int = 20;
macro_rules! static_ints( ($($var:ident $val:expr;)+) => ( $(pub const $var: c_int = $val;)* ); )
#[cfg(target_word_size = "16")] static_ints!{ LUAI_MAXSTACK 15_000; }
#[not(cfg(target_word_size = "16"))] static_ints!{ LUAI_MAXSTACK 1_000_000; }
pub const LUAI_FIRSTPSEUDOIDX: c_int = -LUAI_MAXSTACK - 1000;
pub const LUA_REGISTRYINDEX: c_int = LUAI_FIRSTPSEUDOIDX;

// lua prompt
pub const LUA_PROMPT:  &'static str = "> ";
pub const LUA_PROMPT2: &'static str = ">> ";

// lua error results
pub const LUA_MULTRET:   c_int = -1;
pub const LUA_OK:        c_int = 0;
pub const LUA_YIELD:     c_int = 1;
pub const LUA_ERRRUN:    c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM:    c_int = 4;
pub const LUA_ERRGCMM:   c_int = 5;
pub const LUA_ERRERR:    c_int = 6;
pub const LUA_ERRFILE:   c_int = 7;

// lua gc commands
pub const LUA_GCSTOP:        c_int =  0;
pub const LUA_GCRESTART:     c_int =  1;
pub const LUA_GCCOLLECT:     c_int =  2;
pub const LUA_GCCOUNT:       c_int =  3;
pub const LUA_GCCOUNTB:      c_int =  4;
pub const LUA_GCSTEP:        c_int =  5;
pub const LUA_GCSETPAUSE:    c_int =  6;
pub const LUA_GCSETSTEPMUL:  c_int =  7;
pub const LUA_GCSETMAJORINC: c_int =  8;
pub const LUA_GCISRUNNING:   c_int =  9;
pub const LUA_GCGEN:         c_int = 10;
pub const LUA_GCINC:         c_int = 11;

// lua types
pub const LUA_TNONE:          c_int = -1;
pub const LUA_TNIL:           c_int = 0;
pub const LUA_TBOOLEAN:       c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER:        c_int = 3;
pub const LUA_TSTRING:        c_int = 4;
pub const LUA_TTABLE:         c_int = 5;
pub const LUA_TFUNCTION:      c_int = 6;
pub const LUA_TUSERDATA:      c_int = 7;
pub const LUA_TTHREAD:        c_int = 8;

pub type lua_State = c_void;
pub type lua_Alloc = Option<extern fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void>;
pub type lua_Number = c_double;
pub type lua_Integer = ptrdiff_t;
pub type lua_Unsigned = c_uint;
pub type lua_CFunction = Option<extern fn(L: *mut lua_State) -> c_int>;

extern {
    pub fn luaL_newstate() -> *mut lua_State;
    pub fn luaL_openlibs(L: *mut lua_State);
    //fn luaL_loadbuffer(L: *mut lua_State, buff: *const c_char, sz: size_t, name: *const c_char) -> c_int;
    //fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);
    pub fn lua_pcallk(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: c_int, k: lua_CFunction) -> c_int;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_close(L: *mut lua_State);
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn luaL_loadfilex(L: *mut lua_State, filename: *const c_char, mode: *const c_char) -> c_int;
    pub fn lua_rawset(L: *mut lua_State, idx: c_int);
    //pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    //pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
    pub fn lua_setglobal(L: *mut lua_State, var: *const c_char);
    pub fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number);
    pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
    pub fn lua_getglobal(L: *mut lua_State, var: *const c_char);
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;
    pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
    pub fn lua_tointegerx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Integer;
    pub fn lua_tounsignedx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Unsigned;
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> lua_CFunction;
    pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);
    pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    pub fn lua_pushcclosure(L: *mut lua_State, fun: lua_CFunction, n: c_int);
    pub fn lua_pushlstring(L: *mut lua_State, s: *const c_char, l: size_t) -> *const c_char;
    pub fn lua_pushstring(L: *mut lua_State, s: *const c_char) -> *const c_char;
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    //pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, ...) -> *const c_char;
    pub fn lua_remove(L: *mut lua_State, idx: c_int);
    pub fn lua_insert(L: *mut lua_State, idx: c_int);
    pub fn lua_concat(L: *mut lua_State, n: c_int);
    pub fn luaL_loadbufferx(L: *mut lua_State, buff: *const c_char, sz: size_t, name: *const c_char, mode: *const c_char) -> c_int;
    pub fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const c_char, level: c_int);
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn luaL_checkstack(L: *mut lua_State, sz: c_int, msg: *const c_char);
    pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn luaL_callmeta(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int;
}

//#[inline]
//pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) { lua_settop(L, -n - 1) }
//#[inline]
//pub unsafe fn lua_newtable(L: *mut lua_State) { lua_createtable(L, 0, 0) }
#[inline]
pub unsafe fn luaL_loadfile(L: *mut lua_State, f: *const c_char) -> c_int { luaL_loadfilex(L, f, ptr::null()) }
#[inline]
pub unsafe fn lua_pcall(L: *mut lua_State, n: c_int, r: c_int, f: c_int) -> c_int { lua_pcallk(L, n, r, f, 0, None) }
#[inline]
pub unsafe fn luaL_checkversion(L: *mut lua_State) { luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number) }
#[inline]
pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char { lua_tolstring(L, i, ptr::null_mut()) }
//#[inline]
//pub unsafe fn lua_pushliteral(L: *mut lua_State, s: *const c_char) -> *const c_char { lua_pushlstring(L, s, (size_of??? / size_of<c_char>()) - 1); }
#[inline]
pub unsafe fn luaL_loadbuffer(L: *mut lua_State, s: *const c_char, sz: size_t, n: *const c_char) -> c_int { luaL_loadbufferx(L, s, sz, n, ptr::null()) }
#[inline]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> c_int { (lua_type(L, n) <= 0) as c_int }
#[inline]
pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> c_int { (lua_type(L, n) == LUA_TNIL) as c_int }
#[inline]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) { lua_pushcclosure(L, f, 0) }
#[inline]
pub unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) { lua_pushcfunction(L, f); lua_setglobal(L, n) }
#[inline]
pub unsafe fn lua_upvalueindex(i: c_int) -> c_int { LUA_REGISTRYINDEX - i }
