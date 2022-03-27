# include-dir-rs

[![CircleCI](https://circleci.com/gh/olback/include-dir-rs/tree/master.svg?style=svg)](https://circleci.com/gh/olback/include-dir-rs/tree/master)

build.rs example
```rust
fn main() {
    include_dir::include_dir(
        "my_dir", // Dir to include
        "out_file.rs", // Output file
        VarKind::Both, // What data should be included?
        "VAR_NAME",
        true, // Recursive?
    )
    .unwrap();
}
```
