mod configure;
mod graph;
mod header;
mod stream_running;
mod stream_start;
mod utils;
mod welcome;

use std::path::PathBuf;

use configure::{configure, ConfigureEvent};
use header::{header, HeaderScreen};
use iced::border::Radius;
use iced::widget::{column, container, horizontal_space, row, scrollable, svg, text, text_input};
use iced::{Alignment, Element, Length, Subscription, Task};
use tokio::fs;
use tokio::io::AsyncReadExt;
use utils::{icon_button, rounded_button, rounded_pane, rounded_text_input_style};

use crate::backend::profile::Profile;
use crate::backend::stream::{ActiveStream, WorkerEvent};
use crate::{DAKTRONICS_SINGULAR_UI_PROFILE_FILE_EXTENSION, GITHUB_URL};

#[derive(Debug)]
pub struct DaktronicsSingularUiApp {
    pub screen: Screen,
    pub profile: Profile,
    pub profile_dirty: bool,
    pub dark_mode: bool,
    pub sport_type_keys: Vec<String>,
    pub hide_header: bool,
    pub unattended: Option<usize>,
    pub initial_tty_path: Option<String>,
}

fn use_dark_mode() -> bool {
    matches!(dark_light::detect(), dark_light::Mode::Dark)
}

impl Default for DaktronicsSingularUiApp {
    fn default() -> Self {
        DaktronicsSingularUiApp {
            screen: Default::default(),
            profile_dirty: false,
            profile: Default::default(),
            dark_mode: use_dark_mode(),
            sport_type_keys: vec![],
            hide_header: false,
            unattended: None,
            initial_tty_path: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    TryNewProfile,
    NewProfile,
    TryImportProfile,
    ImportProfileFromPicker,
    ImportProfileFromPath(PathBuf),
    ImportProfileFinished(Profile),
    ExportProfile,
    ExportProfileFinished,
    WelcomeNewProfile,
    WelcomeImportProfile,
    WelcomeGitHub,
    EndStream,
    SwitchScreen(HeaderScreen),
    ProfileNameChange(String),
    UpdateStreamStats,
    UpdateStreamStatsResponse(Vec<WorkerEvent>),
    HandleConfigureEvent(ConfigureEvent),
    CloseRequested,
    Close,
    SetUpTokenUpdated(String),
    SetUpCopyScript,
    SetUpOpenDataStreams,
    SetUpOpenDashboard,

    StreamRunningMessage(stream_running::StreamRunningMessage),
    StreamStartMessage(stream_start::StreamStartMessage),
}

#[derive(Debug, Default)]
pub enum Screen {
    Configure,
    SetUp(String),
    StreamRunning(stream_running::StreamRunning, ActiveStream),
    StreamStart(stream_start::StreamStart, Option<String>),
    #[default]
    Welcome,
}

impl Screen {
    pub fn stream_running(stream: ActiveStream) -> Self {
        Self::StreamRunning(stream_running::StreamRunning::new(), stream)
    }
}

impl DaktronicsSingularUiApp {
    pub fn update(&mut self, message: Message) -> impl Into<Task<Message>> {
        self.dark_mode = use_dark_mode();
        match message {
            Message::NoOp => Task::none(),
            Message::ExportProfile => {
                let profile_name = self.profile.name.clone();
                let result = serde_json::to_string(&self.profile);
                Task::future(async move {
                    async fn export_profile(
                        profile_name: &str,
                        serialized: serde_json::Result<String>,
                    ) -> Result<Option<PathBuf>, String> {
                        let serialized = serialized.map_err(|x| x.to_string())?;
                        if let Some(location) = rfd::AsyncFileDialog::new()
                            .set_title("Save profile as")
                            .add_filter(
                                "Daktronics Singular UI Profile",
                                &[DAKTRONICS_SINGULAR_UI_PROFILE_FILE_EXTENSION],
                            )
                            .set_file_name(&format!("{}.dsu", filenamify::filenamify(profile_name)))
                            .save_file()
                            .await
                        {
                            fs::write(location.path(), serialized)
                                .await
                                .map(|_| Some(location.path().to_path_buf()))
                                .map_err(|err| err.to_string())
                        } else {
                            Ok(None)
                        }
                    }
                    match export_profile(&profile_name, result).await {
                        Ok(Some(location)) => {
                            rfd::AsyncMessageDialog::new()
                                .set_title("Finished export")
                                .set_description(&format!(
                                    "Finished exporting the profile \"{}\" to {}",
                                    profile_name,
                                    location.display()
                                ))
                                .set_level(rfd::MessageLevel::Info)
                                .show()
                                .await;
                            Message::ExportProfileFinished
                        }
                        Ok(None) => Message::NoOp,
                        Err(err) => {
                            rfd::AsyncMessageDialog::new()
                                .set_level(rfd::MessageLevel::Error)
                                .set_title("Failed to export profile")
                                .set_description(err.to_string())
                                .show()
                                .await;
                            Message::NoOp
                        }
                    }
                })
            }
            Message::ExportProfileFinished => {
                self.profile_dirty = false;
                Task::none()
            }
            Message::TryImportProfile => {
                if self.profile_dirty {
                    Task::future(async {
                        match rfd::AsyncMessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning)
                            .set_title("Confirm overwrite profile")
                            .set_description("Your current profile has unsaved changes. Importing a profile will discard any changes that haven't been exported from the old profile.")
                            .set_buttons(rfd::MessageButtons::OkCancel)
                            .show()
                            .await {
                                rfd::MessageDialogResult::Ok => Message::ImportProfileFromPicker,
                                rfd::MessageDialogResult::Cancel => Message::NoOp,
                                _ => unreachable!("ok/cancel dialog will returned non-Ok/Cancel result")
                            }
                    })
                } else {
                    Task::done(Message::ImportProfileFromPicker)
                }
            }
            Message::ImportProfileFromPicker => Task::future(async move {
                if let Some(path) = rfd::AsyncFileDialog::new()
                    .set_title("Open profile")
                    .add_filter(
                        "Daktronics Singular UI Profile",
                        &[DAKTRONICS_SINGULAR_UI_PROFILE_FILE_EXTENSION],
                    )
                    .pick_file()
                    .await
                {
                    Message::ImportProfileFromPath(path.path().to_path_buf())
                } else {
                    Message::NoOp
                }
            }),
            Message::ImportProfileFromPath(path) => Task::future(async move {
                async fn import_from_path(path: PathBuf) -> Result<Profile, String> {
                    let mut file = fs::File::open(path).await.map_err(|err| err.to_string())?;
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)
                        .await
                        .map_err(|err| err.to_string())?;
                    serde_json::from_str(&buffer).map_err(|err| err.to_string())
                }
                match import_from_path(path).await {
                    Ok(profile) => Message::ImportProfileFinished(profile),
                    Err(err) => {
                        rfd::AsyncMessageDialog::new()
                            .set_level(rfd::MessageLevel::Error)
                            .set_title("Failed to import profile")
                            .set_description(err.to_string())
                            .show()
                            .await;
                        Message::NoOp
                    }
                }
            }),
            Message::ImportProfileFinished(profile) => {
                if let Some(sport_type) = profile.sport_type {
                    self.sport_type_keys = sport_type
                        .all_serialized_keys()
                        .expect("failed to get key list for sport");
                } else {
                    self.sport_type_keys.clear();
                }
                self.profile = profile;
                self.profile_dirty = false;
                self.screen = Screen::Configure;
                Task::none()
            }
            Message::TryNewProfile => {
                if self.profile_dirty {
                    Task::future(async {
                        match rfd::AsyncMessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning)
                            .set_title("Confirm overwrite profile")
                            .set_description("Your current profile has unsaved changes. Creating a new one will discard any changes not already exported.")
                            .set_buttons(rfd::MessageButtons::OkCancel)
                            .show()
                            .await {
                                rfd::MessageDialogResult::Ok => Message::NewProfile,
                                rfd::MessageDialogResult::Cancel => Message::NoOp,
                                _ => unreachable!("ok/cancel dialog will returned non-Ok/Cancel result")
                            }
                    })
                } else {
                    Task::done(Message::NewProfile)
                }
            }
            Message::NewProfile => {
                self.profile = Profile::default();
                self.sport_type_keys.clear();
                self.profile_dirty = false;
                Task::none()
            }
            Message::WelcomeImportProfile => Task::done(Message::ImportProfileFromPicker),
            Message::WelcomeNewProfile => {
                self.screen = Screen::Configure;
                Task::none()
            }
            Message::WelcomeGitHub => {
                open::that_detached(GITHUB_URL).expect("failed to launch github in browser");
                Task::none()
            }
            Message::EndStream => {
                // Drop the stream, killing the background threads implicitly
                self.screen = Screen::StreamStart(stream_start::StreamStart::new(), None);
                Task::none()
            }
            Message::ProfileNameChange(new_name) => {
                self.profile_dirty = true;
                self.profile.name = new_name;
                Task::none()
            }
            Message::SwitchScreen(new_screen) => {
                self.screen = match new_screen {
                    HeaderScreen::Configure => Screen::Configure,
                    HeaderScreen::SetUp => Screen::SetUp(String::new()),
                    HeaderScreen::Stream => {
                        Screen::StreamStart(stream_start::StreamStart::new(), None)
                    }
                };
                Task::none()
            }
            Message::UpdateStreamStats => match self.screen {
                Screen::StreamRunning(ref _stream_running, ref mut active_stream) => {
                    let rx = active_stream.worker_event_rx.clone();
                    Task::perform(
                        async move {
                            let mut buffer = Vec::new();
                            rx.lock().await.recv_many(&mut buffer, 16).await;
                            buffer
                        },
                        Message::UpdateStreamStatsResponse,
                    )
                }
                _ => Task::none(),
            },
            Message::UpdateStreamStatsResponse(events) => match self.screen {
                Screen::StreamRunning(ref _stream_running, ref mut active_stream) => {
                    active_stream.update_from_events(events);
                    if self.unattended.is_some()
                        && self.initial_tty_path.is_some()
                        && active_stream.errors().len() > self.unattended.unwrap()
                    {
                        eprintln!(
                            "ERR frontend Stream will be restarted due to volume of errors ({}) exceeding configured value ({}) (unattended mode)",
                            active_stream.errors().len(),
                            self.unattended.unwrap()
                        );
                        match ActiveStream::new(
                            self.profile.clone(),
                            self.initial_tty_path.as_ref().expect("no tty path").clone(),
                        ) {
                            Ok(stream) => {
                                self.screen = Screen::StreamRunning(
                                    stream_running::StreamRunning::new(),
                                    stream,
                                );
                                eprintln!("INFO frontend Restarted stream successfully");
                            }
                            Err(err) => {
                                eprintln!("ERR frontend Failed to restart stream: {}", err);
                            }
                        }
                    }
                    Task::done(Message::UpdateStreamStats)
                }
                _ => Task::none(),
            },
            Message::HandleConfigureEvent(event) => {
                self.profile_dirty = true;
                match event {
                    ConfigureEvent::DataStreamUrlUpdated(new) => self.profile.data_stream_url = new,
                    ConfigureEvent::SubcompNameUpdated(new) => self.profile.subcomp_name = new,
                    ConfigureEvent::SportTypeUpdated(new) => {
                        self.sport_type_keys = new
                            .all_serialized_keys()
                            .expect("failed to get key list for sport");
                        self.profile.sport_type = Some(new)
                    }
                    ConfigureEvent::MultipleRequestsUpdated(new) => {
                        self.profile.multiple_requests = new
                    }
                    ConfigureEvent::ExcludeIncompleteDataUpdated(new) => {
                        self.profile.exclude_incomplete_data = new
                    }
                    ConfigureEvent::MappingItemAdded => {
                        self.profile.mapping.items.push(Default::default())
                    }
                    ConfigureEvent::MappingItemRemoved(i) => {
                        self.profile.mapping.items.remove(i);
                    }
                    ConfigureEvent::MappingItemEnabledUpdated(i, new) => {
                        self.profile.mapping.items[i].enabled = new
                    }
                    ConfigureEvent::MappingItemSourceFieldUpdated(i, new) => {
                        self.profile.mapping.items[i].source_field = new
                    }
                    ConfigureEvent::MappingItemTransformationUpdated(i, new) => {
                        self.profile.mapping.items[i].transformation = new
                    }
                    ConfigureEvent::MappingItemDestinationFieldUpdated(i, new) => {
                        self.profile.mapping.items[i].destination_field = new
                    }
                };
                Task::none()
            }
            Message::CloseRequested => {
                let profile_dirty = self.profile_dirty;
                let is_streaming = matches!(self.screen, Screen::StreamRunning(_, _));
                if profile_dirty || is_streaming {
                    Task::future(async move {
                        match rfd::AsyncMessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning)
                            .set_title("Confirm quit")
                            .set_description(&format!("{} Are you sure you want to close the application?", match (profile_dirty, is_streaming) {
                                (true, true) => "You are currently streaming to Singular, so quitting the application will terminate the data stream! In addition, you have not saved your profile and all unsaved changes will be discarded if you quit.",
                                (false, true) => "You are currently streaming to Singular, so quitting the application will terminate the data stream!",
                                (true, false) => "You have not saved your profile, so all unsaved changes will be discarded if you quit.",
                                (false, false) => unreachable!()
                            }))
                            .set_buttons(rfd::MessageButtons::OkCancel)
                            .show()
                            .await {
                                rfd::MessageDialogResult::Ok => {
                                    Message::Close
                                }
                                rfd::MessageDialogResult::Cancel => {
                                    Message::NoOp
                                }
                                _ => unreachable!("ok/cancel message dialog returned non-Ok/Cancel")
                            }
                    })
                } else {
                    Task::done(Message::Close)
                }
            }
            Message::Close => iced::exit(),
            Message::SetUpCopyScript => match self.screen {
                Screen::SetUp(ref token) => iced::clipboard::write(
                    include_str!("../assets/root_composition_script.js")
                        .replace("{{ token }}", token),
                ),
                _ => Task::none(),
            },
            Message::SetUpOpenDashboard => {
                open::that_detached("https://app.singular.live/dashboard")
                    .expect("failed to launch dashboard in browser");
                Task::none()
            }
            Message::SetUpOpenDataStreams => {
                open::that_detached("https://app.singular.live/datastreammanager")
                    .expect("failed to launch data stream manager in browser");
                Task::none()
            }
            Message::SetUpTokenUpdated(new) => match self.screen {
                Screen::SetUp(ref mut token) => {
                    *token = new;
                    Task::none()
                }
                _ => Task::none(),
            },

