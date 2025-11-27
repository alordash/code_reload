#![allow(unused)]

use code_reload::hotreload;

code_reload::runtime::add_tests_runtime!();

#[hotreload(runtime)]
fn function() {
    let _nothing = ();
}
