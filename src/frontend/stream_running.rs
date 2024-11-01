use iced::{
    widget::{column, container, horizontal_space, row, scrollable, text},
    Element, Font, Length, Padding, Renderer, Theme,
};

use crate::backend::stream::ActiveStream;

use super::{
    graph::graph,
    utils::{icon_button, rounded_pane},
};

#[derive(Debug, Clone, Copy)]
pub struct StreamRunning {
    _no_public_constructor: (),
}

#[derive(Clone, Debug, Copy)]
pub enum StreamRunningMessage {
    ClearErrors,
}

fn pane_header<'a, Message: 'a>(
    label: &'a str,
    stat: i32,
    unit: &'a str,
    button: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
) -> Element<'a, Message, Theme, Renderer> {
    row([
        text(label).width(Length::Fill).size(18).into(),
        text(stat)
            .size(32)
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().secondary.strong.color),
            })
            .into(),
        text(unit)
            .size(14)
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().secondary.strong.color),
            })
            .into(),
    ])
    .push_maybe(button.is_some().then(|| horizontal_space().width(4)))
    .push_maybe(button)
    .height(54)
    .align_y(iced::Alignment::Center)
    .padding(Padding {
        left: 4.0,
        right: 4.0,
        ..Padding::ZERO
    })
    .spacing(2)
    .into()
}

pub enum Update {
    #[allow(dead_code)]
    None,
    ClearErrors,
}

impl StreamRunning {
    pub fn new() -> Self {
        Self {
            _no_public_constructor: (),
        }
    }

    pub fn update(&mut self, message: StreamRunningMessage) -> Update {
        match message {
            StreamRunningMessage::ClearErrors => Update::ClearErrors,
        }
    }

    pub fn view<'a>(
        &'a self,
        active_stream: &'a ActiveStream,
    ) -> Element<'a, StreamRunningMessage, Theme, Renderer> {
        let latency_pane = column([
            pane_header(
                "Latency",
                active_stream
                    .latency_graph_data()
                    .samples
                    .last()
                    .map(|x| x.latency.as_millis().try_into().unwrap_or(i32::MAX))
                    .unwrap_or(0),
                "ms",
                // not sure why Rust needs annotations but whatever
                Option::<&str>::None,
            ),
            rounded_pane(
                container(graph(&active_stream))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(16),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        ]);
        let payload_pane = {
            column([
                pane_header(
                    "Latest payload",
                    active_stream
                        .latest_payload_size()
                        .unwrap_or(0)
                        .try_into()
                        .unwrap_or(i32::MAX),
                    "B",
                    // not sure why Rust needs annotations but whatever
                    // &str can be `into`'ed into an Element, so it works
                    Option::<&str>::None,
                ),
                rounded_pane(scrollable(
                    container(
                        text(active_stream.latest_payload().unwrap_or("No data"))
                            .font(Font::MONOSPACE),
                    )
                    .width(Length::Fill)
                    .padding(16),
                ))
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            ])
        };
        let error_pane = (!active_stream.errors().is_empty()).then(|| {
            column([
                pane_header(
                    "Errors (last 20)",
                    active_stream.errors().len().try_into().unwrap_or(i32::MAX),
                    "x",
                    Some(icon_button(
                        include_bytes!("../../assets/icon_delete.svg"),
                        "Clear messages",
                        Some(StreamRunningMessage::ClearErrors),
                        super::utils::RoundedButtonVariant::Secondary,
                    )),
                ),
                scrollable(
                    column(active_stream.errors().iter().map(|error| {
                        rounded_pane(
                            column([
                                text(timeago::Formatter::new().convert(error.timestamp.elapsed()))
                                    .size(12)
                                    .into(),
                                text(&error.msg).font(Font::MONOSPACE).into(),
                            ])
                            .padding(8)
                            .width(Length::Fill)
                            .spacing(4),
                        )
                        .into()
                    }))
                    .spacing(8),
                )
                .into(),
            ])
        });
        row([latency_pane.into(), payload_pane.into()])
            .push_maybe(error_pane)
            .spacing(12)
            .padding(Padding::new(16.0).top(0.0))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
