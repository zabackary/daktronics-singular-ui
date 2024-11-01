use iced::{
    border::Radius,
    widget::{
        button, column, container, horizontal_space, row, svg, text, text_input, vertical_space,
        Space,
    },
    Border, Color, Element, Length, Padding, Shadow, Theme,
};

use crate::APP_NAME;

use super::utils::{icon_button, rounded_button, rounded_text_input_style};

#[derive(Debug, Clone, Copy)]
pub enum HeaderScreen {
    Configure,
    SetUp,
    Stream,
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
    is_showing_end_stream_confirm: bool,
}

#[derive(Debug, Clone)]
pub enum HeaderMessage {
    ScreenTabClicked(HeaderScreen),

    ProfileNameChange(String),
    ProfileImport,
    ProfileExport,
    ProfileAdd,
    OpenEndStreamConfirm,
    EndStreamConfirmYes,
    EndStreamConfirmCancel,
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

pub enum Update {
    None,
    Switch(HeaderScreen),
    ChangeProfileName(String),
    ImportProfile,
    ExportProfile,
    NewProfile,
    EndStream,
}

impl Header {
    pub fn new() -> Self {
        Self {
            is_showing_end_stream_confirm: false,
        }
    }

    pub fn update(&mut self, message: HeaderMessage) -> Update {
        match message {
            HeaderMessage::ScreenTabClicked(screen) => Update::Switch(screen),
            HeaderMessage::ProfileNameChange(new_name) => Update::ChangeProfileName(new_name),
            HeaderMessage::ProfileImport => Update::ImportProfile,
            HeaderMessage::ProfileExport => Update::ExportProfile,
            HeaderMessage::ProfileAdd => Update::NewProfile,
            HeaderMessage::OpenEndStreamConfirm => {
                self.is_showing_end_stream_confirm = true;
                Update::None
            }
            HeaderMessage::EndStreamConfirmCancel => {
                self.is_showing_end_stream_confirm = false;
                Update::None
            }
            HeaderMessage::EndStreamConfirmYes => {
                self.is_showing_end_stream_confirm = false;
                Update::EndStream
            }
        }
    }

    pub fn view<'a>(
        &self,
        enabled: bool,
        header_screen: HeaderScreen,
        show_end_stream: bool,
        profile_name: &'a str,
    ) -> Element<'a, HeaderMessage> {
        container(
            column([
                row([
                    container(
                        svg(svg::Handle::from_memory(include_bytes!(
                            "../../assets/logo.svg"
                        )))
                        .content_fit(iced::ContentFit::Fill),
                    )
                    .width(24)
                    .height(24)
                    .into(),
                    text(APP_NAME).size(18).into(),
                    horizontal_space().into(),
                    {
                        let input = text_input("Profile name", profile_name)
                            .style(rounded_text_input_style)
                            .padding(8)
                            .width(300);
                        if enabled {
                            input.on_input(HeaderMessage::ProfileNameChange)
                        } else {
                            input
                        }
                    }
                    .into(),
                    icon_button(
                        include_bytes!("../../assets/icon_download.svg"),
                        "Import profile",
                        enabled.then_some(HeaderMessage::ProfileImport),
                        super::utils::RoundedButtonVariant::Secondary,
                    )
                    .into(),
                    icon_button(
                        include_bytes!("../../assets/icon_upload.svg"),
                        "Export profile",
                        enabled.then_some(HeaderMessage::ProfileExport),
                        super::utils::RoundedButtonVariant::Secondary,
                    )
                    .into(),
                    icon_button(
                        include_bytes!("../../assets/icon_add_circle.svg"),
                        "New profile",
                        enabled.then_some(HeaderMessage::ProfileAdd),
                        super::utils::RoundedButtonVariant::Secondary,
                    )
                    .into(),
                ])
                .align_y(iced::Alignment::Center)
                .padding(14)
                .spacing(4)
                .width(Length::Fill)
                .into(),
                row([container(row([
                    button(text("Configure"))
                        .style(move |theme, status| {
                            tab_button_style(
                                theme,
                                status,
                                Radius {
                                    top_left: 256.0,
                                    bottom_left: 256.0,
                                    top_right: 0.0,
                                    bottom_right: 0.0,
                                },
                                matches!(header_screen, HeaderScreen::Configure),
                            )
                        })
                        .padding([10, 14])
                        .on_press_maybe(
                            enabled.then_some(HeaderMessage::ScreenTabClicked(
                                HeaderScreen::Configure,
                            )),
                        )
                        .into(),
                    button(text("Set up"))
                        .style(move |theme, status| {
                            tab_button_style(
                                theme,
                                status,
                                Radius {
                                    top_left: 0.0,
                                    bottom_left: 0.0,
                                    top_right: 0.0,
                                    bottom_right: 0.0,
                                },
                                matches!(header_screen, HeaderScreen::SetUp),
                            )
                        })
                        .padding([10, 14])
                        .on_press_maybe(
                            enabled.then_some(HeaderMessage::ScreenTabClicked(HeaderScreen::SetUp)),
                        )
                        .into(),
                    button(text("Stream"))
                        .style(move |theme, status| {
                            tab_button_style(
                                theme,
                                status,
                                Radius {
                                    top_right: 256.0,
                                    bottom_right: 256.0,
                                    top_left: 0.0,
                                    bottom_left: 0.0,
                                },
                                matches!(header_screen, HeaderScreen::Stream),
                            )
                        })
                        .padding([10, 14])
                        .on_press_maybe(
                            enabled
                                .then_some(HeaderMessage::ScreenTabClicked(HeaderScreen::Stream)),
                        )
                        .into(),
                ]))
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .into()])
                .push_maybe(show_end_stream.then(|| {
                    container(
                        row(if self.is_showing_end_stream_confirm {
                            [
                                text("Are you sure you want to end the stream?").into(),
                                rounded_button(
                                    "Cancel",
                                    super::utils::RoundedButtonVariant::Secondary,
                                )
                                .on_press(HeaderMessage::EndStreamConfirmCancel)
                                .into(),
                                rounded_button(
                                    "Confirm",
                                    super::utils::RoundedButtonVariant::Danger,
                                )
                                .on_press(HeaderMessage::EndStreamConfirmYes)
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
                                .on_press(HeaderMessage::OpenEndStreamConfirm)
                                .into(),
                            ]
                        })
                        .align_y(iced::Alignment::Center)
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
                    .padding(Padding {
                        left: 12.0,
                        bottom: 4.0,
                        right: 4.0,
                        top: 4.0,
                    })
                }))
                .into(),
                vertical_space().height(8).into(),
            ])
            .width(Length::Fill),
        )
        .style(move |theme| {
            if enabled {
                container::transparent(theme)
            } else {
                container::transparent(theme)
                    .background(theme.extended_palette().primary.weak.color.scale_alpha(0.2))
            }
        })
        .into()
    }
}
