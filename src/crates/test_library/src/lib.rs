mod submodule;

include!(concat!(env!("OUT_DIR"), "/__code_reload_hotreload.rs"));

#[code_reload::hotreload(runtime)]
pub fn add(left: i32, right: i32) -> i32 {
    // return left - right;
    let mut a = 0;
    for i in left..right {
        a += i;
    }
    a + left + right * 3
}
