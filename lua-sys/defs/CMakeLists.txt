project(rust-lua-defs C)
cmake_minimum_required(VERSION 2.8)

find_package(Lua)
include_directories(${LUA_INCLUDE_DIR})
add_executable(rust-lua-defs defs.c)
