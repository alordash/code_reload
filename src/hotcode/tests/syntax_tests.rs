#[hotcode::hotreload]
fn tuple((a, b): (i32, i32)) {
    dbg!(a, b);
}

struct Struct {
    a: i32,
    b: i32,
    #[allow(unused)]
    c: i32,
}

#[hotcode::hotreload]
fn deconstruct(Struct { a, b, .. }: Struct) {
    dbg!(a, b);
}

#[hotcode::hotreload]
async fn async_example(v: i32) -> i32 {
    v + 1
}

mod tests {
    #[test]
    fn compile() {}
}
