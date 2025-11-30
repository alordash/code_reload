use proc_macro2::Ident;
use std::path::PathBuf;

pub trait IErrorFormatter {
    fn get_library_opening_error_format(&self, dynamic_library_path: &PathBuf) -> String;
    fn get_symbol_search_error_format(
        &self,
        dynamic_library_path: &PathBuf,
        source_function_export_name: &Ident,
    ) -> String;
}

pub struct ErrorFormatter;

impl IErrorFormatter for ErrorFormatter {
    fn get_library_opening_error_format(&self, dynamic_library_path: &PathBuf) -> String {
        let dynamic_library_path_string = dynamic_library_path.to_str().unwrap();
        let library_opening_error_format =
            format!("Error opening shared library '{dynamic_library_path_string}' in directory '{{:?}}': {{e:?}}");

        return library_opening_error_format;
    }

    fn get_symbol_search_error_format(
        &self,
        dynamic_library_path: &PathBuf,
        source_function_export_name: &Ident,
    ) -> String {
        let dynamic_library_path_string = dynamic_library_path.to_str().unwrap();
        let symbol_search_error_format = format!(
            "Error finding symbol '{source_function_export_name}' in shared library '{dynamic_library_path_string}': {{e:?}}"
        );

        return symbol_search_error_format;
    }
}
