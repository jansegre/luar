luarc
=====

Experimental Lua compiler written in Rust.

This is an experimental sub-project of Luar (Lua bindings for Rust) which
focuses on compiling Lua source code to Lua byte-code.

Current status
--------------

- Lexer: implemented
- Parser: on-going recursive parser experiments, will settle on a LR(1) or LALR(1) parser.

Compiling and Testing
---------------------

Rust and Cargo (comes with all Rust installers) are the only requirements.

To run the current demo:

    cargo run

These are some exemples (note that whitespace/comment are not real tokens: they
are not passed to the parser):


    a + b -- comment
    Ok(Ident("a"))
    Ok(Whitespace)
    Ok(Plus)
    Ok(Whitespace)
    Ok(Ident("b"))
    Ok(Whitespace)
    Ok(Comment)
    Ok(Eof)

    a+b--comment
    Ok(Ident("a"))
    Ok(Plus)
    Ok(Ident("b"))
    Ok(Comment)
    Ok(Eof)

    a1 5 0b
    Ok(Ident("a1"))
    Ok(Whitespace)
    Ok(Int(5))
    Ok(Whitespace)
    Err(TokenIntError(ParseIntError { kind: InvalidDigit }))
    Ok(Whitespace)
    Ok(Eof)

    abcd 123 "asdf\"asdf"
    Ok(Ident("abcd"))
    Ok(Whitespace)
    Ok(Int(123))
    Ok(Whitespace)
    Ok(Str("asdf\\\"asdf"))
    Ok(Whitespace)
    Ok(Eof)

Each example is terminated with `^D` (EOF, which too is not a real token).
