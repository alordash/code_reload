#![allow(unused)]

use code_reload::hotreload;

code_reload::runtime::add_runtime!("custom_dir");

#[hotreload(runtime)]
fn function() {
    let _nothing = ();
}