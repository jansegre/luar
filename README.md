luarc
=====

Lua compiler written in Rust.

Requirements
------------

- [rust](https://www.rust-lang.org)

For developing use [git](https://git-scm.com) to clone the project:

    git clone -b luarc https://github.com/jansegre/luar.git luarc

### Tips for Windows

Temporarily add Rust to the PATH.

On the cmd:

    set PATH=%PATH%;"C:\Program Files\Rust stable 1.3\bin"

If on a BASH terminal (Git Bash):

    export PATH=$PATH:"/c/Program Files/Rust stable 1.3/bin"

Then enter the project dir, usually:

    cd luarc


Current status
--------------

- Lexer: implemented manually
- Parser: implemented LALR(1) parser
- Scope Analysis: implemented for local variables and global functions

Compiling and Testing
---------------------

Rust and Cargo (comes with all Rust installers) are the only requirements.

To run the current program:

    cargo build --release

    ./target/release/luarc < examples/1_err.lua
    ./target/release/luarc < examples/1_fix.lua

    ./target/release/luarc < examples/2_err.lua
    ./target/release/luarc < examples/2_fix.lua

    ./target/release/luarc < examples/3_err.lua
    ./target/release/luarc < examples/3_fix.lua

On the `_err.lua` examples, the compiler will complain and exit with error (1),
correct inputs will be output pseudo assembly code and will exit with success (0).

Check out the example outputs at `examples/*_out`.
