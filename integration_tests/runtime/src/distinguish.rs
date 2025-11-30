use code_reload::hotreload;

#[hotreload(runtime)]
pub fn first() -> i32 {
    let a = 1;
    return a + 2;
}
#[code_reload::hotreload(runtime)]
pub fn second() -> i32 {
    let a = 1;
    return a + 2;
}

pub struct Foo;

pub struct Bar;

impl Foo {
    #[hotreload(runtime)]
    pub fn work2(&self) {}
}
impl Bar {
    #[hotreload(runtime)]
    pub fn work2(&self) {}
}

#[hotreload(runtime)]
pub fn regular_fn() -> i32 {
    let a = 1;
    return a + 2;
}
