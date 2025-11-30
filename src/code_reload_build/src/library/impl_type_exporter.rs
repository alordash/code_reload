use crate::runtime::models::BuildFnData;
use code_reload_core::constants;
use std::io::Write;

pub trait IImplTypeExporter {
    fn export(&self, all_build_fn_datas: &[BuildFnData]);
}

pub struct ImplTypeExporter;

// TODO - need to export fool path to module and underlying impl block type (if exists) always
impl IImplTypeExporter for ImplTypeExporter {
    fn export(&self, all_build_fn_datas: &[BuildFnData]) {
        let mut build_fn_datas = all_build_fn_datas
            .iter()
            .filter(|x| x.impl_block_type().is_some())
            .peekable();
        if build_fn_datas.peek().is_none() {
            return;
        }
        self.prepare_dir();

        for build_fn_data in build_fn_datas {
            self.export(build_fn_data)
        }
    }
}

impl ImplTypeExporter {
    fn prepare_dir(&self) {
        let dir = &*constants::IMPL_BLOCK_TYPES_DIR;
        std::fs::create_dir_all(dir).unwrap();
    }

    fn export(&self, build_fn_data: &BuildFnData) {
        let Some(impl_block_type) = build_fn_data.impl_block_type() else {
            return;
        };

        let path = build_fn_data.source_code_id().get_path();
        let file_path = constants::IMPL_BLOCK_TYPES_DIR.join(path);
        let file_dir_path = file_path.parent().unwrap();
        std::fs::create_dir_all(file_dir_path).unwrap();

        // log!("writing impl block type to: '{:?}'", file_path);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        file.write(impl_block_type).unwrap();
    }
}
