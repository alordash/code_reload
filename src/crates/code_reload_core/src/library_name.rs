pub fn create(library_name: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        return format!("{}.dll", library_name);
    }
    #[cfg(target_os = "linux")]
    {
        return format!("lib{}.so", library_name);
    }
    panic!("OS '{}' is not supported.", std::env::consts::OS);
}
