# mini-ts-typechecker

This is a reimplementation of [mini-typescript](https://github.com/sandersn/mini-typescript) in Rust.

## Notes
- Im quite sure that `fn()()` is not supported -> test.
- Also, you could do something like `"lol" = "haha"`, which is supported by the parser -> is it the responsibility of checker or it should in fact throw error?

## Todo:
- Fix weird handeling of EOF, mainly when calling scan (or next) on the EOF, panic!.
- Make errors more specific, when parsing