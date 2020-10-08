# Minimal Repro for a Rust 1.47 regression

Regressed Rust version:

```
rustc 1.47.0 (18bf6b4f0 2020-10-07)
```

## Working Code

```rust
use scroll_derive::SizeWith;

macro_rules! works {
    (pub struct $name:ident { $( pub $field:ident: $t:tt, )* }) => {
        #[derive(SizeWith)]
        pub struct $name {
            $( pub $field: $t, )*
        }
    };
}

works! {
    pub struct StructThatWorks {
        pub build_string: [u16; 260],
        pub dbg_bld_str: [u16; 40],
    }
}
```

## Failing Code

```rust
use scroll_derive::SizeWith;

macro_rules! broken {
    (pub struct $name:ident { $( pub $field:ident: $t:ty, )* }) => {
        #[derive(SizeWith)]
        pub struct $name {
            $( pub $field: $t, )*
        }
    };
}

broken! {
    pub struct BrokenStruct {
        pub build_string: [u16; 260],
        pub dbg_bld_str: [u16; 40],
    }
}
```

Error produced on that version:

```
Checking z v0.1.0 (/private/tmp/x/z)
error[E0599]: no function or associated item named `size_with` found for array `[u16; 260]` in the current scope
  --> src/lib.rs:5:18
   |
5  |           #[derive(SizeWith)]
   |                    ^^^^^^^^ function or associated item not found in `[u16; 260]`
...
21 | / broken! {
22 | |     pub struct BrokenStruct {
23 | |         pub build_string: [u16; 260],
24 | |         pub dbg_bld_str: [u16; 40],
25 | |     }
26 | | }
   | |_- in this macro invocation
   |
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no function or associated item named `size_with` found for array `[u16; 40]` in the current scope
  --> src/lib.rs:5:18
   |
5  |           #[derive(SizeWith)]
   |                    ^^^^^^^^ function or associated item not found in `[u16; 40]`
...
21 | / broken! {
22 | |     pub struct BrokenStruct {
23 | |         pub build_string: [u16; 260],
24 | |         pub dbg_bld_str: [u16; 40],
25 | |     }
26 | | }
   | |_- in this macro invocation
   |
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `z`.

To learn more, run the command again with --verbose.
```

Both code examples work on older versions of Rust.
