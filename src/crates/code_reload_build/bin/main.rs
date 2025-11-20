use code_reload_build::SERVICES;

fn main() {
    let file_processor = &SERVICES.file_processor;
    let file_path = &std::path::Path::new("crates")
        .join("test_library")
        .join("tests")
        .join("distinguish_tests.rs");
    let build_fn_datas = file_processor.process(file_path);
    println!("done");
}
