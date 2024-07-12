use iced::{
    border::Radius,
    widget::{
        button, column, component, container, horizontal_rule, horizontal_space, row, text,
        text_input, Component, Space,
    },
    Border, Element, Length, Renderer, Shadow, Size, Theme,
};

use super::utils::{icon_button, BORDER_RADIUS};

#[derive(Debug, Clone)]
pub enum HeaderScreen {
    Configure,
    Stream,
}

pub struct Header<Message> {
    screen: HeaderScreen,
    enabled: bool,
    on_switch: Box<dyn Fn(HeaderScreen) -> Message>,
    on_profile_name_change: Box<dyn Fn(String) -> Message>,
    on_profile_import: Box<dyn Fn() -> Message>,
    on_profile_export: Box<dyn Fn() -> Message>,
    on_profile_add: Box<dyn Fn() -> Message>,
    on_end_stream: Option<Box<dyn Fn() -> Message>>,
}

impl<Message> Header<Message> {
    pub fn new(
        screen: HeaderScreen,
        enabled: bool,
        on_switch: impl Fn(HeaderScreen) -> Message + 'static,
        on_profile_name_change: impl Fn(String) -> Message + 'static,
        on_profile_import: impl Fn() -> Message + 'static,
        on_profile_export: impl Fn() -> Message + 'static,
        on_profile_add: impl Fn() -> Message + 'static,
        on_end_stream: Option<impl Fn() -> Message + 'static>,
    ) -> Header<Message> {
        Header::<Message> {
            screen,
            enabled,
            on_switch: Box::new(on_switch),
            on_profile_name_change: Box::new(on_profile_name_change),
            on_profile_import: Box::new(on_profile_import),
            on_profile_export: Box::new(on_profile_export),
            on_profile_add: Box::new(on_profile_add),
            on_end_stream: match on_end_stream {
                Some(x) => Some(Box::new(x)),
                None => None,
            },
        }
    }
}

pub fn header<Message>(
    screen: HeaderScreen,
    enabled: bool,
    on_switch: impl Fn(HeaderScreen) -> Message + 'static,
    on_profile_name_change: impl Fn(String) -> Message + 'static,
    on_profile_import: impl Fn() -> Message + 'static,
    on_profile_export: impl Fn() -> Message + 'static,
    on_profile_add: impl Fn() -> Message + 'static,
    on_end_stream: Option<impl Fn() -> Message + 'static>,
) -> Header<Message> {
    Header::<Message>::new(
        screen,
        enabled,
        on_switch,
        on_profile_name_change,
        on_profile_import,
        on_profile_export,
        on_profile_add,
        on_end_stream,
    )
}

#[derive(Debug, Clone)]
pub enum HeaderEvent {
    ScreenTabClicked(HeaderScreen),
    ProfileNameChange(String),
    ProfileImport,
    ProfileExport,
    ProfileAdd,
    OpenEndStreamConfirm,
    EndStreamConfirmYes,
    EndStreamConfirmCancel,
}

pub struct HeaderState {
    is_showing_end_stream_confirm: bool,
}

impl Default for HeaderState {
    fn default() -> Self {
        HeaderState {
            is_showing_end_stream_confirm: false,
        }
    }
}

impl<Message> Component<Message> for Header<Message> {
    type State = HeaderState;

