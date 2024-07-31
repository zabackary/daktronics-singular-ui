use frontend::DaktronicsSingularUiApp;
use iced::{theme::Palette, Color, Font, Size};

mod backend;
mod frontend;
mod mock;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const DAKTRONICS_SINGULAR_UI_PROFILE_FILE_EXTENSION: &str = "dsu";

fn main() -> iced::Result {
    iced::application(
        "Daktronics Singular UI",
        DaktronicsSingularUiApp::update,
        DaktronicsSingularUiApp::view,
    )
    .window(iced::window::Settings {
        min_size: Some(Size::new(700.0, 400.0)),
        icon: None,
        ..Default::default()
    })
    .font(include_bytes!("../assets/FiraSans-Regular.ttf"))
    .default_font(Font::with_name("Fira Sans"))
    .theme(|app| {
        iced::theme::Theme::custom(
            "Daktronics Singular UI".to_owned(),
            if app.dark_mode {
                Palette {
                    primary: Color::from_rgb8(0xD8, 0xC7, 0x70),
                    success: Color::TRANSPARENT,
                    background: Color::from_rgb8(0x22, 0x20, 0x17),
                    danger: Color::from_rgb8(0xFF, 0xB8, 0xAB),
                    text: Color::from_rgb8(0xE8, 0xE2, 0xD4),
                }
            } else {
                Palette {
                    primary: Color::from_rgb8(0x6B, 0x5F, 0x10),
                    success: Color::TRANSPARENT,
                    background: Color::from_rgb8(0xF3, 0xED, 0xE0),
                    danger: Color::from_rgb8(0xBA, 0x1A, 0x1A),
                    text: Color::from_rgb8(0x1D, 0x1C, 0x13),
                }
            },
        )
    })
    .run()
}
