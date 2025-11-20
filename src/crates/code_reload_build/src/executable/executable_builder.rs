pub trait IExecutableBuilder {
    fn build(&self);
}

pub struct ExecutableBuilder;

impl IExecutableBuilder for ExecutableBuilder {
    fn build(&self) {
        todo!()
    }
}