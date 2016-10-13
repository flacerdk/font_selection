use unicode_script::Script;
use font_style::FontStyle;

pub trait Platform {
    fn get_font_list(&self, generic_family: String) -> Vec<String>;
    fn update_font_list(&mut self);
    fn get_common_fallback_fonts(codepoint: char, next_codepoint: char,
                                  script: Script) -> Vec<String>;
    fn create_platform_font_list(&mut self);
    // TODO: implement FontFamily, FontGroup
    // fn create_font_group(family_list: Vec<FontFamily>, style: FontStyle) -> FontGroup;
    // TODO: implement FontEntry
    // fn lookup_local_font(font_name: String, style: FontStyle) -> FontEntry;
}

#[derive(Debug)]
pub struct FontList {
    pub font_families: Vec<String>,
}

impl FontList {
    pub fn new() -> Self {
        FontList { font_families: Vec::new() }
    }
}
