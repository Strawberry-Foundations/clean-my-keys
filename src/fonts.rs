use iced::widget::Text;
use iced::{Font, font};

pub const ICON_KEYBOARD: &str = "\u{e312}";
pub const ICON_KEYBOARD_LOCK: &str = "\u{f492}";
pub const ICON_MOP: &str = "\u{e28d}";
pub const ICON_USB: &str = "\u{e1e0}";

pub fn load_fonts() -> Vec<std::borrow::Cow<'static, [u8]>> {
    vec![
        include_bytes!("../assets/fonts/gsans_code.ttf")
            .as_slice()
            .into(),
        include_bytes!("../assets/fonts/gsans_code_bold.ttf")
            .as_slice()
            .into(),
        include_bytes!("../assets/fonts/material_symbols_rounded.ttf")
            .as_slice()
            .into(),
    ]
}

pub const GSANSCODE_BOLD: Font = Font {
    family: font::Family::Name("Google Sans Code"),
    weight: font::Weight::Bold,
    stretch: font::Stretch::Normal,
    style: font::Style::Normal,
};

pub fn icon(codepoint: &str) -> Text<'static> {
    Text::new(codepoint.to_string()).font(Font {
        family: font::Family::Name("Material Symbols Rounded"),
        ..Font::DEFAULT
    })
}
