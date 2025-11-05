fn main() {
    let file_processor = &code_reload_build::SERVICES.file_processor;
    println!("current dir: {:?}", std::env::current_dir());
    file_processor.process(r#"crates\test_library\test_files\big.rs"#.as_ref());

    println!("Done.");
}
