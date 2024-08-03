#ifndef SYNTECT_H
#define SYNTECT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Wrapper struct for HighlightFile in Rust.
 */
typedef struct HighlightFileWrapper HighlightFileWrapper;

/**
 * @brief Wrapper struct for HighlightLines in Rust.
 */
typedef struct HighlightLinesWrapper HighlightLinesWrapper;

/**
 * @brief Creates a HighlightFileWrapper.
 *
 * This function initializes a HighlightFileWrapper, which can be used to highlight the content of a file.
 *
 * @param path The path to the file to be highlighted.
 * @param theme_name The name of the theme to be used for highlighting.
 * @param error Pointer to store the error message if the function fails. This should be freed using free_string.
 * @return Pointer to the created HighlightFileWrapper, or NULL if an error occurs.
 */
HighlightFileWrapper* create_highlight_file(const char* path, const char* theme_name, const char** error);

/**
 * @brief Highlights a line from the file.
 *
 * This function reads and highlights a single line from the file associated with the given HighlightFileWrapper.
 *
 * @param wrapper Pointer to the HighlightFileWrapper.
 * @param error Pointer to store the error message if the function fails. This should be freed using free_string.
 * @return The highlighted line as a C string, or NULL if an error occurs. The returned string should be freed using free_string.
 */
const char* highlight_file_line(HighlightFileWrapper* wrapper, const char** error);

/**
 * @brief Frees the HighlightFileWrapper.
 *
 * This function frees the memory allocated for the given HighlightFileWrapper.
 *
 * @param wrapper Pointer to the HighlightFileWrapper to be freed.
 */
void free_highlight_file(HighlightFileWrapper* wrapper);

/**
 * @brief Creates a HighlightLinesWrapper.
 *
 * This function initializes a HighlightLinesWrapper, which can be used to highlight lines of text.
 *
 * @param theme_name The name of the theme to be used for highlighting.
 * @param error Pointer to store the error message if the function fails. This should be freed using free_string.
 * @return Pointer to the created HighlightLinesWrapper, or NULL if an error occurs.
 */
HighlightLinesWrapper* create_highlight_lines(const char* theme_name, const char** error);

/**
 * @brief Highlights a line of text.
 *
 * This function highlights a single line of text using the given HighlightLinesWrapper.
 *
 * @param wrapper Pointer to the HighlightLinesWrapper.
 * @param line The line of text to be highlighted.
 * @param error Pointer to store the error message if the function fails. This should be freed using free_string.
 * @return The highlighted line as a C string, or NULL if an error occurs. The returned string should be freed using free_string.
 */
const char* highlight_text_line(HighlightLinesWrapper* wrapper, const char* line, const char** error);

/**
 * @brief Frees the HighlightLinesWrapper.
 *
 * This function frees the memory allocated for the given HighlightLinesWrapper.
 *
 * @param wrapper Pointer to the HighlightLinesWrapper to be freed.
 */
void free_highlight_lines(HighlightLinesWrapper* wrapper);

/**
 * @brief Frees a C string.
 *
 * This function frees a C string that was allocated by the Rust library.
 *
 * @param s Pointer to the C string to be freed.
 */
void free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif // SYNTECT_H

