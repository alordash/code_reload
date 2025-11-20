use code_reload::hotreload;

struct Foo;

struct Bar;

impl Foo {
    #[hotreload]
    pub fn work(&self) {}
}

impl Bar {
    #[hotreload]
    pub fn work(&self) {}
}