# code_reload (⚠ WIP)

Library for hotreload in Rust.
It's primarily intended to speed-up development process.
It's not recommended to use it in release.

// TODO - add cool but short demo gif

## Simple usage (slower)

Your crate must be `lib` crate. You can have separate binaries in it though.  
Steps to use this library:

1. Add `code_reload` to `Cargo.toml` dependencies:

```toml
[dependencies]
code_reload = *
```

2. Add `crate-type = ["cdylib", "lib"]` to `lib` section in `Cargo.toml` (your crate must produce dynamic library):

```toml
[lib]
crate-type = ["cdylib", "lib"]
```

3. Label function you want to make hotreloadable with `#[hotreload]` attribute:

```rust
use code_reload::hotreload;

#[hotreload]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run your binary application, change `add` function and then try to rebuild it without stopping. You should see that
`add` now returns different value!

### Downside (why slower)

This approach makes it so that each call to function labeled with `#[hotreload]` actually loads your dynamic library,
searches this method there, calls it and unloads library, which is of course very slow.

But there is faster approach! We don't need to load/unload dynamic library with each call, we can keep it in memory and
reload whenever new version is built. Next section tells how to utilize this approach.

## Runtime usage (faster)

Your crate must be `lib` crate. You can have separate binaries in it though.  
This approach also uses `build` script.  
Steps to use this library:

1. Add `code_reload` with `runtime` feature to `Cargo.toml` dependencies and build dependencies:

```toml
[dependencies]
code_reload = { version = "*", features = ["runtime"] }

[build-dependencies]
code_reload = { version = "*", features = ["runtime"] }
```

2. Add `crate-type = ["cdylib", "lib"]` to `lib` section in `Cargo.toml` (your crate must produce dynamic library):

```toml
[lib]
crate-type = ["cdylib", "lib"]
```

3. Add `code_reload::runtime::build()` to your build script (file `build.rs` in the root of your crate). This function
   parses your code and generates dynamic library wrapper structures that hold pointers to hotreloadable functions.

```rust
// build.rs
fn main() {
    code_reload::runtime::build();
}
```

4. Add `code_reload::runtime::start_watchers!(your_crate_name)` somewhere in your binary's `main` function. This spawns
   watcher
   that looks after your dynamic library file and reloads dynamic library when it changes. `your_crate_name` is either
   `package.name` from `Cargo.toml` or just `crate` if your binary is located in the same place as your library's code.

```rust
// main.rs
fn main() {
    code_reload::runtime::start_watchers!(your_crate_name);
    // your code
}
```

5. Label function you want to make hotreloadable with `#[hotreload(runtime)]` attribute:

```rust
use code_reload::hotreload;

#[hotreload(runtime)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run your binary application, change `add` function and then try to rebuild it without stopping. You should see that
`add` now returns different value!

## Examples

You can see usage examples in [examples](examples) directory.

## Limitations

TODO - write about them (there are quite a few unfortunately)

## Benchmarks

Benchmarks are located in [benchmarks](benchmarks) folder. You can run them to see the execution time difference between
no hotreload, simple hotreload and runtime hotreload invocation of same function.

Here are benchmark results from my local PC (i7-14700HX):

```
no hotreload fibonacci time:        [16.245 ns 16.348 ns 16.469 ns]

simple hotreload fibonacci time:    [95.439 µs 96.832 µs 98.403 µs]

runtime hotreload fibonacci time:   [19.498 ns 19.692 ns 19.917 ns]
```

#### // TODO

- [ ] write about limitations
- [ ] write about `code_reload::runtime::build` for tests and separate directories
