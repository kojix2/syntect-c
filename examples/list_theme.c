#include <stdio.h>
#include <stdlib.h>
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

  const char **theme_names = syntect_get_theme_names(theme_set, &error);
  check_error("get_theme_names", error);

  printf("Available themes:\n");
  for (size_t i = 0; theme_names[i] != NULL; i++)
  {
    printf("- %s\n", theme_names[i]);
  }

  syntect_free_theme_names((char **)theme_names);
  syntect_free_theme_set(theme_set);
}

int main()
{
  list_themes();
  return 0;
}