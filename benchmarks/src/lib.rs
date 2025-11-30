use code_reload::hotreload;

code_reload::runtime::add_runtime!();

pub fn no_hotreload_fibonacci(n: u128) -> u128 {
    fibonacci(n)
}

#[hotreload]
pub fn simple_hotreload_fibonacci(n: u128) -> u128 {
    fibonacci(n)
}

#[hotreload(runtime)]
pub fn runtime_hotreload_fibonacci(n: u128) -> u128 {
    fibonacci(n)
}

#[inline(never)]
fn fibonacci(n: u128) -> u128 {
    let (mut a, mut b) = (0, 1);
    for _ in 1..=n {
        (a, b) = (a + b, a);
    }
    return a;
}
