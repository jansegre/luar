Luar
====

A rustic Rust API that embeds the C Lua interpreter.
Main goals are easily embedding Lua and not requiring it to be installed on the system,
meaning only listing it as a dependency on Cargo should be enough:

```toml
[dependencies.luar]
git = "https://github.com/jansegre/luar.git"
```

What's with the name?
---------------------

"Luar" (pronounced __LOO-R__) means "Moonlight" in Portuguese. Also in Portuguese if
there was (there isn't) a verb for "moon" as "to moon" it would be said "Luar", so you
may optionally think of it as "to moon" as the act of using Lua (in Rust).

API
---

This API is inspired on the Goddess of the Moon, [Selene](https://github.com/jeremyong/Selene).

There will be some examples, if there isn't feel free to bother me demanding I put examples here.
