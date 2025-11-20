use code_reload_build::{FileProcessor, FnSyntaxExtractor, IFileProcessor};
use std::sync::Arc;

fn main() {
    let fn_syntax_extractor = Arc::new(FnSyntaxExtractor);
    let file_processor = FileProcessor {
        fn_syntax_extractor,
    };
    println!("current dir: {:?}", std::env::current_dir());
    file_processor.process(r#"crates\test_library\test_files\big.rs"#.as_ref());
    
    println!("Done.");
}
