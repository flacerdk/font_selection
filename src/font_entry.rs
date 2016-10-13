use font_style::FontStyle;

#[derive(Clone)]
pub struct FontEntry {
    pub name: String,
    pub family_name: String,
    pub style: FontStyle
}

impl FontEntry {
    pub fn has_glyph(&self, codepoint: char) -> bool {
        unimplemented!();
    }
}
