// Copyright (c) 2014 Jan Segre
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This is a declaration of the full API described at [the Lua Reference Manual](http://www.lua.org/manual/5.2/manual.html).

#![feature(macro_rules)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//use std::ptr;
extern crate libc;
use libc::{c_void, c_char, c_uchar, c_int, c_long, c_ulong, c_double, size_t, ptrdiff_t};

// code generated from defs/defs.c
include!(concat!(env!("OUT_DIR"), "/defs.rs"))

pub type lua_Alloc = Option<extern fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void>;
pub type lua_CFunction = Option<extern fn(L: *mut lua_State) -> c_int>;
pub type lua_Integer = ptrdiff_t;
pub type lua_Number = c_double;
pub type lua_Reader = Option<extern fn(L: *mut lua_State, data: *mut c_void, size: *mut size_t) -> *const c_char>;
pub type lua_State = c_void;
pub type lua_Unsigned = c_ulong;
pub type lua_Writer = Option<extern fn(L: *mut lua_State, p: *const c_void, sz: size_t, ud: *mut c_void)>;
pub type lua_Hook = Option<extern fn(L: *mut lua_State, ar: *mut lua_Debug) -> c_void>;
pub type luaL_Buffer = c_void;

#[repr(C)]
pub struct luaL_Reg {
 pub name: *const c_char,
 pub func: lua_CFunction,
}

#[repr(C)]
pub struct lua_Debug {
 pub event: c_int,
 pub name: *const c_char,     /*(n) */
 pub namewhat: *const c_char, /*(n) */
 pub what: *const c_char,     /*(S) */
 pub source: *const c_char,   /*(S) */
 pub currentline: c_int,      /*(l) */
 pub linedefined: c_int,      /*(S) */
 pub lastlinedefined: c_int,  /*(S) */
 pub nups: c_uchar,           /*(u) number of upvalues */
 pub nparams: c_uchar,        /*(u) number of parameters */
 pub isvararg: c_char,        /*(u) */
 pub istailcall: c_char,      /*(t) */
 pub short_src: [c_char, ..(LUA_IDSIZE as uint)], /*(S) */
 /* private part */
 //other fields
}

extern {
    pub fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_arith(L: *mut lua_State, op: c_int);
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
    pub fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);
    pub fn lua_callk(L: *mut lua_State, nargs: c_int, nresults: c_int, ctx: c_int, k: lua_CFunction);
    pub fn lua_checkstack(L: *mut lua_State, extra: c_int) -> c_int;
    pub fn lua_close(L: *mut lua_State);
    pub fn lua_compare(L: *mut lua_State, index1: c_int, index2: c_int,op: c_int) -> c_int;
    pub fn lua_concat(L: *mut lua_State, n: c_int);
    pub fn lua_copy(L: *mut lua_State, fromidx: c_int, toidx: c_int);
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn lua_dump(L: *mut lua_State, writer: lua_Writer, data: *mut c_void) -> c_int;
    pub fn lua_error(L: *mut lua_State) -> c_int;
    pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;
    pub fn lua_getctx(L: *mut lua_State, ctx: *mut c_int) -> c_int;
    pub fn lua_getfield(L: *mut lua_State, index: c_int, k: *const c_char);
    pub fn lua_getglobal(L: *mut lua_State, name: *const c_char);
    pub fn lua_getmetatable(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_gettable(L: *mut lua_State, index: c_int);
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn lua_getuservalue(L: *mut lua_State, index: c_int);
    pub fn lua_insert(L: *mut lua_State, index: c_int);
    pub fn lua_isboolean(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_iscfunction(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isfunction(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_islightuserdata(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isnil(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isnone(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isnoneornil(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isnumber(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isstring(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_istable(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isthread(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isuserdata(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_len(L: *mut lua_State, index: c_int);
    pub fn lua_load(L: *mut lua_State, reader: lua_Reader, data: *mut c_void, source: *const c_char, mode: *const c_char) -> c_int;
    pub fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
    pub fn lua_newtable(L: *mut lua_State);
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
    pub fn lua_newuserdata(L: *mut lua_State, size: size_t) -> *mut c_void;
    pub fn lua_next(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_pcall(L: *mut lua_State, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int;
    pub fn lua_pcallk(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: c_int, k: lua_CFunction) -> c_int;
    pub fn lua_pop(L: *mut lua_State, n: c_int);
    pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    pub fn lua_pushcclosure(L: *mut lua_State, f: lua_CFunction, n: c_int);
    pub fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction);
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, ...) -> *const c_char;
    pub fn lua_pushglobaltable(L: *mut lua_State);
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    pub fn lua_pushliteral(L: *mut lua_State, s: *const c_char) -> *const c_char;
    pub fn lua_pushlstring(L: *mut lua_State, s: *const c_char, len: size_t) -> *const c_char;
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_pushstring(L: *mut lua_State, s: *const c_char) -> *const c_char;
    pub fn lua_pushthread(L: *mut lua_State) -> c_int;
    pub fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);
    pub fn lua_pushvalue(L: *mut lua_State, index: c_int);
    //pub fn lua_pushvfstring(L: *mut lua_State, fmt: *const c_char, argp: va_list) -> *const c_char;  // va_list?
    pub fn lua_rawequal(L: *mut lua_State, index1: c_int, index2: c_int,) -> c_int;
    pub fn lua_rawget(L: *mut lua_State, index: c_int);
    pub fn lua_rawgeti(L: *mut lua_State, index: c_int, n: c_int);
    pub fn lua_rawgetp(L: *mut lua_State, index: c_int, p: *const c_void);
    pub fn lua_rawlen(L: *mut lua_State, index: c_int) -> size_t;
    pub fn lua_rawset(L: *mut lua_State, index: c_int);
    pub fn lua_rawseti(L: *mut lua_State, index: c_int, n: c_int);
    pub fn lua_rawsetp(L: *mut lua_State, index: c_int, p: *const c_void);
    pub fn lua_register(L: *mut lua_State, name: *const c_char, f: lua_CFunction);
    pub fn lua_remove(L: *mut lua_State, index: c_int);
    pub fn lua_replace(L: *mut lua_State, index: c_int);
    pub fn lua_resume(L: *mut lua_State, from: *mut lua_State, nargs: c_int) -> c_int;
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut c_void);
    pub fn lua_setfield(L: *mut lua_State, index: c_int, k: *const c_char);
    pub fn lua_setglobal(L: *mut lua_State, name: *const c_char);
    pub fn lua_setmetatable(L: *mut lua_State, index: c_int);
    pub fn lua_settable(L: *mut lua_State, index: c_int);
    pub fn lua_settop(L: *mut lua_State, index: c_int);
    pub fn lua_setuservalue(L: *mut lua_State, index: c_int);
    pub fn lua_status(L: *mut lua_State) -> c_int;
    pub fn lua_toboolean(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_tocfunction(L: *mut lua_State, index: c_int) -> lua_CFunction;
    pub fn lua_tointeger(L: *mut lua_State, index: c_int) -> lua_Integer;
    pub fn lua_tointegerx(L: *mut lua_State, index: c_int, isnum: *mut c_int) -> lua_Integer;
    pub fn lua_tolstring(L: *mut lua_State, index: c_int, len: *mut size_t) -> *const c_char;
    pub fn lua_tonumber(L: *mut lua_State, index: c_int) -> lua_Number;
    pub fn lua_tonumberx(L: *mut lua_State, index: c_int, isnum: *mut c_int) -> lua_Number;
    pub fn lua_topointer(L: *mut lua_State, index: c_int) -> *const c_void;
    pub fn lua_tostring(L: *mut lua_State, index: c_int) -> *const c_char;
    pub fn lua_tothread(L: *mut lua_State, index: c_int) -> *mut lua_State;
    pub fn lua_tounsigned(L: *mut lua_State, index: c_int) -> lua_Unsigned;
    pub fn lua_tounsignedx(L: *mut lua_State, index: c_int, isnum: *mut c_int) -> lua_Unsigned;
    pub fn lua_touserdata(L: *mut lua_State, index: c_int) -> *mut c_void;
    pub fn lua_type(L: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_typename(L: *mut lua_State, tp: c_int) -> *const c_char;
    pub fn lua_upvalueindex(i: c_int) -> c_int;
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);
    pub fn lua_yield(L: *mut lua_State, nresults: c_int) -> c_int;
    pub fn lua_yieldk(L: *mut lua_State, nresults: c_int, ctx: c_int, k: lua_CFunction) -> c_int;
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    pub fn lua_gethookcount(L: *mut lua_State) -> c_int;
    pub fn lua_gethookmask(L: *mut lua_State) -> c_int;
    pub fn lua_getinfo(L: *mut lua_State, what: *const c_char, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getlocal(L: *mut lua_State, ar: *mut lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_getstack(L: *mut lua_State, level: c_int, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;
    pub fn lua_sethook(L: *mut lua_State, f: lua_Hook, mask: c_int, count: c_int) -> c_int;
    pub fn lua_setlocal(L: *mut lua_State, ar: *mut lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_setupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;
    pub fn lua_upvalueid(L: *mut lua_State, funcindex: c_int, n: c_int) -> *mut c_void;
    pub fn lua_upvaluejoin(L: *mut lua_State, funcindex1: c_int, n1: c_int, funcindex2: c_int, n2: c_int);
    pub fn luaL_addchar(B: *mut luaL_Buffer, c: c_char);
    pub fn luaL_addlstring(B: *mut luaL_Buffer, s: *const c_char, l: size_t);
    pub fn luaL_addsize(B: *mut luaL_Buffer, n: size_t);
    pub fn luaL_addstring(B: *mut luaL_Buffer, s: *const c_char);
    pub fn luaL_addvalue(B: *mut luaL_Buffer);
    pub fn luaL_argcheck(L: *mut lua_State, cond: c_int, arg: c_int, extramsg: *const c_char);
    pub fn luaL_argerror(L: *mut lua_State, arg: c_int, extramsg: *const c_char) -> c_int;
    pub fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    pub fn luaL_buffinitsize(L: *mut lua_State, B: *mut luaL_Buffer, sz: size_t) -> *mut c_char;
    pub fn luaL_callmeta(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int;
    pub fn luaL_checkany(L: *mut lua_State, arg: c_int);
    pub fn luaL_checkint(L: *mut lua_State, arg: c_int) -> c_int;
    pub fn luaL_checkinteger(L: *mut lua_State, arg: c_int) -> lua_Integer;
    pub fn luaL_checklong(L: *mut lua_State, arg: c_int) -> c_long;
    pub fn luaL_checklstring(L: *mut lua_State, arg: c_int, l: *mut size_t) -> *const c_char;
    pub fn luaL_checknumber(L: *mut lua_State, arg: c_int) -> lua_Number;
    pub fn luaL_checkoption(L: *mut lua_State, arg: c_int, def: *const c_char, lst: *const *const c_char) -> c_int;
    pub fn luaL_checkstack(L: *mut lua_State, sz: c_int, msg: *const c_char);
    pub fn luaL_checkstring(L: *mut lua_State, arg: c_int) -> *const c_char;
    pub fn luaL_checktype(L: *mut lua_State, arg: c_int, t: c_int);
    pub fn luaL_checkudata(L: *mut lua_State, arg: c_int, tname: *const c_char) -> *mut c_void;
    pub fn luaL_checkunsigned(L: *mut lua_State, arg: c_int) -> lua_Unsigned;
    pub fn luaL_checkversion(L: *mut lua_State);
    pub fn luaL_dofile(L: *mut lua_State, filename: *const c_char) -> c_int;
    pub fn luaL_dostring(L: *mut lua_State, str: *const c_char) -> c_int;
    pub fn luaL_error(L: *mut lua_State, fmt: *const c_char, ...) -> c_int;
    pub fn luaL_execresult(L: *mut lua_State, stat: c_int) -> c_int;
    pub fn luaL_fileresult(L: *mut lua_State, stat: c_int, fname: *const c_char) -> c_int;
    pub fn luaL_getmetafield(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int;
    pub fn luaL_getmetatable(L: *mut lua_State, tname: *const c_char);
    pub fn luaL_getsubtable(L: *mut lua_State, idx: c_int, fname: *const c_char) -> c_int;
    pub fn luaL_gsub(L: *mut lua_State, s: *const c_char, p: *const c_char, r: *const c_char) -> *const c_char;
    pub fn luaL_len(L: *mut lua_State, index: c_int) -> c_int;
    pub fn luaL_loadbuffer(L: *mut lua_State, buff: *const c_char, sz: size_t, name: *const c_char) -> c_int;
    pub fn luaL_loadbufferx(L: *mut lua_State, buff: *const c_char, sz: size_t, name: *const c_char, mode: *const c_char) -> c_int;
    pub fn luaL_loadfile(L: *mut lua_State, filename: *const c_char) -> c_int;
    pub fn luaL_loadfilex(L: *mut lua_State, filename: *const c_char, mode: *const c_char) -> c_int;
    pub fn luaL_loadstring(L: *mut lua_State, s: *const c_char) -> c_int;
    pub fn luaL_newlib(L: *mut lua_State, l: *const luaL_Reg);
    pub fn luaL_newlibtable(L: *mut lua_State, l: *const luaL_Reg);
    pub fn luaL_newmetatable(L: *mut lua_State, tname: *const c_char) -> c_int;
    pub fn luaL_newstate() -> *mut lua_State;
    pub fn luaL_openlibs(L: *mut lua_State);
    pub fn luaL_optint(L: *mut lua_State, arg: c_int, d: c_int) -> c_int;
    pub fn luaL_optinteger(L: *mut lua_State, arg: c_int, d: lua_Integer) -> lua_Integer;
    pub fn luaL_optlong(L: *mut lua_State, arg: c_int, d: c_long) -> c_long;
    pub fn luaL_optlstring(L: *mut lua_State, arg: c_int, d: *const c_char, l: *mut size_t) -> *const c_char;
    pub fn luaL_optnumber(L: *mut lua_State, arg: c_int, d: lua_Number) -> lua_Number;
    pub fn luaL_optstring(L: *mut lua_State, arg: c_int, d: *const c_char) -> *const c_char;
    pub fn luaL_optunsigned(L: *mut lua_State, arg: c_int, u: lua_Unsigned) -> lua_Unsigned;
    pub fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut c_char;
    pub fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut c_char;
    pub fn luaL_pushresult(B: *mut luaL_Buffer);
    pub fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: size_t);
    pub fn luaL_ref(L: *mut lua_State, t: c_int) -> c_int;
    pub fn luaL_requiref(L: *mut lua_State, modname: *const c_char, openf: lua_CFunction, glb: c_int);
    pub fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: c_int);
    pub fn luaL_setmetatable(L: *mut lua_State, tname: *const c_char);
    pub fn luaL_testudata(L: *mut lua_State, arg: c_int, tname: *const c_char) -> *mut c_void;
    pub fn luaL_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;
    pub fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const c_char, level: c_int);
    pub fn luaL_typename(L: *mut lua_State, index: c_int) -> *const c_char;
    pub fn luaL_unref(L: *mut lua_State, t: c_int, _ref: c_int);
    pub fn luaL_where(L: *mut lua_State, lvl: c_int);
}
