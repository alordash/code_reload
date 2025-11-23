code_reload::runtime::add_tests_runtime!();

use std::pin::Pin;

pub struct Model;

impl Model {
    #[hotreload(runtime)]
    pub fn pin(self: Pin<&Self>) {}
}
