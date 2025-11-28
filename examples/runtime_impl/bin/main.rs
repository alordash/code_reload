use runtime_impl::*;

fn main() {
    code_reload::runtime::start_watchers!(runtime_impl);

    loop {
        let mut model = Model::new();
        model.mutate();
        let number = model.number();
        println!("number = {number}, model = {model:?}");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
