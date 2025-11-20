use code_reload_runtime::models::FnData;

pub trait IOutputGenerator {
    fn generate(&self, fn_datas: Vec<FnData>) -> String;
}

pub struct OutputGenerator;

impl IOutputGenerator for OutputGenerator {
    fn generate(&self, fn_datas: Vec<FnData>) -> String {
        todo!()
    }
}

impl OutputGenerator {
    fn generate_payload_fields(&self, fn_datas: &Vec<FnData>) -> String {
        // let payload_fields = fn_datas.iter().map()
        todo!()
    }
}