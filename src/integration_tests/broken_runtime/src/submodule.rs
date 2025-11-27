use code_reload::hotreload;

struct Foo;

impl Foo {
    // TODO - won't work because `Foo` is private
    #[hotreload(runtime)]
    pub fn work2(&self) {}
}
