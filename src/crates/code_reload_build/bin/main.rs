use std::path::Path;
use code_reload_build::SERVICES;
use code_reload_core::merge_file_and_manifest_paths;

fn main() {
    let a = Path::new("crates\\test_demrary\\src\\lib.rs");
    let b = Path::new("C:\\_RS\\code_reload\\src\\crates\\test_demrary");
    let c = merge_file_and_manifest_paths(a, b);
    
    let file_processor = &SERVICES.file_processor;
    let file_path = &std::path::Path::new("crates")
        .join("test_library")
        .join("tests")
        .join("distinguish_tests.rs");
    let build_fn_datas = file_processor.process(file_path);
    println!("done");
}
