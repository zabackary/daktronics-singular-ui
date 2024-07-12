use frontend::DaktronicsSingularUiApp;

mod backend;
mod frontend;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn main() -> iced::Result {
    iced::application(
        "Daktronics Singular UI",
        DaktronicsSingularUiApp::update,
        DaktronicsSingularUiApp::view,
    )
    .theme(|_| iced::theme::Theme::KanagawaWave)
    .exit_on_close_request(true)
    .run()
}
