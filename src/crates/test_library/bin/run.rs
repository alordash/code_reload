use test_library::add;

fn main() {
    let mut i = 0;
    loop {
        i += 1;
        let a = 44;
        let b = 18;
        let result = add(a, b);
        println!("#{i}. {a} + {b} = {result}");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Done.");
}
