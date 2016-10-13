use fontconfig::fontconfig::{FcChar8, FcConfigGetCurrent, FcConfigGetFonts, FcFontSet};
use fontconfig::fontconfig::{FcPattern, FcPatternGetString, FcResultMatch, FcSetSystem};
use libc::{c_char, c_int};
use platform::{Platform, FontList};
use std::ffi::CStr;
use std::str;
use std::ptr;
use unicode_script::Script;

static FC_FAMILY: &'static [u8] = b"family\0";
static FC_FAMILYLANG: &'static [u8] = b"familylang\0";
static FC_FONTFORMAT: &'static [u8] = b"fontformat\0";

#[derive(Debug)]
pub struct Fontconfig {
    pub font_list: FontList,
}

impl Fontconfig {
    pub fn new() -> Self {
        Fontconfig { font_list: FontList::new() }
    }
}

impl Platform for Fontconfig {
    fn get_font_list(&self, generic_family: String) -> Vec<String> {
        let font_list: Vec<String> = vec!();
        font_list
    }

    fn update_font_list(&mut self) {
        unimplemented!();
    }

    fn get_common_fallback_fonts(codepoint: char, next_codepoint: char,
                                 script: Script) -> Vec<String> {
        unimplemented!();
    }

    fn create_platform_font_list(&mut self) {
        unsafe {
            let config = FcConfigGetCurrent();
            let font_set: *const FcFontSet = FcConfigGetFonts(config, FcSetSystem);
            self.font_list = add_font_set_families(font_set);
        }
    }
}

unsafe fn add_font_set_families(font_set: *const FcFontSet) -> FontList {
    let mut font_list = FontList::new();
    for i in 0..((*font_set).nfont as isize) {
        let font = (*font_set).fonts.offset(i);
        let mut family: *mut FcChar8 = ptr::null_mut();
        let mut format: *mut FcChar8 = ptr::null_mut();
        let mut v: c_int = 0;
        if FcPatternGetString(*font, FC_FONTFORMAT.as_ptr() as *mut c_char, v, &mut format) != FcResultMatch {
            continue;
        }

        let font_format = c_str_to_string(format as *const c_char);
        if font_format != "TrueType" && font_format != "CFF" && font_format != "Type 1" {
            continue;
        }

        let canonical_index = find_canonical_name_index(*font, FC_FAMILYLANG);
        if canonical_index < 0 {
            continue;
        }
        if FcPatternGetString(*font, FC_FAMILY.as_ptr() as *mut c_char,
                              canonical_index, &mut family) == FcResultMatch {
            let family_name = c_str_to_string(family as *const c_char);
            font_list.font_families.push(family_name);
        }
    }
    font_list
}

/// Creates a String from the given null-terminated buffer.
/// Panics if the buffer does not contain UTF-8.
unsafe fn c_str_to_string(s: *const c_char) -> String {
    str::from_utf8(CStr::from_ptr(s).to_bytes()).unwrap().to_owned()
}

unsafe fn find_canonical_name_index(font: *mut FcPattern, lang_slice: &[u8]) -> c_int {
    let language = str::from_utf8(lang_slice).unwrap();
    let mut n: c_int = 0;
    let mut en: c_int = 0;
    let mut fc_lang: *mut FcChar8 = ptr::null_mut();
    while FcPatternGetString(font, language.as_ptr() as *mut c_char, n, &mut fc_lang) == FcResultMatch {
        let lang = c_str_to_string(fc_lang as *const c_char);
        let is_en = lang.starts_with("en");
        if is_en && (lang.len() == 2 || (lang.len() > 2 && language.chars().nth(2) == Some('-'))) {
            println!("{}", n);
            en = n;
            break;
        }
        n += 1;
    }
    en
}
