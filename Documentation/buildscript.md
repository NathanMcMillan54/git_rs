## Build Script

A Rust build script (build.rs) usually is used for rerunning the cargo compiler when something changes, or for adding 
external files to your project. Most build scripts look something like this:

```rust
extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/my_external_file.c");
    cc::Build::new()
        .file("src/my_external_file.c")
        .compile("external_file");
}
```

But that's just adding a single file to your project, with Git Rs you can add an entire git repository to your project.
After all you'll need to do is link you project with the git repository. Build script and project example:

``build.rs``:
```rust
extern crate git;
use std::env;

fn main() {
    git::GitCommands::clone("myName", "myRepository");
    env::set_current_dir("myRepository");
    git::GitCommands::checkout_t("myRepository", "0.1.0");
}
```

Just a way of linking:
``build.sh``
```shell
#!/bin/bash

cargo build
gcc -static target/debug/libmylib.a myRepository/main.c -o main
```
