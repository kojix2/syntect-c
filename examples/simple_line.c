#include <stdio.h>
#include <stdlib.h>
#include <string.h>
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

void highlight_line(const char *line, const char *theme_name)
{
  const char *error = NULL;
  SyntectLines *wrapper = syntect_create_lines(theme_name, &error);
  check_error("syntect_create_lines", error);

  const char *highlighted_line = syntect_highlight_text_line(wrapper, line, &error);
  check_error("syntect_highlight_text_line", error);

  printf("Highlighted line: %s\n", highlighted_line);
  syntect_free_string((char *)highlighted_line);
  syntect_free_lines(wrapper);
}

int main()
{
  const char *line = "fn main() { println!(\"Hello, world!\"); }";
  highlight_line(line, "base16-ocean.dark");

  return 0;
}