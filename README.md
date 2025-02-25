# syntect-c

[![test](https://github.com/kojix2/syntect-c/actions/workflows/test.yml/badge.svg)](https://github.com/kojix2/syntect-c/actions/workflows/test.yml)

`syntect-c` provides a C API for the [syntect](https://github.com/trishume/syntect) library, enabling its use from various programming languages.

## Installation

### Download Binaries

You can download the pre-built binaries from [GitHub Releases](https://github.com/kojix2/syntect-c/releases).

### Build from Source

To build from source, follow these steps:

```sh
git clone https://github.com/kojix2/syntect-c
cd syntect-c
cargo build --release
# Binaries will be located in target/release/
# libsyntect_c.so (Linux), libsyntect_c.dylib (macOS), syntect_c.dll (Windows)
```

## Usage

### C API

The following functions are provided in the C API:

```c
SyntectFile* syntect_create_file(const char* path, const char* theme_name, const char** error);
SyntectFile* syntect_create_file_with_theme(const char* path, const SyntectTheme* theme, const char** error);
const char* syntect_highlight_file_line(SyntectFile* wrapper, const char** error);
void syntect_free_file(SyntectFile* wrapper);

SyntectLines* syntect_create_lines(const char* theme_name, const char** error);
SyntectLines* syntect_create_lines_with_theme(const SyntectTheme* theme, const char** error);
const char* syntect_highlight_text_line(SyntectLines* wrapper, const char* line, const char** error);
void syntect_free_lines(SyntectLines* wrapper);

SyntectThemeSet* syntect_load_default_theme_set(const char** error);
SyntectThemeSet* syntect_load_theme_set_from_folder(const char* folder, const char** error);
const char** syntect_get_theme_names(const SyntectThemeSet* theme_set, const char** error);
size_t syntect_get_theme_count(const char** theme_names);
void syntect_free_theme_names(char** theme_names);

SyntectTheme* syntect_load_theme(const char* theme_path, int enable_caching, const char** error);
SyntectTheme* syntect_get_theme_from_set(SyntectThemeSet* theme_set, const char* theme_name, const char** error);
void syntect_free_theme_set(SyntectThemeSet* theme_set);
void syntect_free_theme(SyntectTheme* theme);
const char* syntect_get_theme_name(const SyntectTheme* theme);

void syntect_free_string(char* s);
```

### Example

#### Highlighting a file:

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "syntect.h"

void check_error(const char *function, const char *error) {
    if (error != NULL) {
        fprintf(stderr, "%s error: %s\n", function, error);
        syntect_free_string((char*)error);
        exit(1);
    }
}

void highlight_file(const char *filename) {
    const char *error = NULL;
    SyntectFile *wrapper = syntect_create_file(filename, "base16-ocean.dark", &error);
    check_error("syntect_create_file", error);

    const char *line = NULL;
    while ((line = syntect_highlight_file_line(wrapper, &error)) != NULL) {
        printf("%s", line);
        syntect_free_string((char*)line);
    }

    syntect_free_file(wrapper);
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Please provide some files to highlight.\n");
        return 1;
    }

    for (int i = 1; i < argc; i++) {
        highlight_file(argv[i]);
    }

    return 0;
}
```

#### Highlighting a single line of text:

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "syntect.h"

void check_error(const char *function, const char *error) {
    if (error != NULL) {
        fprintf(stderr, "%s error: %s\n", function, error);
        syntect_free_string((char*)error);
        exit(1);
    }
}

void highlight_line(const char *line, const char *theme_name) {
    const char *error = NULL;
    SyntectLines *wrapper = syntect_create_lines(theme_name, &error);
    check_error("syntect_create_lines", error);

    const char *highlighted_line = syntect_highlight_text_line(wrapper, line, &error);
    check_error("syntect_highlight_text_line", error);

    printf("Highlighted line: %s\n", highlighted_line);
    syntect_free_string((char*)highlighted_line);
    syntect_free_lines(wrapper);
}

int main() {
    const char *line = "fn main() { println!(\"Hello, world!\"); }";
    highlight_line(line, "base16-ocean.dark");

    return 0;
}
```

#### Listing available themes:

```c
#include <stdio.h>
#include <stdlib.h>
#include "syntect.h"

void check_error(const char *function, const char *error) {
    if (error != NULL) {
        fprintf(stderr, "%s error: %s\n", function, error);
        syntect_free_string((char*)error);
        exit(1);
    }
}

void list_themes() {
    const char *error = NULL;
    SyntectThemeSet *theme_set = syntect_load_default_theme_set(&error);
    check_error("load_default_theme_set", error);

    const char **theme_names = syntect_get_theme_names(theme_set, &error);
    check_error("get_theme_names", error);

    printf("Available themes:\n");
    for (size_t i = 0; theme_names[i] != NULL; i++) {
        printf("%s\n", theme_names[i]);
    }

    syntect_free_theme_names((char**)theme_names);
    syntect_free_theme_set(theme_set);
}

int main() {
    list_themes();
    return 0;
}
```

## Development

All code for this project is generated using ChatGPT.

### Running Tests

To run the tests written in Rust:

```sh
cargo test
```

To run the tests in C:

```sh
cd test && ./test.sh
```

## License

MIT