            Message::StreamRunningMessage(message) => match self.screen {
                Screen::StreamRunning(ref mut stream_running, ref mut stream) => {
                    match stream_running.update(message) {
                        stream_running::Update::None => Task::none(),
                        stream_running::Update::ClearErrors => {
                            stream.clear_errors();
                            Task::none()
                        }
                    }
                }
                _ => Task::none(),
            },
            Message::StreamStartMessage(message) => match self.screen {
                Screen::StreamStart(ref mut screen_start, ref mut error) => {
                    match screen_start.update(message) {
                        stream_start::Update::None => Task::none(),
                        stream_start::Update::StartStream { port } => {
                            if self.profile.sport_type.is_some() {
                                match ActiveStream::new(self.profile.to_owned(), port) {
                                    Ok(stream) => {
                                        self.screen = Screen::StreamRunning(
                                            stream_running::StreamRunning::new(),
                                            stream,
                                        );
                                    }
                                    Err(err) => *error = Some(err.to_string()),
                                }
                                Task::done(Message::UpdateStreamStats)
                            } else {
                                *error = Some(
                                    "You must set a sport type before beginning the stream."
                                        .to_owned(),
                                );
                                Task::none()
                            }
                        }
                    }
                }
                _ => Task::none(),
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, status, _id| {
            if matches!(status, iced::event::Status::Ignored) {
                match event {
                    iced::Event::Window(window_event) => match window_event {
                        iced::window::Event::CloseRequested => Some(Message::CloseRequested),
                        iced::window::Event::FileDropped(path) => {
                            Some(Message::ImportProfileFromPath(path))
                        }
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    pub fn view(&self) -> Element<Message> {
        if matches!(self.screen, Screen::Welcome) {
            welcome::view(
                Message::WelcomeImportProfile,
                Message::WelcomeNewProfile,
                Message::WelcomeGitHub,
            )
        } else {
            column([
                if self.hide_header {
                    container(
                        row([
                            container(
                                svg(svg::Handle::from_memory(include_bytes!(
                                    "../assets/logo.svg"
                                )))
                                .content_fit(iced::ContentFit::Fill),
                            )
                            .width(18)
                            .height(18)
                            .into(),
                            horizontal_space().width(4.0).into(),
                            text("Daktronics Singular UI - Profile: \"").size(12.0).into(),
                            text(&self.profile.name).size(12.0).into(),
                            text("\" live").size(12.0).into(),
                        ])
                        .align_y(Alignment::Center)
                        .padding(4.0)
                    )
                    .style(|theme: &iced::Theme| {
                        let palette = theme.extended_palette();
                        let error_state = match &self.screen {
                            Screen::StreamRunning(_, stream) => stream.errors().len() > 0,
                            _ => false
                        };
                        let base_style = container::Style {
                            border: iced::Border { color: iced::Color::TRANSPARENT, width: 0.0, radius: Radius::new(0.0).bottom(8.0) },
                            ..Default::default()
                        };
                        if error_state {
                            container::Style {
                                text_color: Some(palette.danger.base.text),
                                background: Some(palette.danger.base.color.into()),
                                ..base_style
                            }
                        } else {
                            container::Style {
                                text_color: Some(palette.secondary.base.text),
                                background: Some(palette.secondary.base.color.into()),
                                ..base_style
                            }
                        }
                    })
                    .into()
                } else {
                    header(
                        match self.screen {
                            Screen::Configure => HeaderScreen::Configure,
                            Screen::SetUp(_) => HeaderScreen::SetUp,
                            Screen::StreamRunning(_, _) | Screen::StreamStart(_, _) => HeaderScreen::Stream,
                            Screen::Welcome => unreachable!(),
                        },
                        !matches!(self.screen, Screen::StreamRunning(_, _)),
                        Message::SwitchScreen,
                        &self.profile.name,
                        Message::ProfileNameChange,
                        Message::TryImportProfile,
                        Message::ExportProfile,
                        Message::TryNewProfile,
                        matches!(self.screen, Screen::StreamRunning(_, _)).then_some(Message::EndStream),
                    )
                    .into()
                },
                match &self.screen {
                    Screen::Configure => configure(
                        &self.profile,
                        &self.sport_type_keys,
                        Message::HandleConfigureEvent,
                    )
                    .into(),
                    Screen::SetUp(public_token) => container(
                        scrollable(
                            rounded_pane(
                                column([
                                    text("Let's get set up.")
                                        .style(|theme: &iced::Theme| text::Style {
                                            color: Some(theme.palette().text),
                                        })
                                        .size(32)
                                        .into(),
                                    text("You only need to do this once per composition.")
                                        .style(|theme: &iced::Theme| text::Style {
                                            color: Some(theme.palette().text),
                                        })
                                        .size(16)
                                        .into(),
                                    text("Step 1: Find your public data stream token at app.singular.live/datastreammanager and copy it. Make sure it corresponds with the private token you entered.")
                                        .width(Length::Fill)
                                        .into(),
                                    container(
                                        rounded_button("Open data stream manager", utils::RoundedButtonVariant::Secondary)
                                        .on_press(Message::SetUpOpenDataStreams),
                                    )
                                    .width(Length::Fill)
                                    .align_x(iced::alignment::Horizontal::Right)
                                    .into(),
                                    text("Step 2: Paste it here and press the copy button to copy the root composition script to your clipboard.")
                                        .width(Length::Fill)
                                        .into(),
                                    row([
                                        text_input(
                                            "Your public data stream token",
                                            &public_token
                                        )
                                        .width(Length::Fill)
                                        .style(rounded_text_input_style)
                                        .on_input(Message::SetUpTokenUpdated)
                                        .padding(8)
                                        .into(),
                                        icon_button(
                                            include_bytes!("../assets/icon_content_copy.svg"),
                                            "Copy root composition script",
                                            Some(Message::SetUpCopyScript),
                                            utils::RoundedButtonVariant::Primary
                                        ).into()
                                    ])
                                    .spacing(4)
                                    .into(),
                                    text("Step 3: Head over to app.singular.live/dashboard. Right click on the composition and select \"Open script editor\".")
                                        .width(Length::Fill)
                                        .into(),
                                    container(
                                        rounded_button("Open dashboard", utils::RoundedButtonVariant::Secondary)
                                        .on_press(Message::SetUpOpenDashboard),
                                    )
                                    .width(Length::Fill)
                                    .align_x(iced::alignment::Horizontal::Right)
                                    .into(),
                                    text("Step 4: Press the plus button in the tabs at the top left. Click \"Root composition\".")
                                        .width(Length::Fill)
                                        .into(),
                                    text("Step 5: Paste the composition script from your clipboard and save the script (Ctrl + S).")
                                        .width(Length::Fill)
                                        .into(),
                                    text("Step 6 (optional): Try setting up a stream to test whether it worked.")
                                        .width(Length::Fill)
                                        .into(),
                                ])
                                .spacing(8)
                                .width(500)
                                .padding(12)
                                .align_x(Alignment::Center)
                            )
                        )
                    )
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
                    Screen::StreamRunning(stream_running, active_stream) => {
                        stream_running.view(active_stream).map(Message::StreamRunningMessage)
                    }
                    Screen::StreamStart(stream_start, error) => {
                        stream_start.view(error.as_deref(), self.profile_dirty).map(Message::StreamStartMessage)
                    }
                    Screen::Welcome => unreachable!(),
                },
            ])
            .align_x(Alignment::Center)
            .into()
        }
    }
}