    type Event = HeaderEvent;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            HeaderEvent::ScreenTabClicked(screen) => Some((self.on_switch)(screen)),
            HeaderEvent::ProfileNameChange(new_name) => {
                Some((self.on_profile_name_change)(new_name))
            }
            HeaderEvent::ProfileImport => Some((self.on_profile_import)()),
            HeaderEvent::ProfileExport => Some((self.on_profile_export)()),
            HeaderEvent::ProfileAdd => Some((self.on_profile_add)()),
            HeaderEvent::OpenEndStreamConfirm => {
                state.is_showing_end_stream_confirm = true;
                None
            }
            HeaderEvent::EndStreamConfirmCancel => {
                state.is_showing_end_stream_confirm = false;
                None
            }
            HeaderEvent::EndStreamConfirmYes => {
                state.is_showing_end_stream_confirm = false;
                self.on_end_stream.as_ref().map(|f| (f)())
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Theme, Renderer> {
        column([
            row([
                text("Daktronics Singular UI").size(18).into(),
                horizontal_space().into(),
                {
                    let input = text_input("Profile name", "")
                        .style(|theme, status| {
                            let mut style = text_input::default(theme, status);
                            style.border.radius = Radius::from(BORDER_RADIUS);
                            style
                        })
                        .padding(8)
                        .width(300);
                    if self.enabled {
                        input.on_input(HeaderEvent::ProfileNameChange)
                    } else {
                        input
                    }
                }
                .into(),
                icon_button(include_bytes!("../../assets/icon_download.svg"))
                    .on_press_maybe(self.enabled.then_some(HeaderEvent::ProfileImport))
                    .into(),
                icon_button(include_bytes!("../../assets/icon_upload.svg"))
                    .on_press_maybe(self.enabled.then_some(HeaderEvent::ProfileExport))
                    .into(),
                icon_button(include_bytes!("../../assets/icon_add_circle.svg"))
                    .on_press_maybe(self.enabled.then_some(HeaderEvent::ProfileAdd))
                    .into(),
            ])
            .align_items(iced::Alignment::Center)
            .padding(14)
            .spacing(4)
            .width(Length::Fill)
            .into(),
            row([
                container(row([
                    button(text("Configure"))
                        .style(|theme, status| {
                            let mut style = if matches!(self.screen, HeaderScreen::Configure) {
                                button::secondary(theme, status)
                            } else {
                                button::text(theme, status)
                            };
                            style.border.radius =
                                Radius::from([BORDER_RADIUS, BORDER_RADIUS, 0.0, 0.0]);
                            style
                        })
                        .padding([10, 14])
                        .on_press_maybe(
                            self.enabled
                                .then_some(HeaderEvent::ScreenTabClicked(HeaderScreen::Configure)),
                        )
                        .into(),
                    button(text("Stream"))
                        .style(|theme, status| {
                            let mut style = if matches!(self.screen, HeaderScreen::Stream) {
                                button::secondary(theme, status)
                            } else {
                                button::text(theme, status)
                            };
                            style.border.radius =
                                Radius::from([BORDER_RADIUS, BORDER_RADIUS, 0.0, 0.0]);
                            style
                        })
                        .padding([10, 14])
                        .on_press_maybe(
                            self.enabled
                                .then_some(HeaderEvent::ScreenTabClicked(HeaderScreen::Stream)),
                        )
                        .into(),
                ]))
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .into(),
                if self.on_end_stream.is_some() {
                    container(
                        row(if state.is_showing_end_stream_confirm {
                            [
                                text("Are you sure you want to end the stream?").into(),
                                button(text("Cancel"))
                                    .style(|theme, status| {
                                        let mut style = button::secondary(theme, status);
                                        style.border.radius = 999.into();
                                        style
                                    })
                                    .on_press(HeaderEvent::EndStreamConfirmCancel)
                                    .into(),
                                button(text("Confirm"))
                                    .style(|theme, status| {
                                        let mut style = button::danger(theme, status);
                                        style.border.radius = Radius::from(999);
                                        style
                                    })
                                    .on_press(HeaderEvent::EndStreamConfirmYes)
                                    .into(),
                            ]
                        } else {
                            [
                                text("Danger zone").into(),
                                Space::new(0, 0).into(),
                                button(text("End stream"))
                                    .style(|theme, status| {
                                        let mut style = button::danger(theme, status);
                                        style.border.radius = 999.into();
                                        style
                                    })
                                    .on_press(HeaderEvent::OpenEndStreamConfirm)
                                    .into(),
                            ]
                        })
                        .align_items(iced::Alignment::Center)
                        .spacing(4),
                    )
                    .style(|theme| container::Style {
                        background: None,
                        text_color: None,
                        shadow: Shadow::default(),
                        border: Border {
                            color: theme.palette().danger,
                            width: 1.0,
                            radius: 999.into(),
                        },
                    })
                    .padding([4, 4, 4, 12])
                    .into()
                } else {
                    Space::new(0, 0).into()
                },
            ])
            .into(),
            horizontal_rule(2).into(),
        ])
        .width(Length::Fill)
        .into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }
}

impl<'a, Message> From<Header<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: Header<Message>) -> Self {
        component(header)
    }
}
