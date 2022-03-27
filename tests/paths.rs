#[cfg(windows)]
compile_error!("Tests only work with unix paths!");

use {include_dir::*, std::fs};

const OUT_FILE: &str = "test_files_paths.rs";
const COMPARE: &str = r#"
const LUA_PRELOAD: &[&str] = &[
    "tests/dir/a.lua",
    "tests/dir/sub/b.lua",
];
"#;

#[test]
fn main() {
    include_dir("tests/dir", OUT_FILE, Data::Paths, "LUA_PRELOAD", true).unwrap();

    assert_eq!(fs::read_to_string(OUT_FILE).unwrap().trim(), COMPARE.trim());
}
