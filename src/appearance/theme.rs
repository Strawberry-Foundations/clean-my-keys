use iced::widget::{button, container, pick_list};
use iced::{Border, Color, Theme, color};

pub const THEME_CORNER_RADIUS: f32 = 8.0;

#[derive(Clone, Copy)]
struct ControlColors {
    surface_bg: Color,
    surface_hover_bg: Color,
    surface_pressed_bg: Color,
    surface_border: Color,
    surface_border_hover: Color,
    text: Color,
    text_muted: Color,
}

fn mix(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

fn control_colors(theme: &Theme) -> ControlColors {
    let palette = theme.palette();
    let is_dark = theme.extended_palette().is_dark;

    if is_dark {
        ControlColors {
            surface_bg: with_alpha(mix(palette.background, palette.text, 0.10), 0.88),
            surface_hover_bg: with_alpha(mix(palette.background, palette.text, 0.16), 0.94),
            surface_pressed_bg: with_alpha(mix(palette.background, palette.text, 0.06), 0.98),
            surface_border: with_alpha(mix(palette.background, palette.text, 0.36), 0.82),
            surface_border_hover: with_alpha(mix(palette.background, palette.text, 0.52), 0.95),
            text: with_alpha(palette.text, 1.0),
            text_muted: with_alpha(mix(palette.text, palette.background, 0.36), 1.0),
        }
    } else {
        ControlColors {
            surface_bg: with_alpha(mix(palette.background, palette.text, 0.05), 0.98),
            surface_hover_bg: with_alpha(mix(palette.background, palette.text, 0.10), 1.0),
            surface_pressed_bg: with_alpha(mix(palette.background, palette.text, 0.14), 1.0),
            surface_border: with_alpha(mix(palette.background, palette.text, 0.28), 0.72),
            surface_border_hover: with_alpha(mix(palette.background, palette.text, 0.40), 0.86),
            text: with_alpha(mix(palette.text, palette.background, 0.05), 1.0),
            text_muted: with_alpha(mix(palette.text, palette.background, 0.45), 1.0),
        }
    }
}

pub fn border_style() -> Border {
    Border {
        color: color!(0x0053_4f58),
        width: 1.0,
        radius: THEME_CORNER_RADIUS.into(),
    }
}

pub fn container_style() -> container::Style {
    container::Style {
        border: border_style(),
        ..container::Style::default()
    }
}

pub fn pick_list_style(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let colors = control_colors(theme);

    match status {
        pick_list::Status::Hovered => pick_list::Style {
            border: Border {
                color: colors.surface_border_hover,
                width: 1.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            background: iced::Background::Color(colors.surface_hover_bg),
            text_color: colors.text,
            placeholder_color: colors.text_muted,
            handle_color: colors.text,
            ..pick_list::default(theme, status)
        },
        _ => pick_list::Style {
            border: Border {
                color: colors.surface_border,
                width: 1.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            background: iced::Background::Color(colors.surface_bg),
            text_color: colors.text,
            placeholder_color: colors.text_muted,
            handle_color: colors.text,
            ..pick_list::default(theme, status)
        },
    }
}

pub fn button_style(theme: &Theme, status: button::Status) -> button::Style {
    let colors = control_colors(theme);
    let mut style = button::primary(theme, status);

    style.border = Border {
        color: colors.surface_border,
        width: 1.0,
        radius: THEME_CORNER_RADIUS.into(),
    };
    style.background = Some(iced::Background::Color(colors.surface_bg));
    style.text_color = colors.text;

    if matches!(status, button::Status::Disabled) {
        style.border.color = with_alpha(colors.surface_border, 0.35);
        style.background = Some(iced::Background::Color(with_alpha(colors.surface_bg, 0.45)));
        style.text_color = with_alpha(colors.text_muted, 0.55);
        return style;
    }

    if matches!(status, button::Status::Hovered) {
        style.border.color = colors.surface_border_hover;
        style.background = Some(iced::Background::Color(colors.surface_hover_bg));
    }

    if matches!(status, button::Status::Pressed) {
        style.border.color = colors.surface_border_hover;
        style.background = Some(iced::Background::Color(colors.surface_pressed_bg));
    }

    style
}

pub fn theme_from_name(name: &str) -> Option<Theme> {
    Theme::ALL
        .iter()
        .find(|theme| theme.to_string() == name)
        .cloned()
}

pub fn window_icon() -> Option<iced::window::Icon> {
    let bytes = include_bytes!("../../assets/image/icon.png");
    let dyn_img = image::load_from_memory(bytes)
        .expect("failed to load icon image from assets");
    let rgba = dyn_img.to_rgba8();
    let (w, h) = rgba.dimensions();
    iced::window::icon::from_rgba(rgba.into_raw(), w, h).ok()
}