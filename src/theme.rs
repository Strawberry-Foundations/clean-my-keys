use iced::widget::{button, container, pick_list};
use iced::{Border, Color, Theme, color};

pub const THEME_CORNER_RADIUS: f32 = 8.0;

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
    let default_style = pick_list::default(theme, status);

    match status {
        pick_list::Status::Hovered => pick_list::Style {
            border: border_style(),
            background: iced::Background::Color(Color::from_rgb8(100, 100, 105)),
            ..default_style
        },
        _ => pick_list::Style {
            border: border_style(),
            ..default_style
        },
    }
}

pub fn button_style(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::primary(theme, status);

    style.border = Border {
        color: color!(0x0053_4f58),
        width: 1.0,
        radius: 8.0.into(),
    };

    if matches!(status, button::Status::Hovered) {
        style.border.color = Color::from_rgb(0.0, 0.5, 1.0);
    }

    style
}
