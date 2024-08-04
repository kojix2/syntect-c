extern crate libc;
extern crate syntect;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::io::BufRead;
use std::path::Path;
use std::ptr;
use std::sync::Once;
use syntect::dumps::{dump_to_file, from_dump_file};
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

fn load_theme(tm_file: &str, enable_caching: bool) -> Result<Theme, String> {
    let tm_path = Path::new(tm_file);

    if enable_caching {
        let tm_cache = tm_path.with_extension("tmdump");

        if tm_cache.exists() {
            from_dump_file(tm_cache).map_err(|e| format!("Error loading from cache: {}", e))
        } else {
            let theme =
                ThemeSet::get_theme(tm_path).map_err(|e| format!("Error loading theme: {}", e))?;
            dump_to_file(&theme, tm_cache)
                .map_err(|e| format!("Error dumping theme to cache: {}", e))?;
            Ok(theme)
        }
    } else {
        ThemeSet::get_theme(tm_path).map_err(|e| format!("Error loading theme: {}", e))
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

#[repr(C)]
pub struct SyntectThemeSet {
    themes: ThemeSet,
}

#[repr(C)]
pub struct SyntectTheme {
    theme: Theme,
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
pub extern "C" fn syntect_load_theme_set_from_folder(
    folder: *const c_char,
    error: *mut *const c_char,
) -> *mut SyntectThemeSet {
    let folder = unsafe {
        CStr::from_ptr(folder).to_str().unwrap_or_else(|_| {
            *error = CString::new("Invalid folder path").unwrap().into_raw();
            return "";
        })
    };

    match ThemeSet::load_from_folder(folder) {
        Ok(theme_set) => Box::into_raw(Box::new(SyntectThemeSet { themes: theme_set })),
        Err(err) => {
            unsafe {
                *error = CString::new(format!("Failed to load themes from folder: {}", err))
                    .unwrap()
                    .into_raw();
            }
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_get_theme_names(
    theme_set: *const SyntectThemeSet,
    _error: *mut *const c_char,
) -> *mut *mut c_char {
    let theme_set = unsafe {
        assert!(!theme_set.is_null());
        &*theme_set
    };

    let theme_names: Vec<CString> = theme_set
        .themes
        .themes
        .keys()
        .map(|name| CString::new(name.clone()).unwrap())
        .collect();

    let mut c_theme_names: Vec<*mut c_char> = theme_names
        .iter()
        .map(|name| name.as_ptr() as *mut c_char)
        .collect();

    c_theme_names.push(ptr::null_mut());

    let c_theme_names_ptr = c_theme_names.as_mut_ptr();

    std::mem::forget(c_theme_names);
    std::mem::forget(theme_names);

    c_theme_names_ptr
}

#[no_mangle]
pub extern "C" fn syntect_get_theme_count(theme_names: *const *mut c_char) -> usize {
    unsafe {
        let mut count = 0;
        let mut ptr = theme_names;
        while !(*ptr).is_null() {
            count += 1;
            ptr = ptr.add(1);
        }
        count
    }
}

#[no_mangle]
pub extern "C" fn syntect_free_theme_names(theme_names: *mut *mut c_char) {
    if !theme_names.is_null() {
        unsafe {
            let mut ptr = theme_names;
            while !(*ptr).is_null() {
                let _ = CString::from_raw(*ptr);
                ptr = ptr.add(1);
            }
            drop(Box::from_raw(theme_names));
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_load_theme(
    theme_path: *const c_char,
    enable_caching: bool,
    error: *mut *const c_char,
) -> *mut SyntectTheme {
    let theme_path = unsafe {
        CStr::from_ptr(theme_path).to_str().unwrap_or_else(|_| {
            *error = CString::new("Invalid theme path").unwrap().into_raw();
            return "";
        })
    };

    match load_theme(theme_path, enable_caching) {
        Ok(theme) => Box::into_raw(Box::new(SyntectTheme { theme })),
        Err(err) => {
            unsafe {
                *error = CString::new(err).unwrap().into_raw();
            }
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_get_theme_from_set(
    theme_set: *const SyntectThemeSet,
    theme_name: *const c_char,
    error: *mut *const c_char,
) -> *mut SyntectTheme {
    let theme_set = unsafe {
        assert!(!theme_set.is_null());
        &*theme_set
    };

    let theme_name = unsafe {
        CStr::from_ptr(theme_name).to_str().unwrap_or_else(|_| {
            *error = CString::new("Invalid theme name").unwrap().into_raw();
            return "";
        })
    };

    match theme_set.themes.themes.get(theme_name) {
        Some(theme) => Box::into_raw(Box::new(SyntectTheme {
            theme: theme.clone(),
        })),
        None => {
            unsafe {
                *error = CString::new(format!("Theme '{}' not found", theme_name))
                    .unwrap()
                    .into_raw();
            }
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_free_theme_set(theme_set: *mut SyntectThemeSet) {
    if !theme_set.is_null() {
        unsafe {
            drop(Box::from_raw(theme_set));
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_free_theme(theme: *mut SyntectTheme) {
    if !theme.is_null() {
        unsafe {
            drop(Box::from_raw(theme));
        }
    }
}

#[no_mangle]
pub extern "C" fn syntect_get_theme_name(theme: *const SyntectTheme) -> *const c_char {
    let theme = unsafe {
        assert!(!theme.is_null());
        &*theme
    };

    match &theme.theme.name {
        Some(name) => CString::new(name.clone()).unwrap().into_raw(),
        None => ptr::null(),
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
        let mut error: *const c_char = ptr::null();

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
        let mut error: *const c_char = ptr::null();

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
        let mut error: *const c_char = ptr::null();

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

    #[test]
    fn test_load_theme_set_from_folder() {
        let folder = CString::new("test/themes").unwrap();
        let mut error: *const c_char = ptr::null();

        let theme_set = syntect_load_theme_set_from_folder(folder.as_ptr(), &mut error);

        assert!(!theme_set.is_null(), "Failed to load theme set");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        syntect_free_theme_set(theme_set);
    }

    #[test]
    fn test_get_theme_names() {
        let folder = CString::new("test/themes").unwrap();
        let mut error: *const c_char = ptr::null();

        let theme_set = syntect_load_theme_set_from_folder(folder.as_ptr(), &mut error);
        assert!(!theme_set.is_null(), "Failed to load theme set");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        let theme_names = syntect_get_theme_names(theme_set, &mut error);
        assert!(!theme_names.is_null(), "Failed to get theme names");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        let theme_count = syntect_get_theme_count(theme_names);
        assert!(theme_count > 0, "Theme count should be greater than 0");

        for i in 0..theme_count {
            let theme_name = unsafe { CStr::from_ptr(*theme_names.add(i)).to_str().unwrap() };
            println!("Theme {}: {}", i, theme_name);
        }

        syntect_free_theme_names(theme_names);
        syntect_free_theme_set(theme_set);
    }

    #[test]
    fn test_load_theme() {
        let theme_path = CString::new("test/themes/base16-ocean.tmTheme").unwrap();
        let mut error: *const c_char = ptr::null();

        let theme = syntect_load_theme(theme_path.as_ptr(), true, &mut error);
        assert!(!theme.is_null(), "Failed to load theme");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        syntect_free_theme(theme);
    }

    #[test]
    fn test_get_theme_from_set() {
        let folder = CString::new("test/themes").unwrap();
        let theme_name = CString::new("base16-ocean").unwrap();
        let mut error: *const c_char = ptr::null();

        let theme_set = syntect_load_theme_set_from_folder(folder.as_ptr(), &mut error);
        assert!(!theme_set.is_null(), "Failed to load theme set");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        let theme = syntect_get_theme_from_set(theme_set, theme_name.as_ptr(), &mut error);
        assert!(!theme.is_null(), "Failed to get theme from set");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        syntect_free_theme(theme);
        syntect_free_theme_set(theme_set);
    }

    #[test]
    fn test_get_theme_name() {
        let theme_path = CString::new("test/themes/base16-ocean.dark").unwrap();
        let mut error: *const c_char = ptr::null();

        let theme = syntect_load_theme(theme_path.as_ptr(), true, &mut error);
        assert!(!theme.is_null(), "Failed to load theme");
        assert!(error.is_null(), "Unexpected error: {:?}", unsafe {
            CStr::from_ptr(error).to_str().unwrap()
        });

        let theme_name = syntect_get_theme_name(theme);
        assert!(!theme_name.is_null(), "Failed to get theme name");
        let theme_name_str = unsafe { CStr::from_ptr(theme_name).to_str().unwrap() };
        assert_eq!(theme_name_str, "Base16 Ocean", "Theme name does not match");

        syntect_free_string(theme_name as *mut c_char);
        syntect_free_theme(theme);
    }
}