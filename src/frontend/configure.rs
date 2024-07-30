use iced::{
    widget::{
        checkbox, column, component, container, horizontal_rule, pick_list, row, scrollable, text,
        text_input, Component,
    },
    Element, Length, Renderer, Size, Theme,
};

use crate::backend::{
    mapping::transformation::Transformation, profile::Profile, sports::DynamicSportType,
};

use super::utils::{
    icon_button, rounded_button, rounded_pick_list_style, rounded_text_input_style,
};

pub struct Configure<'a, Message> {
    profile: &'a Profile,
    on_event: Box<dyn Fn(ConfigureEvent) -> Message>,
}

impl<'a, Message> Configure<'a, Message> {
    pub fn new(
        profile: &'a Profile,
        on_event: impl Fn(ConfigureEvent) -> Message + 'static,
    ) -> Configure<'a, Message> {
        Configure {
            profile,
            on_event: Box::new(on_event),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConfigureEvent {
    DataStreamUrlUpdated(String),
    SubcompNameUpdated(String),
    SportTypeUpdated(DynamicSportType),
    MultipleRequestsUpdated(bool),
    MappingItemAdded,
    MappingItemRemoved(usize),
    MappingItemEnabledUpdated(usize, bool),
    MappingItemSourceFieldUpdated(usize, String),
    MappingItemTransformationUpdated(usize, Transformation),
    MappingItemDestinationFieldUpdated(usize, String),
}

pub fn configure<'a, Message>(
    profile: &'a Profile,
    on_event: impl Fn(ConfigureEvent) -> Message + 'static,
) -> Configure<'a, Message> {
    Configure::new(&profile, on_event)
}

impl<'a, Message: Clone> Component<Message> for Configure<'a, Message> {
    type State = ();

    type Event = ConfigureEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some((self.on_event)(event))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Theme, Renderer> {
        scrollable(
            column([
                row([
                    column([
                        text("Data stream private URL")
                            .style(|theme: &Theme| text::Style {
                                color: Some(theme.palette().text.scale_alpha(0.6))
                            })
                            .into(),
                        text_input(
                            "https://datastream.singular.live/datastreams/ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                            &self.profile.data_stream_url,
                        )
                        .padding(8)
                        .on_input(ConfigureEvent::DataStreamUrlUpdated)
                        .style(rounded_text_input_style)
                        .into(),
                    ])
                    .spacing(4)
                    .into(),
                    column([
                        text("Sport")
                            .style(|theme: &Theme| text::Style {
                                color: Some(theme.palette().text.scale_alpha(0.6))
                            })
                            .into(),
                        pick_list(
                            DynamicSportType::ALL,
                            self.profile.sport_type.as_ref(),
                            ConfigureEvent::SportTypeUpdated,
                        )
                        .padding(8)
                        .style(rounded_pick_list_style)
                        .into(),
                    ])
                    .spacing(4)
                    .into(),
                    column([
                        text("Subcomp name")
                            .style(|theme: &Theme| text::Style {
                                color: Some(theme.palette().text.scale_alpha(0.6))
                            })
                            .into(),
                        text_input(
                            "Score Bug",
                            &self.profile.subcomp_name,
                        )
                        .padding(8)
                        .on_input(ConfigureEvent::SubcompNameUpdated)
                        .style(rounded_text_input_style)
                        .into(),
                    ])
                    .spacing(4)
                    .into(),
                ])
                .spacing(8)
                .into(),
                row([
                    checkbox("Allow concurrent updates to the server", self.profile.multiple_requests)
                    .on_toggle(ConfigureEvent::MultipleRequestsUpdated)
                    .into()
                ]).into(),
                horizontal_rule(2.0).into(),
                column(self.profile.mapping.items.iter().enumerate().map(|(i, item)| {
                    row([
                        Element::from(
                            icon_button(
                                include_bytes!("../../assets/icon_delete.svg"),
                                "Remove mapping",
                                Some(()),
                                super::utils::RoundedButtonVariant::Danger
                            )
                        )
                        .map(move |_| ConfigureEvent::MappingItemRemoved(i)),
                        Element::from(
                            icon_button(
                                if item.enabled {
                                    include_bytes!("../../assets/icon_check_box.svg")
                                } else {
                                    include_bytes!("../../assets/icon_check_box_outline_blank.svg")
                                },
                                if item.enabled {
                                    "Disable mapping"
                                } else {
                                    "Enable mapping"
                                },
                                Some(()),
                                super::utils::RoundedButtonVariant::Secondary
                            )
                        )
                        .map(move |_| ConfigureEvent::MappingItemEnabledUpdated(i, !item.enabled))
                        .into(),
                        text_input(
                            "Source field",
                            &item.source_field
                        )
                        .padding(8)
                        .on_input(move |new| ConfigureEvent::MappingItemSourceFieldUpdated(i, new))
                        .style(rounded_text_input_style)
                        .into(),
                        pick_list(
                            Transformation::ALL,
                            Some(item.transformation),
                            move |new| ConfigureEvent::MappingItemTransformationUpdated(i, new)
                        )
                        .padding(8)
                        .style(rounded_pick_list_style)
                        .into(),
                        text_input(
                            "Destination field",
                            &item.destination_field
                        )
                        .padding(8)
                        .on_input(move |new| ConfigureEvent::MappingItemDestinationFieldUpdated(i, new))
                        .style(rounded_text_input_style)
                        .into(),
                    ])
                    .spacing(8)
                    .align_items(iced::Alignment::Center)
                    .into()
                }))
                .spacing(8)
                .into(),
                container(
                    rounded_button("New mapping", super::utils::RoundedButtonVariant::Secondary)
                    .on_press(ConfigureEvent::MappingItemAdded)
                )
                .center_x(Length::Fill)
                .into()
            ])
            .spacing(8)
            .padding(16)
            .width(Length::Fill),
        )
        .height(Length::Fill)
        .into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }
}

impl<'a, Message: Clone> From<Configure<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: Configure<'a, Message>) -> Self {
        component(header)
    }
}
