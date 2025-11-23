use code_reload_build::SERVICES;
use code_reload_core::merge_file_and_manifest_paths;
use std::path::Path;

fn main() {
    let a = Path::new("crates\\test_demrary\\src\\lib.rs");
    let b = Path::new("C:\\_RS\\code_reload\\src\\crates\\test_demrary");
    let c = merge_file_and_manifest_paths(a, b);

    unsafe {
        std::env::set_var(
            "OUT_DIR",
            r#"D:\.cargo-target\debug\build\test_library-557d6477c2aff484\out"#,
        );
    }

    let file_processor = &SERVICES.file_processor;
    let file_path = &std::path::Path::new("crates")
        .join("test_library")
        .join("tests")
        .join("hotreload_tests.rs");
    let build_fn_datas = file_processor.process(file_path);
    let impl_type_exporter = &SERVICES.impl_type_exporter;
    impl_type_exporter.export(&build_fn_datas);
    println!("done");
}
