#[cfg(windows)]
compile_error!("Tests only work with unix paths!");

use {include_dir::*, std::fs};

const OUT_FILE: &str = "test_files_content.rs";
const COMPARE: &str = r#"
const LUA_PRELOAD: &[&[u8]] = &[
    include_bytes!("tests/dir/a.lua"),
    include_bytes!("tests/dir/sub/b.lua"),
];
"#;

#[test]
fn main() {
    include_dir("tests/dir", OUT_FILE, Data::Content, "LUA_PRELOAD", true).unwrap();

    assert_eq!(fs::read_to_string(OUT_FILE).unwrap().trim(), COMPARE.trim());
}
