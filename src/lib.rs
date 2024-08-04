extern crate libc;
extern crate syntect;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::io::BufRead;
use std::ptr;
use std::sync::Once;
use syntect::easy::{HighlightFile, HighlightLines};
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

static INIT: Once = Once::new();
static mut THEME_SET: Option<ThemeSet> = None;
static mut SYNTAX_SET: Option<SyntaxSet> = None;

fn initialize() {
    INIT.call_once(|| {
        let ts = ThemeSet::load_defaults();
        let ss = SyntaxSet::load_defaults_newlines();
        unsafe {
            THEME_SET = Some(ts);
            SYNTAX_SET = Some(ss);
        }
    });
}

fn get_syntax_and_theme(theme_name: &str) -> Result<(&'static SyntaxSet, &'static Theme), String> {
    unsafe {
        let ss = SYNTAX_SET
            .as_ref()
            .ok_or_else(|| "SyntaxSet not initialized".to_string())?;
        let ts = THEME_SET
            .as_ref()
            .ok_or_else(|| "ThemeSet not initialized".to_string())?;
        let theme = ts
            .themes
            .get(theme_name)
            .ok_or_else(|| format!("Theme '{}' not found", theme_name))?;
        Ok((ss, theme))
    }
}

#[repr(C)]
pub struct SyntectFile {
    highlighter: HighlightFile<'static>,
}

#[repr(C)]
pub struct SyntectLines {
    highlighter: HighlightLines<'static>,
}

#[no_mangle]
pub extern "C" fn syntect_create_file(
    path: *const c_char,
    theme_name: *const c_char,
    error: *mut *const c_char,
) -> *mut SyntectFile {
    initialize();

    let path = unsafe {
        CStr::from_ptr(path).to_str().unwrap_or_else(|_| {
            *error = CString::new("Invalid path").unwrap().into_raw();
            return "";
        })
    };
    let theme_name = unsafe {
        CStr::from_ptr(theme_name).to_str().unwrap_or_else(|_| {
            *error = CString::new("Invalid theme name").unwrap().into_raw();
            return "";
        })
    };

    let (ss, theme) = match get_syntax_and_theme(theme_name) {
        Ok(result) => result,
        Err(err) => {
            unsafe {
                *error = CString::new(err).unwrap().into_raw();
            }
            return ptr::null_mut();
        }
    };

    let highlighter = match HighlightFile::new(path, ss, theme) {
        Ok(highlighter) => highlighter,
        Err(err) => {
            unsafe {
                *error = CString::new(format!("Failed to create HighlightFile: {}", err))
                    .unwrap()
                    .into_raw();
            }
            return ptr::null_mut();
        }
    };

    Box::into_raw(Box::new(SyntectFile { highlighter }))
}

#[no_mangle]
pub extern "C" fn syntect_highlight_file_line(
    wrapper: *mut SyntectFile,
    error: *mut *const c_char,
) -> *const c_char {
    let wrapper = unsafe {
        assert!(!wrapper.is_null());
        &mut *wrapper
    };

    let mut line = String::new();
    if wrapper.highlighter.reader.read_line(&mut line).unwrap() > 0 {
        let regions: Vec<(Style, &str)> = match wrapper
            .highlighter
            .highlight_lines
            .highlight_line(&line, unsafe { SYNTAX_SET.as_ref().unwrap() })
        {
            Ok(regions) => regions,
            Err(err) => {
                unsafe {
                    *error = CString::new(format!("Highlighting error: {}", err))
                        .unwrap()
                        .into_raw();
                }
                return ptr::null();
            }
        };
        let highlighted_line = as_24_bit_terminal_escaped(&regions[..], true);
        let c_highlighted_line = CString::new(highlighted_line).unwrap();
        return c_highlighted_line.into_raw();
    }

    ptr::null()
}

