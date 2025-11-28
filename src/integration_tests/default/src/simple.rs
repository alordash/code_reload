use code_reload::hotreload;
use std::env::Args;

#[hotreload]
fn function() {
    let _nothing = ();
}

#[hotreload]
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
