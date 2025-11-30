use code_reload::hotreload;

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
