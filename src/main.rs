use frontend::DaktronicsSingularUiApp;

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
    .theme(|_| iced::theme::Theme::KanagawaWave)
    .run()
}
