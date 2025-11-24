#![allow(unused)]

code_reload::runtime::add_tests_runtime!();

use code_reload::hotreload;
use std::env::Args;
use std::pin::Pin;

struct RuntimeModel {
    value: i32,
}

impl RuntimeModel {
    #[hotreload(runtime)]
    pub fn r#static(chevo: Pin<Self>) {
        let _nothing = ();
    }

    #[hotreload(runtime)]
    pub fn pin(self: Pin<&Self>) {}

    #[hotreload(runtime)]
    pub fn pin_mut(self: Pin<&mut Self>) {}

    #[hotreload(runtime)]
    pub fn mut_pin(mut self: Pin<&Self>) {}

    #[hotreload(runtime)]
    pub fn mut_pin_mut(mut self: Pin<&mut Self>) {}

    #[hotreload(runtime)]
    pub fn consume(self) -> i32 {
        self.value
    }
}
