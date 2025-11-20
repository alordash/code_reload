fn main() {
    let services = &code_reload_build::SERVICES;
    let file_processor = &services.file_processor;
    let output_generator = &services.output_generator;
    println!("current dir: {:?}", std::env::current_dir());
    let build_fn_datas = file_processor.process(r#"crates\test_library\test_files\big.rs"#.as_ref());
    let output = output_generator.generate(build_fn_datas);
    
    println!("output:\n{output}");

    println!("Done.");
}
