use code_reload_core::constants;
use std::path::Path;
use crate::debug_log::log;

pub trait IOutputWriter {
    fn write(&self, output: &str) -> Result<(), std::io::Error>;
}

pub struct OutputWriter;

impl IOutputWriter for OutputWriter {
    fn write(&self, output: &str) -> Result<(), std::io::Error> {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        log!("out_dir: {out_dir}");
        let out_path = Path::new(&out_dir).join(constants::GENERATED_CODE_FILE_NAME);
        return std::fs::write(out_path, &output);
    }
}
