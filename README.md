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

Assuming the following directory structure, the build script would produce the following:

```
my_dir
├── file1.txt
└── sub
    └── file2.txt
```

```rust
const VAR_NAME: &[(&str, &[u8])] = &[
    ("my_dir/file1.txt", include_bytes!("my_dir/file1.txt")),
    ("my_dir/sub/file2.txt", include_bytes!("my_dir/sub/file2.txt")),
];
```
