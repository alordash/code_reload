mod submodule;

code_reload::runtime::add_runtime!();
// code_reload::runtime::add_runtime!("test_files");

#[code_reload::hotreload(runtime)]
pub fn add(left: i32, right: i32) -> i32 {
    // return left - right;
    let mut a = 0;
    for i in left..right {
        a += i;
    }
    a + left + right * 3
}
