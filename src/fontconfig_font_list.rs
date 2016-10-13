use fontconfig::fontconfig::{FcChar8, FcConfigGetCurrent, FcConfigGetFonts, FcFontSet};
use fontconfig::fontconfig::{FcPatternGetString, FcResultMatch, FcSetSystem};
use libc::{c_char, c_int};
use platform::{Platform, FontList};
use std::ffi::CStr;
use std::str;
use std::ptr;
use unicode_script::Script;

static FC_FAMILY: &'static [u8] = b"family\0";
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
            let font_list: *mut FcFontSet = FcConfigGetFonts(config, FcSetSystem);
            self.font_list = find_and_add_families(font_list);
        }
    }
}

unsafe fn find_and_add_families(font_set: *mut FcFontSet) -> FontList {
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

        if FcPatternGetString(*font, FC_FAMILY.as_ptr() as *mut c_char, v, &mut family) == FcResultMatch {
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
