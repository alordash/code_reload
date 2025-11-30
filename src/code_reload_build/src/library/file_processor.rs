use crate::IItemFnMapper;
use crate::runtime::models::BuildFnData;
use code_reload_core::SourceCodeId;
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

    const IMPL_BLOCK_START: &'static [u8] = b"impl ";
    const IMPL_BLOCK_START_LEN: usize = Self::IMPL_BLOCK_START.len();

    fn extract(&self, file_path: &Path, byte_str: &[u8]) -> Vec<BuildFnData> {
        let line_indices = LazyCell::new(|| {
            iter::once(0)
                .chain(memchr::memchr_iter(b'\n', byte_str).map(|x| x + 1))
                .collect::<Vec<_>>()
        });

        let attribute_body_indices: Vec<_> =
            memchr::memmem::find_iter(&byte_str, Self::ATTRIBUTE_BODY).collect();
        if attribute_body_indices.is_empty() {
            return Vec::new();
        }
        let relative_file_path = SourceCodeId::get_source_code_relative_file_path(file_path);
        let fn_syntaxes: Vec<_> = attribute_body_indices
            .iter()
            .filter_map(|attribute_body_index| {
                let Some(attribute_borders) =
                    self.try_get_attribute_borders(&byte_str, *attribute_body_index)
                else {
                    return None;
                };
                let fn_syntax_byte_str_end = attribute_borders.end_index
                    + self
                        .get_last_closing_bracket_index(
                            &byte_str[attribute_borders.end_index..],
                            b'{',
                            b'}',
                        )
                        .unwrap();
                let fn_syntax_byte_str =
                    &byte_str[attribute_borders.end_index..=fn_syntax_byte_str_end];
                let fn_syntax_str = str::from_utf8(fn_syntax_byte_str).unwrap();
                // println!("fn_syntax_str: {}", fn_syntax_str);
                let item_fn = syn::parse_str(fn_syntax_str).unwrap();

                let line_index =
                    self.get_line_index(attribute_borders.start_index + 1, &line_indices);
                let column = attribute_borders.start_index + 1 - line_indices[line_index];
                let source_code_id =
                    SourceCodeId::new(relative_file_path.clone(), line_index + 1, column);

                let maybe_impl_block_type =
                    self.try_get_impl_block_type(&byte_str[..attribute_borders.start_index]);

                let build_fn_data =
                    self.item_fn_mapper
                        .map(item_fn, source_code_id, maybe_impl_block_type);
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

    fn get_last_closing_bracket_index(
        &self,
        byte_str: &[u8],
        open_bracket: u8,
        close_bracket: u8,
    ) -> Option<usize> {
        let mut byte_indices = byte_str.iter().enumerate();
        if byte_indices
            .find(|(_, char)| **char == open_bracket)
            .is_none()
        {
            return None;
        }

        let mut brackets_count = 1;
        for (index, char) in byte_indices {
            match *char {
                x if x == open_bracket => brackets_count += 1,
                x if x == close_bracket => {
                    brackets_count -= 1;
                    if brackets_count == 0 {
                        return Some(index);
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

    fn try_get_impl_block_type<'a>(&self, byte_str: &'a [u8]) -> Option<&'a [u8]> {
        let Some(impl_block_start_index) = memchr::memmem::rfind(byte_str, Self::IMPL_BLOCK_START)
        else {
            return None;
        };

        let brackets_count = self.count_brackets(&byte_str[impl_block_start_index..]);

        if brackets_count < 1 {
            return None;
        }

        // let str = str::from_utf8(&byte_str[impl_block_start_index..]).unwrap();
        let impl_block_type = self.get_impl_block_type(&byte_str[impl_block_start_index..]);

        return Some(impl_block_type);
    }

    fn get_impl_block_type<'a>(&self, byte_str: &'a [u8]) -> &'a [u8] {
        let open_bracket_index = memchr::memchr(b'{', byte_str).unwrap();
        let mut type_start_index = self
            .get_last_closing_bracket_index(&byte_str[..open_bracket_index], b'<', b'>')
            .unwrap_or(Self::IMPL_BLOCK_START_LEN);
        while byte_str[type_start_index] == b' ' {
            type_start_index += 1;
        }
        let type_end_index =
            type_start_index + memchr::memchr(b' ', &byte_str[type_start_index..]).unwrap();

        return &byte_str[type_start_index..type_end_index];
    }

    fn count_brackets(&self, byte_str: &[u8]) -> isize {
        let mut brackets_count = 0;
        for char in byte_str {
            match *char {
                b'{' => brackets_count += 1,
                b'}' => brackets_count -= 1,
                _ => (),
            }
        }
        return brackets_count;
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
