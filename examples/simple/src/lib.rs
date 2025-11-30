use code_reload::hotreload;

#[hotreload]
pub fn add(mut a: i32, b: i32) -> i32 {
    a += 1;
    a + b
}
