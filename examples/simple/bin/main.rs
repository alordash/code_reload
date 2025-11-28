use simple::add;

fn main() {
    let a = 3;
    let b = 4;
    loop {
        let sum = add(a, b);
        println!("{a} + {b} = {sum}");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
