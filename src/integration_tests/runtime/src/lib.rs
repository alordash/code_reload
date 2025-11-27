#![allow(unused)]

mod distinguish;

use code_reload::hotreload;

code_reload::runtime::add_runtime!();

#[hotreload(runtime)]
fn function() {
    let _nothing = ();
}

#[hotreload(runtime)]
#[inline(never)]
fn function_with_attribute() {
    let _nothing = ();
}

#[inline(never)]
#[hotreload(runtime)]
fn function_with_args(args: std::env::Args, number: i32, (tuple_1, tuple_2): (String, f32)) {
    let _args = args;
}

#[hotreload(runtime)]
fn function_with_return() -> Result<String, String> {
    Ok(String::from("function_with_return"))
}

#[hotreload(runtime)]
fn function_with_args_and_return(args: std::env::Args) -> Result<String, String> {
    Ok("function_with_args_and_return".to_owned() + &*args.len().to_string())
}

struct Model {
    value: i32,
}

impl Model {
    #[hotreload(runtime)]
    #[inline(never)]
    pub fn read(&self, another_num: f32) -> i32 {
        self.value + another_num as i32
    }

    #[hotreload(runtime)]
    pub fn write(&mut self) {
        self.value += 1;
    }

    #[hotreload(runtime)]
    pub fn read_and_write(&mut self) -> i32 {
        self.value += 1;
        self.value
    }

    #[hotreload(runtime)]
    pub fn r#static(kavo: Box<Self>, chevo: std::pin::Pin<Self>) {
        let _nothing = ();
    }

    #[hotreload(runtime)]
    pub fn pin(self: std::pin::Pin<&Self>) {}

    #[hotreload(runtime)]
    pub fn pin_mut(self: std::pin::Pin<&mut Self>) {}

    #[hotreload(runtime)]
    pub fn mut_pin(mut self: std::pin::Pin<&Self>) {}

    #[hotreload(runtime)]
    pub fn mut_pin_mut(mut self: std::pin::Pin<&mut Self>) {}

    #[hotreload(runtime)]
    pub fn consume(self) -> i32 {
        self.value
    }
}
