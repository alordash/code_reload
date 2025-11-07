fn main() {
    code_reload::runtime::build();
    code_reload::runtime::build_tests();
    code_reload::runtime::build_dir("test_files");
}
