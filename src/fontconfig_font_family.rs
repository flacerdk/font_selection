use font_family::{PlatformFontFamily, FontFamily};

struct FontconfigFontFamily;

impl PlatformFontFamily for FontconfigFontFamily {
    fn find_style_variations(font_family: &mut FontFamily) {
        unimplemented!();
    }
}
