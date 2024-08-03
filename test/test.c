#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../syntect.h"

/**
 * @brief Checks for errors and prints the error message if any.
 *
 * This function checks if an error message is present, prints it, frees the error string,
 * and exits the program with a failure status.
 *
 * @param function The name of the function where the error occurred.
 * @param error The error message to check.
 */
void check_error(const char *function, const char *error) {
    if (error != NULL) {
        fprintf(stderr, "%s error: %s\n", function, error);
        syntect_free_string((char*)error);
        exit(1);
    }
}

/**
 * @brief Tests the creation of a SyntectFile and highlights its content.
 *
 * This function tests the creation of a SyntectFile for the given file path and theme,
 * reads and highlights each line of the file, and prints the highlighted content.
 */
void test_create_highlight_file() {
    const char *path = "hello_world.c";
    const char *theme_name = "base16-ocean.dark";
    const char *error = NULL;

    SyntectFile *wrapper = syntect_create_file(path, theme_name, &error);
    check_error("syntect_create_file", error);

    if (wrapper == NULL) {
        fprintf(stderr, "Failed to create SyntectFile\n");
        exit(1);
    }
    printf("Highlight file: %s\n", path);

    const char *line = NULL;
    while ((line = syntect_highlight_file_line(wrapper, &error)) != NULL) {
        printf("%s", line);
        syntect_free_string((char*)line);
    }

    syntect_free_file(wrapper);
}

/**
 * @brief Tests the creation and freeing of a SyntectLines.
 *
 * This function tests the creation of a SyntectLines for the given theme
 * and ensures it can be properly freed without errors.
 */
void test_create_highlight_lines() {
    const char *theme_name = "base16-ocean.dark";
    const char *error = NULL;

    SyntectLines *wrapper = syntect_create_lines(theme_name, &error);
    check_error("syntect_create_lines", error);

    if (wrapper == NULL) {
        fprintf(stderr, "Failed to create SyntectLines\n");
        exit(1);
    }

    syntect_free_lines(wrapper);
}

/**
 * @brief Tests highlighting a single line of text using SyntectLines.
 *
 * This function tests the creation of a SyntectLines for the given theme,
 * highlights a single line of text, prints the highlighted line, and frees the memory.
 */
void test_highlight_text_line() {
    const char *theme_name = "base16-ocean.dark";
    const char *error = NULL;

    SyntectLines *wrapper = syntect_create_lines(theme_name, &error);
    check_error("syntect_create_lines", error);

    const char *line = "fn main() { println!(\"Hello, world!\"); }";
    const char *highlighted_line = syntect_highlight_text_line(wrapper, line, &error);
    check_error("syntect_highlight_text_line", error);

    if (highlighted_line == NULL) {
        fprintf(stderr, "Failed to highlight line\n");
        exit(1);
    }

    printf("Highlighted line: %s\n", highlighted_line);
    syntect_free_string((char*)highlighted_line);

    syntect_free_lines(wrapper);
}

/**
 * @brief Main function to run all tests.
 *
 * This function runs all defined test functions and prints a success message if all tests pass.
 *
 * @return int Returns 0 on success, exits with failure status otherwise.
 */
int main() {
    test_create_highlight_file();
    test_create_highlight_lines();
    test_highlight_text_line();

    printf("All tests passed!\n");
    return 0;
}
