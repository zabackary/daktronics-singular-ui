mod configure;
mod header;
mod stream_running;
mod stream_start;
mod utils;

use configure::{configure, ConfigureEvent};
use header::{header, HeaderScreen};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Task};
use stream_running::stream_running;
use stream_start::stream_start;
use utils::rounded_button;

use crate::backend::profile::Profile;
use crate::backend::stream::{ActiveStream, WorkerEvent};

#[derive(Debug, Default)]
pub struct DaktronicsSingularUiApp {
    screen: Screen,
    profile: Profile,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewProfile,
    ImportProfile,
    ExportProfile,
    WelcomeNewProfile,
    WelcomeImportProfile,
    StartStream(String),
    EndStream,
    SwitchScreen(HeaderScreen),
    ProfileNameChange(String),
    UpdateStreamStats,
    UpdateStreamStatsResponse(Vec<WorkerEvent>),
    ClearStreamErrors,
    HandleConfigureEvent(ConfigureEvent),
}

#[derive(Debug, Default)]
enum Screen {
    Configure,
    Stream(ActiveStream),
    StreamStart(Option<String>),
    #[default]
    Welcome,
}

impl DaktronicsSingularUiApp {
    pub fn update(&mut self, message: Message) -> impl Into<Task<Message>> {
        match message {
            Message::ExportProfile => {
                todo!()
            }
            Message::ImportProfile => {
                todo!()
            }
            Message::NewProfile => {
                self.profile = Profile::default();
                Task::none()
            }
            Message::WelcomeImportProfile => {
                todo!()
            }
            Message::WelcomeNewProfile => {
                self.screen = Screen::Configure;
                Task::none()
            }
            Message::StartStream(tty_path) => match self.screen {
                Screen::StreamStart(ref mut error) => {
                    if self.profile.sport_type.is_some() {
                        match ActiveStream::new(self.profile.to_owned(), tty_path) {
                            Ok(stream) => {
                                self.screen = Screen::Stream(stream);
                            }
                            Err(err) => *error = Some(err.to_string()),
                        }
                        Task::done(Message::UpdateStreamStats)
                    } else {
                        *error = Some(
                            "You must set a sport type before beginning the stream.".to_owned(),
                        );
                        Task::none()
                    }
                }
                _ => Task::none(),
            },
            Message::EndStream => {
                // Drop the stream, killing the background threads implicitly
                self.screen = Screen::StreamStart(None);
                Task::none()
            }
            Message::ProfileNameChange(new_name) => {
                self.profile.name = new_name;
                Task::none()
            }
            Message::SwitchScreen(new_screen) => {
                self.screen = match new_screen {
                    HeaderScreen::Configure => Screen::Configure,
                    HeaderScreen::Stream => Screen::StreamStart(None),
                };
                Task::none()
            }
            Message::UpdateStreamStats => match self.screen {
                Screen::Stream(ref mut active_stream) => {
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
                Screen::Stream(ref mut active_stream) => {
                    active_stream.update_from_events(events);
                    Task::done(Message::UpdateStreamStats)
                }
                _ => Task::none(),
            },
            Message::ClearStreamErrors => match self.screen {
                Screen::Stream(ref mut active_stream) => {
                    active_stream.clear_errors();
                    Task::none()
                }
                _ => Task::none(),
            },
            Message::HandleConfigureEvent(event) => {
                match event {
                    ConfigureEvent::DataStreamUrlUpdated(new) => self.profile.data_stream_url = new,
                    ConfigureEvent::SubcompNameUpdated(new) => self.profile.subcomp_name = new,
                    ConfigureEvent::SportTypeUpdated(new) => self.profile.sport_type = Some(new),
                    ConfigureEvent::MultipleRequestsUpdated(new) => {
                        self.profile.multiple_requests = new
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
        }
    }

    pub fn view(&self) -> Element<Message> {
        if matches!(self.screen, Screen::Welcome) {
            container(
                column([
                    text(concat!(
                        "Daktronics Singular UI v",
                        env!("CARGO_PKG_VERSION")
                    ))
                    .size(18)
                    .style(|theme: &iced::Theme| text::Style {
                        color: Some(theme.palette().text.scale_alpha(0.6)),
                    })
                    .into(),
                    text("Welcome.")
                        .style(|theme: &iced::Theme| text::Style {
                            color: Some(theme.palette().text),
                        })
                        .size(76)
                        .into(),
                    row([
                        rounded_button(
                            text("Import profile").size(18),
                            utils::RoundedButtonVariant::Primary,
                        )
                        .on_press(Message::WelcomeImportProfile)
                        .into(),
                        rounded_button(
                            text("New profile").size(18),
                            utils::RoundedButtonVariant::Primary,
                        )
                        .on_press(Message::WelcomeNewProfile)
                        .into(),
                    ])
                    .spacing(8)
                    .into(),
                ])
                .spacing(16)
                .align_items(Alignment::Start),
            )
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            column([
                header(
                    match self.screen {
                        Screen::Configure => HeaderScreen::Configure,
                        Screen::Stream(_) | Screen::StreamStart(_) => HeaderScreen::Stream,
                        Screen::Welcome => unreachable!(),
                    },
                    !matches!(self.screen, Screen::Stream(_)),
                    Message::SwitchScreen,
                    &self.profile.name,
                    Message::ProfileNameChange,
                    Message::ImportProfile,
                    Message::ExportProfile,
                    Message::NewProfile,
                    matches!(self.screen, Screen::Stream(_)).then_some(Message::EndStream),
                )
                .into(),
                match &self.screen {
                    Screen::Configure => {
                        configure(&self.profile, Message::HandleConfigureEvent).into()
                    }
                    Screen::Stream(active_stream) => {
                        stream_running(active_stream, Message::ClearStreamErrors).into()
                    }
                    Screen::StreamStart(error) => {
                        stream_start(Message::StartStream, error.as_deref()).into()
                    }
                    Screen::Welcome => unreachable!(),
                },
            ])
            .align_items(Alignment::Center)
            .into()
        }
    }
}
