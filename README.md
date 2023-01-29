# pngme
My implementation of the PNGme intermediate Rust Project

The unit tests and project guide can be found here:
https://picklenerd.github.io/pngme_book/introduction.html

Due to taking commandline arguments, you will need to build an executable before using. Sample steps (run within the pngme folder):
```
cargo build --release
./target/release/pngme decode [file path] [chunk type]
```

Available commands and their arguments can be accessed with the `--help` flag.
