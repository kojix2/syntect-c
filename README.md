# syntect-c

[![test](https://github.com/kojix2/syntect-c/actions/workflows/test.yml/badge.svg)](https://github.com/kojix2/syntect-c/actions/workflows/test.yml)

Provides a C API for [syntect](https://github.com/trishume/syntect), enabling its use from various programming languages.

## Installation

Download binaries from [GitHub Releases](https://github.com/kojix2/syntect-c/releases).

From source code:

```sh
git clone https://github.com/kojix2/syntect-c
cd syntect-c
cargo build --release
# target/release/libsyntect_c.so (Linux), libsyntect_c.dylib (macOS), syntect_c.dll (Windows)
```

### C API

```c
```

Example:

```c
```

## Development

### Running Tests

To run tests written in Rust:

```sh
cargo test
```

To run tests in C:

```sh
cd test && ./test.sh
```

## License

MIT
