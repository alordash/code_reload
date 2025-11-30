use code_reload::hotreload;

pub struct Model;
trait Trait {
    fn create() -> Self;

    fn read(&self) -> i32;

    fn write(&mut self, number: i32);
}

// TODO - not supported (generated functions are not part of trait)
impl Trait for Model {
    #[hotreload]
    fn create() -> Self {
        todo!()
    }

    #[hotreload]
    fn read(&self) -> i32 {
        todo!()
    }

    #[hotreload]
    fn write(&mut self, number: i32) {
        todo!()
    }
}
