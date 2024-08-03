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

void highlight_file(const char *filename) {
    const char *error = NULL;
    HighlightFileWrapper *wrapper = create_highlight_file(filename, "base16-ocean.dark", &error);
    check_error("create_highlight_file", error);

    const char *line = NULL;
    while ((line = highlight_file_line(wrapper, &error)) != NULL) {
        printf("%s", line);
        free_string((char*)line);
    }

    free_highlight_file(wrapper);
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
