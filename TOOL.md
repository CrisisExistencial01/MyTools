# 🔍 filter
A simple filter similar to `grep`, but in another color 🟢  
And written in Rust 🦀

## Usage
```bash
<cmd> | filter <pattern>
```

## Example
```bash
ls -l | filter txt
```

## Why another filter?
I was learning Rust, and I realized I rarely used most of `grep`'s options except for simple filtering, so I made this simple tool.  
And I wanted a filter that highlights matches in **green** 🟢

## Features

- Simple and lightweight.
- Highlights matched lines in green.
- Easy to integrate in shell pipelines.

## Build and Install
If you have Rust installed, you can build it with:
```bash
cargo build --release
```

Then you can find the binary in `target/release/filter`.
You can move it to a directory in your PATH, for example:
```bash
mv target/release/filter /usr/local/bin/
```
## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

