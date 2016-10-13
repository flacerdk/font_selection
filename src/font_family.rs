use font_style::FontStyle;
use font_entry::FontEntry;

pub struct FontFamily {
    name: String,
    available_fonts: Vec<FontEntry>
}

// This is for functions that are not platform specific.
impl FontFamily {
    fn add_font_entry(&mut self, font: FontEntry) {
        self.available_fonts.push(font)
    }

    fn find_font_for_style(&self, font_style: FontStyle) -> Option<&FontEntry> {
        if self.available_fonts.len() > 0 {
            Some(&self.available_fonts[0])
        } else {
            None
        }
    }

    fn find_all_fonts_for_style(&self, font_style: FontStyle) -> Vec<FontEntry> {
        // TODO: Should populate FontFamily first; implement FindStyleVariations()
        // TODO: should compute metrics for font style distance, and return the ones
        // that more or less match.
        let mut fonts = vec!();
        for font in &self.available_fonts {
            fonts.push(font.clone());
        }
        fonts
    }

    fn find_font_for_char(&self, codepoint: char) -> Option<FontEntry> {
        for font in &self.available_fonts {
            if font.has_glyph(codepoint) {
                return Some(font.clone())
            }
        }
        None
    }
}

// Here, each platform will have its own function.
pub trait PlatformFontFamily {
    fn find_style_variations(font_family: &mut FontFamily);
}
