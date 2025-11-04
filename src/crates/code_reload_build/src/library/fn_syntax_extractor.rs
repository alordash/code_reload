use syn::ItemFn;

pub trait IFnSyntaxExtractor {
    fn extract(&self, byte_str: &[u8]) -> Vec<ItemFn>;
}

pub struct FnSyntaxExtractor;

impl IFnSyntaxExtractor for FnSyntaxExtractor {
    fn extract(&self, byte_str: &[u8]) -> Vec<ItemFn> {
        let fn_syntaxes: Vec<_> = memchr::memmem::find_iter(&byte_str, Self::ATTRIBUTE_TAIL)
            .filter(|attribute_tail_index| {
                self.is_attribute_tail_valid(&byte_str, *attribute_tail_index)
            })
            .map(|attribute_tail_index| attribute_tail_index + Self::ATTRIBUTE_TAIL_LEN)
            .map(|fn_syntax_start_index| {
                self.get_fn_byte_substr(&byte_str[fn_syntax_start_index..])
            })
            .map(Option::unwrap)
            .map(|fn_syntax_byte_str| str::from_utf8(fn_syntax_byte_str))
            .map(Result::unwrap)
            .map(syn::parse_str)
            .map(Result::unwrap)
            .collect();

        return fn_syntaxes;
    }
}

impl FnSyntaxExtractor {
    const ATTRIBUTE_TAIL: &'static [u8] = b"hotreload]";
    const ATTRIBUTE_TAIL_LEN: usize = Self::ATTRIBUTE_TAIL.len();

    const SHORT_ATTRIBUTE_HEAD: &'static [u8] = b"#[";
    const SHORT_ATTRIBUTE_HEAD_LEN: usize = Self::SHORT_ATTRIBUTE_HEAD.len();
    const LONG_ATTRIBUTE_HEAD: &'static [u8] = b"#[code_reload::";
    const LONG_ATTRIBUTE_HEAD_LEN: usize = Self::LONG_ATTRIBUTE_HEAD.len();

    fn is_attribute_tail_valid(&self, byte_str: &[u8], attribute_tail_index: usize) -> bool {
        if attribute_tail_index < Self::SHORT_ATTRIBUTE_HEAD_LEN {
            return false;
        }
        let short_start = attribute_tail_index - Self::SHORT_ATTRIBUTE_HEAD_LEN;
        if &byte_str[short_start..attribute_tail_index] == Self::SHORT_ATTRIBUTE_HEAD {
            return true;
        }
        if attribute_tail_index < Self::LONG_ATTRIBUTE_HEAD_LEN {
            return false;
        }
        let long_start = attribute_tail_index - Self::LONG_ATTRIBUTE_HEAD_LEN;
        if &byte_str[long_start..attribute_tail_index] == Self::LONG_ATTRIBUTE_HEAD {
            return true;
        }

        return false;
    }

    fn get_fn_byte_substr<'a>(&self, byte_str: &'a [u8]) -> Option<&'a [u8]> {
        let str = str::from_utf8(byte_str).unwrap();
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
                        let result_str = str::from_utf8(&byte_str[0..=index]);
                        return Some(&byte_str[0..=index]);
                    }
                }
                _ => (),
            };
        }

        return None;
    }
}
