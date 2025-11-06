use code_reload::hotreload;

include!(concat!(env!("OUT_DIR"), "/__code_reload_hotreload.rs"));

#[derive(Debug)]
pub struct Data {
    number: i32,
    string: String,
}

// TODO - can not use `runtime` in `impl`s (`Self` get's treated as HotreloadPayload and not target type)
impl Data {
    #[hotreload]
    pub fn new() -> Self {
        Self {
            number: 1,
            string: String::from("quo vadis"),
        }
    }

    #[hotreload]
    pub fn string(&self) -> &str {
        return &self.string[1..6];

        // TODO - check if this method will be publicly available in generated DLL
        #[unsafe(export_name = "__scode_reload_string")]
        #[doc(hidden)]
        fn __scode_reload_string(_self: &Data) -> &str {
            &_self.string[1..4]
        }
    }
}

// TODO - can not use just `Data`, generated module doesn't know about this type
#[hotreload(runtime)]
pub fn mutate(data: &mut crate::Data) {
    std::thread::sleep(std::time::Duration::from_millis(100));
    data.number += 121;
    if data.number > 1000 {
        data.number *= -1;
    }
    data.string += " susssssas";
    if data.string.len() > 30 {
        data.string = data.string[30..].to_string();
    }
}

pub trait IGetNumber {
    fn get_number(&self) -> i32;
}

// TODO - can not use `hotreload` in trait `impl` blocks (extra function is not a part of trait)
impl IGetNumber for Data {
    // #[hotreload]
    fn get_number(&self) -> i32 {
        self.number
    }
}
