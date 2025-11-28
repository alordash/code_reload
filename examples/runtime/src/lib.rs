use code_reload::hotreload;

code_reload::runtime::add_runtime!();

#[hotreload(runtime)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
