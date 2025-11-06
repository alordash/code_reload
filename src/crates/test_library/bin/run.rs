use test_demrary::{mutate, Data};
use test_library::add;

fn main() {
    code_reload::runtime::start_watchers!(test_library, test_demrary);
    
    let mut i = 0;
    loop {
        i += 1;
        let a = 18;
        let b = 44;
        let result = add(a, b);
        println!("#{i}. {a} + {b} = {result}");
        
        let mut data = Data::new();
        println!("\tnew data:     {data:?}");
        println!("\tdata string: {}", data.string());
        mutate(&mut data);
        println!("\tmutated data: {data:?}");
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }

    println!("Done.");
}
