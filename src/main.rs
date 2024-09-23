#![windows_subsystem = "windows"]

use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use clap::Parser;
use frontend::{DaktronicsSingularUiApp, Screen};
use iced::{theme::Palette, window::icon, Color, Font, Size};

mod backend;
mod frontend;
mod mock;

/// Links the output of an Daktronics AllSport 5000 to Singular.Live
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to hide the UI (run without a window). Must be used with
    /// --start.
    #[arg(short = 'l', long, default_value_t = false)]
    headless: bool,

    /// The configuration file. If not provided, the UI will prompt for one.
    #[arg(short, long)]
    config_file: Option<PathBuf>,

    /// Whether to start the stream immediately. Must be used with --config-file
    /// and --serial-path.
    #[arg(short, long, default_value_t = false)]
    start: bool,

    /// What serial path (e.g. /dev/xxx or COM1 on Windows) to use. If not
    /// provided, the UI will prompt for one.
    #[arg(short = 'p', long)]
    serial_path: Option<String>,
}

enum DSUError {
    Iced(iced::Error),
    HeadlessWithoutStart,
    StartWithoutConfigSerial,
}

impl Display for DSUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Iced(iced_error) => write!(f, "error when running ui: {}", iced_error),
            Self::HeadlessWithoutStart => write!(f, "cannot run headless without using --start"),
            Self::StartWithoutConfigSerial => write!(
                f,
                "cannot start automatically without a config file and serial path"
            ),
        }
    }
}

// Hacky implementation that prints errors nicely to the CLI
impl Debug for DSUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl std::error::Error for DSUError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Iced(iced_error) => Some(iced_error),
            _ => None,
        }
    }
}

const APP_NAME: &str = "Daktronics Singular UI";
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const DAKTRONICS_SINGULAR_UI_PROFILE_FILE_EXTENSION: &str = "dsu";
const GITHUB_URL: &str = "https://github.com/zabackary/daktronics-singular-ui";

fn main() -> Result<(), DSUError> {
    let args = Args::parse();

    if args.headless && !args.start {
        return Err(DSUError::HeadlessWithoutStart);
    }
    if args.start && (args.serial_path.is_none() || args.config_file.is_none()) {
        return Err(DSUError::StartWithoutConfigSerial);
    }

    let program_icon = image::load_from_memory_with_format(
        include_bytes!("../assets/logo.png"),
        image::ImageFormat::Png,
    )
    .expect("couldn't parse static program icon")
    .to_rgba8();
    let program_icon_width = program_icon.width();
    let program_icon_height = program_icon.height();

    iced::application(
        |app: &DaktronicsSingularUiApp| {
            if matches!(app.screen, Screen::Welcome) {
                APP_NAME.to_owned()
            } else {
                format!(
                    "{} - {}{}",
                    APP_NAME,
                    app.profile.name,
                    if app.profile_dirty { "*" } else { "" }
                )
            }
        },
        DaktronicsSingularUiApp::update,
        DaktronicsSingularUiApp::view,
    )
    .subscription(DaktronicsSingularUiApp::subscription)
    .window(iced::window::Settings {
        min_size: Some(Size::new(700.0, 400.0)),
        icon: Some(
            icon::from_rgba(
                program_icon.into_vec(),
                program_icon_width,
                program_icon_height,
            )
            .expect("failed to construct static program icon"),
        ),
        ..Default::default()
    })
    .font(include_bytes!("../assets/FiraSans-Regular.ttf"))
    .default_font(Font::with_name("Fira Sans"))
    .exit_on_close_request(false)
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
    .map_err(DSUError::Iced)
}
