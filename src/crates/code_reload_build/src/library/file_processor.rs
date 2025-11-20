use crate::debug_log::log;
use grep_regex::RegexMatcher;
use grep_searcher::Searcher;
use grep_searcher::sinks::UTF8;
use std::cell::LazyCell;
use std::path::Path;
use syn::parse::Parse;

const HOTRELOAD_ATTRIBUTE: &'static str = "#\\[(code_reload::)?hotreload\\]";
const HOTRELOAD_ATTRIBUTE_MATCHER: LazyCell<RegexMatcher> =
    LazyCell::new(|| RegexMatcher::new(HOTRELOAD_ATTRIBUTE).unwrap());

pub trait IFileProcessor {
    fn process(&self, file_path: &Path);
}

pub struct FileProcessor;

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

impl FileProcessor {
    fn handle_entry(&self, line_number: u64, line: &str) -> Result<bool, std::io::Error> {
        log!("Found macro at line #{}: {:?}", line_number, line);

        return Ok(true);
    }
}
