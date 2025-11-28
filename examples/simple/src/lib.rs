use code_reload::hotreload;

#[hotreload]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
