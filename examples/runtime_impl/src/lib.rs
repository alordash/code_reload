use code_reload::hotreload;

code_reload::runtime::add_runtime!();

#[derive(Debug)]
pub struct Model {
    pub number: i32,
}

impl Model {
    #[hotreload(runtime)]
    pub fn new() -> Self {
        Self { number: 3 }
    }

    #[hotreload(runtime)]
    pub fn number(&self) -> i32 {
        self.number + 1
    }

    #[hotreload(runtime)]
    pub fn mutate(&mut self) {
        self.number += 5;
    }
}
