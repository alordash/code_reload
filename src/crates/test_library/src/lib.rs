mod submodule;

include!(concat!(env!("OUT_DIR"), "/__code_reload_hotreload.rs"));

#[code_reload::hotreload]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
