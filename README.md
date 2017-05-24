This is an example repo with crates M, L, and C.
M is the main binary crate.
L is a dynamic library that is dyamically loaded by M.
C is a crate used by both M and L that containing a global static value that should not be shadowed.

To get this working, put

```text
[lib]
crate-type = ["dylib"]
```

in C & L's `Cargo.toml`.
