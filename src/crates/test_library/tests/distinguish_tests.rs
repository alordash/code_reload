#[hotreload(runtime)]
pub fn first() -> i32 {
    let a = 1;
    return a + 2;
}

use code_reload::hotreload;

struct Foo;

struct Bar;

impl Foo { #[hotreload(runtime)] pub fn work2(&self) {} } impl Bar { #[hotreload(runtime)] pub fn work2(&self) {} }

   #[hotreload(runtime)]
pub fn regular_fn() -> i32 {
    let a = 1;
    return a + 2;
}
