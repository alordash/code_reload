use code_reload::hotreload;
use std::env::Args;
use std::pin::Pin;

struct Model {
    number: i32,
}

impl Model {
    #[hotreload]
    #[inline(never)]
    pub fn read(&self, another_num: f32) -> i32 {
        self.number + another_num as i32
    }

    #[hotreload]
    pub fn write(&mut self) {
        self.number += 1;
    }

    #[hotreload]
    pub fn read_and_write(&mut self) -> i32 {
        self.number += 1;
        self.number
    }

    #[hotreload]
    pub fn r#static(kavo: Box<Self>, chevo: Pin<Self>) {
        let _nothing = ();
    }

    #[hotreload]
    pub fn pin(self: Pin<&Self>) {}

    #[hotreload]
    pub fn pin_mut(self: Pin<&mut Self>) {}

    #[hotreload]
    pub fn mut_pin(mut self: Pin<&Self>) {}

    #[hotreload]
    pub fn mut_pin_mut(mut self: Pin<&mut Self>) {}

    #[hotreload]
    pub fn consume(self) -> i32 {
        self.number
    }
}
