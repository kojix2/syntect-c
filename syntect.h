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