#[no_mangle]
pub extern "C" fn syntect_free_file(wrapper: *mut SyntectFile) {
    if !wrapper.is_null() {
        unsafe {
            drop(Box::from_raw(wrapper));
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_create_lines(
    theme_name: *const c_char,
    error: *mut *const c_char,
) -> *mut SyntectLines {
    initialize();

    let theme_name = unsafe {
        CStr::from_ptr(theme_name).to_str().unwrap_or_else(|_| {
            *error = CString::new("Invalid theme name").unwrap().into_raw();
            return "";
        })
    };

    let (ss, theme) = match get_syntax_and_theme(theme_name) {
        Ok(result) => result,
        Err(err) => {
            unsafe {
                *error = CString::new(err).unwrap().into_raw();
            }
            return ptr::null_mut();
        }
    };

    let syntax = match ss.find_syntax_by_extension("rs") {
        Some(syntax) => syntax,
        None => {
            unsafe {
                *error = CString::new("Syntax for 'rs' not found")
                    .unwrap()
                    .into_raw();
            }
            return ptr::null_mut();
        }
    };

    let highlighter = HighlightLines::new(syntax, theme);

    Box::into_raw(Box::new(SyntectLines { highlighter }))
}

#[no_mangle]
pub extern "C" fn syntect_highlight_text_line(
    wrapper: *mut SyntectLines,
    line: *const c_char,
    error: *mut *const c_char,
) -> *const c_char {
    let wrapper = unsafe {
        assert!(!wrapper.is_null());
        &mut *wrapper
    };

    let c_str = unsafe { CStr::from_ptr(line) };
    let line = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => {
            unsafe {
                *error = CString::new("Invalid input line").unwrap().into_raw();
            }
            return ptr::null();
        }
    };

    let ranges: Vec<(Style, &str)> = match wrapper
        .highlighter
        .highlight_line(line, unsafe { SYNTAX_SET.as_ref().unwrap() })
    {
        Ok(ranges) => ranges,
        Err(err) => {
            unsafe {
                *error = CString::new(format!("Highlighting error: {}", err))
                    .unwrap()
                    .into_raw();
            }
            return ptr::null();
        }
    };

    let highlighted_line = as_24_bit_terminal_escaped(&ranges[..], true);
    let c_highlighted_line = CString::new(highlighted_line).unwrap();
    c_highlighted_line.into_raw()
}

#[no_mangle]
pub extern "C" fn syntect_free_lines(wrapper: *mut SyntectLines) {
    if !wrapper.is_null() {
        unsafe {
            drop(Box::from_raw(wrapper));
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_create_file() {
        let path = CString::new("test/hello_world.c").unwrap();
        let theme_name = CString::new("base16-ocean.dark").unwrap();
        let mut error: *const c_char = std::ptr::null();

        let wrapper = syntect_create_file(path.as_ptr(), theme_name.as_ptr(), &mut error);

        if !wrapper.is_null() {
            println!("SyntectFile created successfully");
        } else {
            let err_msg = unsafe { CStr::from_ptr(error).to_str().unwrap() };
            println!("Error: {}", err_msg);
        }

        assert!(!wrapper.is_null(), "Failed to create SyntectFile");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        syntect_free_file(wrapper);
    }

    #[test]
    fn test_create_lines() {
        let theme_name = CString::new("base16-ocean.dark").unwrap();
        let mut error: *const c_char = std::ptr::null();

        let wrapper = syntect_create_lines(theme_name.as_ptr(), &mut error);

        assert!(!wrapper.is_null(), "Failed to create SyntectLines");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        syntect_free_lines(wrapper);
    }

    #[test]
    fn test_highlight_text_line() {
        let theme_name = CString::new("base16-ocean.dark").unwrap();
        let mut error: *const c_char = std::ptr::null();

        let wrapper = syntect_create_lines(theme_name.as_ptr(), &mut error);
        assert!(!wrapper.is_null(), "Failed to create SyntectLines");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        let line = CString::new("fn main() { println!(\"Hello, world!\"); }").unwrap();
        let highlighted_line = syntect_highlight_text_line(wrapper, line.as_ptr(), &mut error);
        assert!(!highlighted_line.is_null(), "Failed to highlight line");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        syntect_free_string(highlighted_line as *mut c_char);
        syntect_free_lines(wrapper);
    }
}
