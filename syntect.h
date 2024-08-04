#ifndef SYNTECT_H
#define SYNTECT_H

#ifdef __cplusplus
extern "C"
{
#endif

  /**
   * @brief Wrapper struct for HighlightFile in Rust.
   *
   * This struct is used to manage the highlighting of files.
   */
  typedef struct SyntectFile SyntectFile;

  /**
   * @brief Wrapper struct for HighlightLines in Rust.
   *
   * This struct is used to manage the highlighting of individual lines of text.
   */
  typedef struct SyntectLines SyntectLines;

  /**
   * @brief Wrapper struct for ThemeSet in Rust.
   *
   * This struct is used to manage a set of themes.
   */
  typedef struct SyntectThemeSet SyntectThemeSet;

  /**
   * @brief Wrapper struct for Theme in Rust.
   *
   * This struct is used to represent a theme.
   */
  typedef struct SyntectTheme SyntectTheme;

  /**
   * @brief Creates a SyntectFile for highlighting a file.
   *
   * This function initializes a SyntectFile, which can be used to highlight the content of a file.
   *
   * @param path The path to the file to be highlighted.
   * @param theme_name The name of the theme to be used for highlighting.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the created SyntectFile, or NULL if an error occurs.
   */
  SyntectFile *syntect_create_file(const char *path, const char *theme_name, const char **error);

  /**
   * @brief Creates a SyntectFile for highlighting a file with a specified theme.
   *
   * This function initializes a SyntectFile with the given theme, which can be used to highlight the content of a file.
   *
   * @param path The path to the file to be highlighted.
   * @param theme Pointer to the SyntectTheme to be used for highlighting.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the created SyntectFile, or NULL if an error occurs.
   */
  SyntectFile *syntect_create_file_with_theme(const char *path, const SyntectTheme *theme, const char **error);

  /**
   * @brief Highlights a line from the file.
   *
   * This function reads and highlights a single line from the file associated with the given SyntectFile.
   *
   * @param wrapper Pointer to the SyntectFile.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return The highlighted line as a C string, or NULL if an error occurs. The returned string should be freed using syntect_free_string.
   */
  const char *syntect_highlight_file_line(SyntectFile *wrapper, const char **error);

  /**
   * @brief Creates a SyntectLines for highlighting lines of text with a specified theme.
   *
   * This function initializes a SyntectLines with the given theme, which can be used to highlight individual lines of text.
   *
   * @param theme Pointer to the SyntectTheme to be used for highlighting.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the created SyntectLines, or NULL if an error occurs.
   */
  SyntectLines *syntect_create_lines_with_theme(const SyntectTheme *theme, const char **error);

  /**
   * @brief Frees the SyntectFile.
   *
   * This function frees the memory allocated for the given SyntectFile.
   *
   * @param wrapper Pointer to the SyntectFile to be freed.
   */
  void syntect_free_file(SyntectFile *wrapper);

  /**
   * @brief Creates a SyntectLines for highlighting lines of text.
   *
   * This function initializes a SyntectLines, which can be used to highlight individual lines of text.
   *
   * @param theme_name The name of the theme to be used for highlighting.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the created SyntectLines, or NULL if an error occurs.
   */
  SyntectLines *syntect_create_lines(const char *theme_name, const char **error);

  /**
   * @brief Highlights a line of text.
   *
   * This function highlights a single line of text using the given SyntectLines.
   *
   * @param wrapper Pointer to the SyntectLines.
   * @param line The line of text to be highlighted.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return The highlighted line as a C string, or NULL if an error occurs. The returned string should be freed using syntect_free_string.
   */
  const char *syntect_highlight_text_line(SyntectLines *wrapper, const char *line, const char **error);

  /**
   * @brief Frees the SyntectLines.
   *
   * This function frees the memory allocated for the given SyntectLines.
   *
   * @param wrapper Pointer to the SyntectLines to be freed.
   */
  void syntect_free_lines(SyntectLines *wrapper);

  /**
   * @brief Loads the default ThemeSet.
   *
   * This function loads the default themes provided by syntect.
   *
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the created SyntectThemeSet, or NULL if an error occurs.
   */
  SyntectThemeSet *syntect_load_default_theme_set(const char **error);

  /**
   * @brief Loads a ThemeSet from a folder.
   *
   * This function loads all themes found in the specified folder into a ThemeSet.
   *
   * @param folder The path to the folder containing the theme files.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the created SyntectThemeSet, or NULL if an error occurs.
   */
  SyntectThemeSet *syntect_load_theme_set_from_folder(const char *folder, const char **error);

  /**
   * @brief Gets the names of all themes in a ThemeSet.
   *
   * This function retrieves the names of all themes in the given ThemeSet.
   *
   * @param theme_set Pointer to the SyntectThemeSet.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return A NULL-terminated array of theme names as C strings. The returned array and strings should be freed using syntect_free_theme_names.
   */
  const char **syntect_get_theme_names(const SyntectThemeSet *theme_set, const char **error);

  /**
   * @brief Gets the count of themes in a ThemeSet.
   *
   * This function retrieves the number of themes in the given ThemeSet.
   *
   * @param theme_names A NULL-terminated array of theme names as C strings.
   * @return The number of themes in the ThemeSet.
   */
  size_t syntect_get_theme_count(const char **theme_names);

  /**
   * @brief Frees an array of theme names.
   *
   * This function frees the memory allocated for the array of theme names returned by syntect_get_theme_names.
   *
   * @param theme_names A NULL-terminated array of theme names as C strings to be freed.
   */
  void syntect_free_theme_names(char **theme_names);

  /**
   * @brief Loads a Theme from a file.
   *
   * This function loads a theme from the specified file.
   *
   * @param theme_path The path to the theme file.
   * @param enable_caching Whether to enable caching of the theme.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the created SyntectTheme, or NULL if an error occurs.
   */
  SyntectTheme *syntect_load_theme(const char *theme_path, int enable_caching, const char **error);

  /**
   * @brief Gets a Theme from a ThemeSet.
   *
   * This function retrieves a theme by name from the given ThemeSet.
   *
   * @param theme_set Pointer to the SyntectThemeSet.
   * @param theme_name The name of the theme to retrieve.
   * @param error Pointer to store the error message if the function fails. This should be freed using syntect_free_string.
   * @return Pointer to the SyntectTheme, or NULL if an error occurs.
   */
  SyntectTheme *syntect_get_theme_from_set(SyntectThemeSet *theme_set, const char *theme_name, const char **error);

  /**
   * @brief Frees the SyntectThemeSet.
   *
   * This function frees the memory allocated for the given SyntectThemeSet.
   *
   * @param theme_set Pointer to the SyntectThemeSet to be freed.
   */
  void syntect_free_theme_set(SyntectThemeSet *theme_set);

  /**
   * @brief Frees the SyntectTheme.
   *
   * This function frees the memory allocated for the given SyntectTheme.
   *
   * @param theme Pointer to the SyntectTheme to be freed.
   */
  void syntect_free_theme(SyntectTheme *theme);

  /**
   * @brief Gets the name of a theme.
   *
   * This function retrieves the name of the given theme.
   *
   * @param theme Pointer to the SyntectTheme.
   * @return The name of the theme as a C string, or NULL if the name is not set. The returned string should be freed using syntect_free_string.
   */
  const char *syntect_get_theme_name(const SyntectTheme *theme);

  /**
   * @brief Frees a C string.
   *
   * This function frees a C string that was allocated by the Rust library.
   *
   * @param s Pointer to the C string to be freed.
   */
  void syntect_free_string(char *s);

#ifdef __cplusplus
}
#endif

#endif // SYNTECT_H
