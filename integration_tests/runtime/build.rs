fn main() {
    code_reload::runtime::build();
    code_reload::runtime::build_dir("custom_dir");
    code_reload::runtime::build_tests();
}
