use runtime::*;

fn main() {
    code_reload::runtime::start_watchers!(runtime);

    let a = 3;
    let b = 4;
    loop {
        let sum = add(a, b);
        println!("{a} + {b} = {sum}");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
