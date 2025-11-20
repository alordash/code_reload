use crate::IItemFnMapper;
use crate::debug_log::log;
use crate::runtime::models::BuildFnData;
use code_reload_core::{SourceCodeId, constants};
use memmap2::Mmap;
use std::cell::LazyCell;
use std::iter;
use std::path::Path;
use std::sync::Arc;

pub trait IFileProcessor {
    fn process(&self, file_path: &Path) -> Vec<BuildFnData>;
}

pub struct FileProcessor {
    pub item_fn_mapper: Arc<dyn IItemFnMapper>,
}

impl IFileProcessor for FileProcessor {
    fn process(&self, file_path: &Path) -> Vec<BuildFnData> {
        // log!("processing file: '{:?}'", file_path);
        let file = std::fs::File::open(file_path).unwrap();
        let file_memory = unsafe { Mmap::map(&file).unwrap() };

        let build_fn_datas = self.extract(file_path, &file_memory);
        for build_fn_data in build_fn_datas.iter() {
            let source_code_id = build_fn_data.source_code_id();
            log!("[build] source_code_id: {source_code_id}");
        }

        return build_fn_datas;
    }
}

impl FileProcessor {
    /// Attribute can be specified as
    /// `#[hotreload]`
    /// or
    /// `#[code_reload::hotreload]`
    const ATTRIBUTE_BODY: &'static [u8] = b"hotreload";
    const ATTRIBUTE_BODY_LEN: usize = Self::ATTRIBUTE_BODY.len();

    const SHORT_ATTRIBUTE_HEAD: &'static [u8] = b"#[";
    const SHORT_ATTRIBUTE_HEAD_LEN: usize = Self::SHORT_ATTRIBUTE_HEAD.len();
    const LONG_ATTRIBUTE_HEAD: &'static [u8] = b"#[code_reload::";
    const LONG_ATTRIBUTE_HEAD_LEN: usize = Self::LONG_ATTRIBUTE_HEAD.len();

    const ATTRIBUTE_TAIL: &'static [u8] = b"(runtime)]"; // TODO - add test that value in parenthesis this is equal to `code_reload_core::constants::RUNTIME_TARGET_KEYWORD`
    const ATTRIBUTE_TAIL_LEN: usize = Self::ATTRIBUTE_TAIL.len();

    fn extract(&self, file_path: &Path, byte_str: &[u8]) -> Vec<BuildFnData> {
        let line_indices = LazyCell::new(|| {
            iter::once(0)
                .chain(memchr::memchr_iter(b'\n', byte_str).map(|x| x + 1))
                .collect::<Vec<_>>()
        });

        let attribute_body_indices: Vec<_> =
            memchr::memmem::find_iter(&byte_str, Self::ATTRIBUTE_BODY).collect();
        let fn_syntaxes: Vec<_> = attribute_body_indices
            .iter()
            .filter_map(|attribute_body_index| {
                let Some(attribute_borders) =
                    self.try_get_attribute_borders(&byte_str, *attribute_body_index)
                else {
                    return None;
                };
                let fn_syntax_byte_str = self
                    .get_fn_byte_substr(&byte_str[attribute_borders.end_index..])
                    .unwrap();
                let fn_syntax_byte_str = str::from_utf8(fn_syntax_byte_str).unwrap();
                let item_fn = syn::parse_str(fn_syntax_byte_str).unwrap();

                let line_index =
                    self.get_line_index(attribute_borders.start_index + 1, &line_indices);
                let column = attribute_borders.start_index + 1 - line_indices[line_index];
                let source_code_id = SourceCodeId::new(file_path, line_index + 1, column);
                let build_fn_data = self.item_fn_mapper.map(item_fn, source_code_id);
                return Some(build_fn_data);
            })
            .collect();

        return fn_syntaxes;
    }

    fn try_get_attribute_borders(
        &self,
        byte_str: &[u8],
        attribute_body_index: usize,
    ) -> Option<AttributeBorders> {
        let start_index = self.try_get_attribute_start_index(byte_str, attribute_body_index)?;
        let end_index = self.try_get_attribute_end_index(byte_str, attribute_body_index)?;

        return Some(AttributeBorders {
            start_index,
            end_index,
        });
    }

    fn get_fn_byte_substr<'a>(&self, byte_str: &'a [u8]) -> Option<&'a [u8]> {
        let mut byte_indices = byte_str.iter().enumerate();
        if byte_indices.find(|(_, char)| **char == b'{').is_none() {
            return None;
        }

        let mut brackets_count = 1;
        for (index, char) in byte_indices {
            match *char {
                b'{' => brackets_count += 1,
                b'}' => {
                    brackets_count -= 1;
                    if brackets_count == 0 {
                        return Some(&byte_str[0..=index]);
                    }
                }
                _ => (),
            };
        }

        return None;
    }

    fn get_line_index(&self, fn_syntax_start_index: usize, line_indices: &Vec<usize>) -> usize {
        return line_indices
            .binary_search(&fn_syntax_start_index)
            .unwrap_or_else(|x| x - 1);
    }

    fn try_get_attribute_start_index(
        &self,
        byte_str: &[u8],
        attribute_body_index: usize,
    ) -> Option<usize> {
        if attribute_body_index < Self::SHORT_ATTRIBUTE_HEAD_LEN {
            return None;
        }
        let short_start = attribute_body_index - Self::SHORT_ATTRIBUTE_HEAD_LEN;
        if &byte_str[short_start..attribute_body_index] == Self::SHORT_ATTRIBUTE_HEAD {
            return Some(short_start);
        }
        if attribute_body_index < Self::LONG_ATTRIBUTE_HEAD_LEN {
            return None;
        }
        let long_start = attribute_body_index - Self::LONG_ATTRIBUTE_HEAD_LEN;
        if &byte_str[long_start..attribute_body_index] == Self::LONG_ATTRIBUTE_HEAD {
            return Some(long_start);
        }

        return None;
    }

    fn try_get_attribute_end_index(
        &self,
        byte_str: &[u8],
        attribute_body_index: usize,
    ) -> Option<usize> {
        let start = attribute_body_index + Self::ATTRIBUTE_BODY_LEN;
        let remaining_bytes_count = byte_str.len() - start;
        if remaining_bytes_count < Self::ATTRIBUTE_TAIL_LEN {
            return None;
        }
        let end = start + Self::ATTRIBUTE_TAIL_LEN;
        if &byte_str[start..end] == Self::ATTRIBUTE_TAIL {
            return Some(end);
        }

        return None;
    }
}

struct AttributeBorders {
    pub start_index: usize,
    pub end_index: usize,
}
