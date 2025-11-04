#[code_reload::hotreload]
pub fn add1(left: i32, right: i32) -> i32 {
    left + right
}

pub struct FileProcessor;

#[code_reload::hotreload]pub fn add2(left: i32, right: i32) -> i32 {
    left + right
}

impl IFileProcessor for FileProcessor {
    fn process(&self, file_path: &Path) {
        let mut searcher = Searcher::new();

        log!("Searching for {:?} in '{:?}'", HOTRELOAD_ATTRIBUTE, file_path);
        searcher
            .search_path(
                &*HOTRELOAD_ATTRIBUTE_MATCHER,
                file_path,
                UTF8(|line_number, line| self.handle_entry(line_number, line)),
            )
            .unwrap();
    }
}

#[code_reload::hotreload] pub fn add3(left: i32, right: i32) -> i32 {
    left + right
}

impl FileProcessor {
    fn handle_entry(&self, line_number: u64, line: &str) -> Result<bool, std::io::Error> {
        log!("Found macro at line #{}: {:?}", line_number, line);

        return Ok(true);
    }
}

#[code_reload::hotreload]
pub fn add4(left: i32, right: i32) -> i32 {
    left + right
}