#![allow(unused)]

code_reload::runtime::add_tests_runtime!();

use code_reload::hotreload;
use std::env::Args;
use std::pin::Pin;

#[hotreload]
fn function() {
    let _nothing = ();
}

#[hotreload(runtime)]
#[inline(never)]
fn function_with_attribute() {
    let _nothing = ();
}

#[inline(never)]
#[hotreload]
fn function_with_args(args: Args, number: i32, (tuple_1, tuple_2): (String, f32)) {
    let _args = args;
}

#[hotreload]
fn function_with_return() -> Result<String, String> {
    Ok(String::from("function_with_return"))
}

#[hotreload]
fn function_with_args_and_return(args: Args) -> Result<String, String> {
    Ok("function_with_args_and_return".to_owned() + &*args.len().to_string())
}

struct Model {
    value: i32,
}

impl Model {
    #[hotreload]
    #[inline(never)]
    pub fn read(&self, another_num: f32) -> i32 {
        self.value + another_num as i32
    }
    
    #[hotreload]
    pub fn write(&mut self) {
        self.value += 1;
    }
    
    #[hotreload(runtime)]
    pub fn read_and_write(&mut self) -> i32 {
        self.value += 1;
        self.value
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
        self.value
    }
}
