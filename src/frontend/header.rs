use iced::{
    border::Radius,
    widget::{
        button, column, component, container, horizontal_space, row, text, text_input,
        vertical_space, Component, Space,
    },
    Border, Color, Element, Length, Renderer, Shadow, Size, Theme,
};

use super::utils::{icon_button, rounded_button, rounded_text_input_style};

#[derive(Debug, Clone)]
pub enum HeaderScreen {
    Configure,
    Stream,
}

pub struct Header<'a, Message: Clone> {
    screen: HeaderScreen,
    enabled: bool,
    on_switch: Box<dyn Fn(HeaderScreen) -> Message>,
    profile_name: &'a str,
    on_profile_name_change: Box<dyn Fn(String) -> Message>,
    on_profile_import: Message,
    on_profile_export: Message,
    on_profile_add: Message,
    on_end_stream: Option<Message>,
}

impl<'a, Message: Clone> Header<'a, Message> {
    pub fn new(
        screen: HeaderScreen,
        enabled: bool,
        on_switch: impl Fn(HeaderScreen) -> Message + 'static,
        profile_name: &'a str,
        on_profile_name_change: impl Fn(String) -> Message + 'static,
        on_profile_import: Message,
        on_profile_export: Message,
        on_profile_add: Message,
        on_end_stream: Option<Message>,
    ) -> Header<Message> {
        Header::<Message> {
            screen,
            enabled,
            on_switch: Box::new(on_switch),
            profile_name,
            on_profile_name_change: Box::new(on_profile_name_change),
            on_profile_import,
            on_profile_export,
            on_profile_add,
            on_end_stream,
        }
    }
}

pub fn header<Message: Clone>(
    screen: HeaderScreen,
    enabled: bool,
    on_switch: impl Fn(HeaderScreen) -> Message + 'static,
    profile_name: &str,
    on_profile_name_change: impl Fn(String) -> Message + 'static,
    on_profile_import: Message,
    on_profile_export: Message,
    on_profile_add: Message,
    on_end_stream: Option<Message>,
) -> Header<Message> {
    Header::<Message>::new(
        screen,
        enabled,
        on_switch,
        profile_name,
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

fn tab_button_style(
    theme: &Theme,
    status: button::Status,
    radius: impl Into<Radius>,
    selected: bool,
) -> button::Style {
    match status {
        button::Status::Disabled | button::Status::Active => button::Style {
            background: Some(
                if selected {
                    theme.extended_palette().primary.base.color
                } else {
                    theme.extended_palette().primary.weak.color
                }
                .into(),
            ),
            shadow: Shadow::default(),
            text_color: if selected {
                theme.extended_palette().primary.base.text
            } else {
                theme.extended_palette().primary.weak.text
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius.into(),
            },
        },
        button::Status::Hovered => button::Style {
            background: Some(
                theme
                    .extended_palette()
                    .primary
                    .strong
                    .color
                    .scale_alpha(0.8)
                    .into(),
            ),
            shadow: Shadow::default(),
            text_color: theme.extended_palette().primary.strong.text,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius.into(),
            },
        },
        button::Status::Pressed => button::Style {
            background: Some(
                theme
                    .extended_palette()
                    .primary
                    .base
                    .color
                    .scale_alpha(0.8)
                    .into(),
            ),
            shadow: Shadow::default(),
            text_color: theme.extended_palette().primary.base.text,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius.into(),
            },
        },
    }
}

impl<Message: Clone> Component<Message> for Header<'_, Message> {
    type State = HeaderState;

    type Event = HeaderEvent;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            HeaderEvent::ScreenTabClicked(screen) => Some((self.on_switch)(screen)),
            HeaderEvent::ProfileNameChange(new_name) => {
                Some((self.on_profile_name_change)(new_name))
            }
            HeaderEvent::ProfileImport => Some(self.on_profile_import.clone()),
            HeaderEvent::ProfileExport => Some(self.on_profile_export.clone()),
            HeaderEvent::ProfileAdd => Some(self.on_profile_add.clone()),
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
                self.on_end_stream.clone()
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Theme, Renderer> {
        container(
            column([
                row([
                    text("Daktronics Singular UI").size(18).into(),
                    horizontal_space().into(),
                    {
                        let input = text_input("Profile name", self.profile_name)
                            .style(rounded_text_input_style)
                            .padding(8)
                            .width(300);
                        if self.enabled {
                            input.on_input(HeaderEvent::ProfileNameChange)
                        } else {
                            input
                        }
                    }
                    .into(),
                    icon_button(
                        include_bytes!("../../assets/icon_download.svg"),
                        "Import profile",
                        self.enabled.then_some(HeaderEvent::ProfileImport),
                        super::utils::RoundedButtonVariant::Secondary,
                    )
                    .into(),
                    icon_button(
                        include_bytes!("../../assets/icon_upload.svg"),
                        "Export profile",
                        self.enabled.then_some(HeaderEvent::ProfileExport),
                        super::utils::RoundedButtonVariant::Secondary,
                    )
                    .into(),
                    icon_button(
                        include_bytes!("../../assets/icon_add_circle.svg"),
                        "New profile",
                        self.enabled.then_some(HeaderEvent::ProfileAdd),
                        super::utils::RoundedButtonVariant::Secondary,
                    )
                    .into(),
                ])
                .align_items(iced::Alignment::Center)
                .padding(14)
                .spacing(4)
                .width(Length::Fill)
                .into(),
                row([container(row([
                    button(text("Configure"))
                        .style(|theme, status| {
                            tab_button_style(
                                theme,
                                status,
                                [256.0, 0.0, 0.0, 256.0],
                                matches!(self.screen, HeaderScreen::Configure),
                            )
                        })
                        .padding([10, 14])
                        .on_press_maybe(
                            self.enabled
                                .then_some(HeaderEvent::ScreenTabClicked(HeaderScreen::Configure)),
                        )
                        .into(),
                    button(text("Stream"))
                        .style(|theme, status| {
                            tab_button_style(
                                theme,
                                status,
                                [0.0, 256.0, 256.0, 0.0],
                                matches!(self.screen, HeaderScreen::Stream),
                            )
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
                .into()])
                .push_maybe(self.on_end_stream.is_some().then(|| {
                    container(
                        row(if state.is_showing_end_stream_confirm {
                            [
                                text("Are you sure you want to end the stream?").into(),
                                rounded_button(
                                    "Cancel",
                                    super::utils::RoundedButtonVariant::Secondary,
                                )
                                .on_press(HeaderEvent::EndStreamConfirmCancel)
                                .into(),
                                rounded_button(
                                    "Confirm",
                                    super::utils::RoundedButtonVariant::Danger,
                                )
                                .on_press(HeaderEvent::EndStreamConfirmYes)
                                .into(),
                            ]
                        } else {
                            [
                                text("Danger zone").into(),
                                Space::new(0, 0).into(),
                                rounded_button(
                                    "End stream",
                                    super::utils::RoundedButtonVariant::Primary,
                                )
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
                }))
                .into(),
                vertical_space().height(8).into(),
            ])
            .width(Length::Fill),
        )
        .style(|theme| {
            if self.enabled {
                container::transparent(theme)
            } else {
                container::transparent(theme)
                    .with_background(theme.extended_palette().primary.weak.color.scale_alpha(0.2))
            }
        })
        .into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }
}

impl<'a, Message: Clone> From<Header<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: Header<'a, Message>) -> Self {
        component(header)
    }
}
