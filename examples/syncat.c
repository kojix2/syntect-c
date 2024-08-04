#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <getopt.h> // for getopt_long
#include "../syntect.h"

void check_error(const char *function, const char *error)
{
    if (error != NULL)
    {
        fprintf(stderr, "%s error: %s\n", function, error);
        syntect_free_string((char *)error);
        exit(1);
    }
}

void list_themes()
{
    const char *error = NULL;
    SyntectThemeSet *theme_set = syntect_load_default_theme_set(&error);
    check_error("load_default_theme_set", error);

    const char **theme_names = (const char **)syntect_get_theme_names(theme_set, &error);
    check_error("get_theme_names", error);

    printf("Available themes:\n");
    for (size_t i = 0; theme_names[i] != NULL; i++)
    {
        printf("%s\n", theme_names[i]);
    }

    syntect_free_theme_names((char **)theme_names);
    syntect_free_theme_set(theme_set);
}

void highlight_file(const char *filename, const char *theme_name, const char *theme_path)
{
    const char *error = NULL;
    SyntectFile *wrapper = NULL;

    if (theme_path != NULL)
    {
        SyntectTheme *theme = syntect_load_theme(theme_path, 1, &error);
        check_error("load_theme", error);
        wrapper = syntect_create_file_with_theme(filename, theme, &error);
        syntect_free_theme(theme);
    }
    else
    {
        wrapper = syntect_create_file(filename, theme_name, &error);
    }

    check_error("create_highlight_file", error);

    const char *line = NULL;
    while ((line = syntect_highlight_file_line(wrapper, &error)) != NULL)
    {
        printf("%s", line);
        syntect_free_string((char *)line);
    }

    syntect_free_file(wrapper);
}

void print_help(const char *program_name)
{
    printf("Usage: %s [OPTIONS] FILES...\n", program_name);
    printf("Options:\n");
    printf("  -t, --theme THEME_NAME   Specify the theme to use for highlighting (default: base16-ocean.dark)\n");
    printf("  -p, --theme-path PATH    Specify the path to a theme file to use for highlighting\n");
    printf("  -l, --list-themes        List all available themes\n");
    printf("  -h, --help               Display this help message\n");
}

int main(int argc, char *argv[])
{
    int opt;
    int list_themes_flag = 0;
    const char *theme_name = "base16-ocean.dark";
    const char *theme_path = NULL;

    static struct option long_options[] = {
        {"theme", required_argument, 0, 't'},
        {"theme-path", required_argument, 0, 'p'},
        {"list-themes", no_argument, 0, 'l'},
        {"help", no_argument, 0, 'h'},
        {0, 0, 0, 0}};

    // Parse command line options
    while ((opt = getopt_long(argc, argv, "t:p:lh", long_options, NULL)) != -1)
    {
        switch (opt)
        {
        case 't':
            theme_name = optarg;
            break;
        case 'p':
            theme_path = optarg;
            break;
        case 'l':
            list_themes_flag = 1;
            break;
        case 'h':
            print_help(argv[0]);
            return 0;
        default:
            print_help(argv[0]);
            return 1;
        }
    }

    if (list_themes_flag)
    {
        list_themes();
        return 0;
    }

    if (optind >= argc)
    {
        fprintf(stderr, "Expected argument after options\n");
        print_help(argv[0]);
        return 1;
    }

    for (int i = optind; i < argc; i++)
    {
        highlight_file(argv[i], theme_name, theme_path);
    }

    return 0;
}
