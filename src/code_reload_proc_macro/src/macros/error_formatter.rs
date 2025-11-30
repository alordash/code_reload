use proc_macro2::Ident;

pub trait IErrorFormatter {
    fn get_library_opening_error_format(&self, dynamic_library_filename: &str) -> String;
    fn get_symbol_search_error_format(
        &self,
        dynamic_library_filename: &str,
        source_function_export_name: &Ident,
    ) -> String;
}

pub struct ErrorFormatter;

impl IErrorFormatter for ErrorFormatter {
    fn get_library_opening_error_format(&self, dynamic_library_filename: &str) -> String {
        let library_opening_error_format =
            format!("Error opening shared library '{dynamic_library_filename}': {{e:?}}");

        return library_opening_error_format;
    }

    fn get_symbol_search_error_format(
        &self,
        dynamic_library_filename: &str,
        source_function_export_name: &Ident,
    ) -> String {
        let symbol_search_error_format = format!(
            "Error finding symbol '{source_function_export_name}' in shared library '{dynamic_library_filename}': {{e:?}}"
        );

        return symbol_search_error_format;
    }
}
