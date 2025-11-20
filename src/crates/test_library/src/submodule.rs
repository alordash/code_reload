use code_reload::hotreload;

#[hotreload]
pub fn another(str: &str) -> i32 {
    str.len() as i32
}