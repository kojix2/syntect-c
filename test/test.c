#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../syntect.h"

void check_error(const char *function, const char *error) {
    if (error != NULL) {
        fprintf(stderr, "%s error: %s\n", function, error);
        free_string((char*)error);
        exit(1);
    }
}

void test_create_highlight_file() {
    const char *path = "hello_world.c";
    const char *theme_name = "base16-ocean.dark";
    const char *error = NULL;

    struct HighlightFileWrapper *wrapper = create_highlight_file(path, theme_name, &error);
    check_error("create_highlight_file", error);

    if (wrapper == NULL) {
        fprintf(stderr, "Failed to create HighlightFileWrapper\n");
        exit(1);
    }
    printf("Highlight file: %s\n", path);
    // output the highlighted file
    const char *line = NULL;
    while ((line = highlight_file_line(wrapper, &error)) != NULL) {
        printf("%s", line);
        free_string((char*)line);
    }

    free_highlight_file(wrapper);
}

void test_create_highlight_lines() {
    const char *theme_name = "base16-ocean.dark";
    const char *error = NULL;

    struct HighlightLinesWrapper *wrapper = create_highlight_lines(theme_name, &error);
    check_error("create_highlight_lines", error);

    if (wrapper == NULL) {
        fprintf(stderr, "Failed to create HighlightLinesWrapper\n");
        exit(1);
    }

    free_highlight_lines(wrapper);
}

void test_highlight_text_line() {
    const char *theme_name = "base16-ocean.dark";
    const char *error = NULL;

    struct HighlightLinesWrapper *wrapper = create_highlight_lines(theme_name, &error);
    check_error("create_highlight_lines", error);

    const char *line = "fn main() { println!(\"Hello, world!\"); }";
    const char *highlighted_line = highlight_text_line(wrapper, line, &error);
    check_error("highlight_text_line", error);

    if (highlighted_line == NULL) {
        fprintf(stderr, "Failed to highlight line\n");
        exit(1);
    }

    printf("Highlighted line: %s\n", highlighted_line);
    free_string((char*)highlighted_line);

    free_highlight_lines(wrapper);
}

int main() {
    test_create_highlight_file();
    test_create_highlight_lines();
    test_highlight_text_line();

    printf("All tests passed!\n");
    return 0;
}
