extern crate fontconfig;
extern crate libc;
extern crate unicode_script;

mod font_style;
mod font_family;
mod font_entry;
pub mod platform;
pub mod fontconfig_font_list;
pub mod fontconfig_font_family;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
