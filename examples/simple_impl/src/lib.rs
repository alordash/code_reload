use code_reload::hotreload;

#[derive(Debug)]
pub struct Model {
    pub number: i32,
}

impl Model {
    #[hotreload]
    pub fn new() -> Self {
        Self { number: 1 }
    }

    #[hotreload]
    pub fn number(&self) -> i32 {
        self.number + 7
    }

    #[hotreload]
    pub fn mutate(&mut self) {
        self.number += 3;
    }
}
