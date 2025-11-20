use code_reload_core::constants;
use std::path::Path;

pub trait IOutputWriter {
    fn write(&self, code_dir_name: &str, output: &str) -> Result<(), std::io::Error>;
}

pub struct OutputWriter;

impl IOutputWriter for OutputWriter {
    fn write(&self, code_dir_name: &str, output: &str) -> Result<(), std::io::Error> {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let out_file_name = self.get_out_file_name(code_dir_name);
        let out_path = Path::new(&out_dir).join(out_file_name);
        return std::fs::write(out_path, &output);
    }
}

impl OutputWriter {
    fn get_out_file_name(&self, code_dir_name: &str) -> String {
        format!(
            "{}_{}_{}",
            constants::GENERATED_CODE_PREFIX,
            code_dir_name,
            constants::GENERATED_CODE_FILE_NAME_SUFFIX
        )
    }
}
