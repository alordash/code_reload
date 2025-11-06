use crate::IItemFnMapper;
use crate::runtime::models::BuildFnData;
use std::sync::Arc;

pub trait IFnSyntaxExtractor {
    fn extract(&self, byte_str: &[u8]) -> Vec<BuildFnData>;
}

pub struct FnSyntaxExtractor {
    pub item_fn_mapper: Arc<dyn IItemFnMapper>,
}

impl IFnSyntaxExtractor for FnSyntaxExtractor {
    fn extract(&self, byte_str: &[u8]) -> Vec<BuildFnData> {
        let fn_syntaxes: Vec<_> = memchr::memmem::find_iter(&byte_str, Self::ATTRIBUTE_BODY)
            .filter_map(|attribute_body_index| {
                self.try_get_fn_syntax_start_index(&byte_str, attribute_body_index)
            })
            .map(|fn_syntax_start_index| {
                self.get_fn_byte_substr(&byte_str[fn_syntax_start_index..])
            })
            .map(Option::unwrap)
            .map(|fn_syntax_byte_str| str::from_utf8(fn_syntax_byte_str))
            .map(Result::unwrap)
            .map(syn::parse_str)
            .map(Result::unwrap)
            .map(|item_fn| self.item_fn_mapper.map(item_fn))
            .collect();

        return fn_syntaxes;
    }
}

impl FnSyntaxExtractor {
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

    fn try_get_fn_syntax_start_index(&self, byte_str: &[u8], attribute_body_index: usize) -> Option<usize> {
        if !self.is_attribute_head_valid(byte_str, attribute_body_index) {
            return None;
        }

        return self.try_get_attribute_end_index(byte_str, attribute_body_index);
    }

    fn is_attribute_head_valid(&self, byte_str: &[u8], attribute_body_index: usize) -> bool {
        if attribute_body_index < Self::SHORT_ATTRIBUTE_HEAD_LEN {
            return false;
        }
        let short_start = attribute_body_index - Self::SHORT_ATTRIBUTE_HEAD_LEN;
        if &byte_str[short_start..attribute_body_index] == Self::SHORT_ATTRIBUTE_HEAD {
            return true;
        }
        if attribute_body_index < Self::LONG_ATTRIBUTE_HEAD_LEN {
            return false;
        }
        let long_start = attribute_body_index - Self::LONG_ATTRIBUTE_HEAD_LEN;
        if &byte_str[long_start..attribute_body_index] == Self::LONG_ATTRIBUTE_HEAD {
            return true;
        }

        return false;
    }

    fn try_get_attribute_end_index(&self, byte_str: &[u8], attribute_body_index: usize) -> Option<usize> {
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
}
