use iced::{
    widget::{
        checkbox, column, component, container, horizontal_rule, pick_list, row, scrollable, svg,
        text, text_input, Component,
    },
    Border, Element, Length, Renderer, Size, Theme,
};

use crate::backend::{
    mapping::transformation::Transformation, profile::Profile, sports::DynamicSportType,
};

use super::utils::{
    icon_button, rounded_button, rounded_pick_list_style, rounded_text_input_style,
};

pub struct Configure<'a, Message> {
    profile: &'a Profile,
    sport_type_keys: &'a Vec<String>,
    on_event: Box<dyn Fn(ConfigureEvent) -> Message>,
}

impl<'a, Message> Configure<'a, Message> {
    pub fn new(
        profile: &'a Profile,
        sport_type_keys: &'a Vec<String>,
        on_event: impl Fn(ConfigureEvent) -> Message + 'static,
    ) -> Configure<'a, Message> {
        Configure {
            profile,
            sport_type_keys,
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
    ExcludeIncompleteDataUpdated(bool),
    MappingItemAdded,
    MappingItemRemoved(usize),
    MappingItemEnabledUpdated(usize, bool),
    MappingItemSourceFieldUpdated(usize, String),
    MappingItemTransformationUpdated(usize, Transformation),
    MappingItemDestinationFieldUpdated(usize, String),
}

pub fn configure<'a, Message>(
    profile: &'a Profile,
    sport_type_keys: &'a Vec<String>,
    on_event: impl Fn(ConfigureEvent) -> Message + 'static,
) -> Configure<'a, Message> {
    Configure::new(&profile, &sport_type_keys, on_event)
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
                        .width(Length::Fill)
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
                        .width(Length::Fill)
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
                        .width(Length::Fill)
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
                checkbox("Allow concurrent updates to the server", self.profile.multiple_requests)
                    .on_toggle(ConfigureEvent::MultipleRequestsUpdated)
                    .into(),
                checkbox("Exclude incomplete data from payload instead of erroring", self.profile.exclude_incomplete_data)
                    .on_toggle(ConfigureEvent::ExcludeIncompleteDataUpdated)
                    .into(),
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
                        pick_list(
                            self.sport_type_keys.as_ref(),
                            Some(&item.source_field),
                            move |new| ConfigureEvent::MappingItemSourceFieldUpdated(i, new)
                        )
                        .width(Length::Fill)
                        .padding(8)
                        .style(rounded_pick_list_style)
                        .into(),
                        // It would be nice to use a combo box, but combo box
                        // requires external state to be stored. This could be
                        // extracted into a Component but there are lifetime
                        // issues when a Component's State is borrowed in its
                        // view method.
                        //
                        // See https://discourse.iced.rs/t/lifetime-problems-when-using-iced-component/383
                        pick_list(
                            Transformation::ALL,
                            Some(item.transformation),
                            move |new| ConfigureEvent::MappingItemTransformationUpdated(i, new)
                        )
                        .width(Length::Fill)
                        .padding(8)
                        .style(rounded_pick_list_style)
                        .into(),
                        text_input(
                            "Destination field",
                            &item.destination_field
                        )
                        .width(Length::Fill)
                        .padding(8)
                        .on_input(move |new| ConfigureEvent::MappingItemDestinationFieldUpdated(i, new))
                        .style(rounded_text_input_style)
                        .into(),
                    ])
                    .spacing(8)
                    .align_y(iced::Alignment::Center)
                    .into()
                }))
                .spacing(8)
                .into()
            ])
            .push_maybe(self.profile.mapping.items.is_empty().then(|| {
                column([
                    text("You don't have any mappings yet.").into(),
                    container(
                        row([
                            container(
                                svg(svg::Handle::from_memory(include_bytes!(
                                    "../../assets/icon_question_mark.svg"
                                )))
                                .style(|theme: &Theme, _| svg::Style {
                                    color: Some(theme.palette().primary),
                                })
                                .content_fit(iced::ContentFit::Fill),
                            )
                            .width(24)
                            .height(24)
                            .into(),
                            column([
                                text("What's a mapping?")
                                    .style(text::primary)
                                    .into(),
                                    text("A mapping converts the map of data produced from the control console into a format intended for your Singular composition. Daktronics Singular UI provides transformations useful for parsing times and period names to help you convert the raw data into the right type of text. Press the button below to create a new one.")
                                        .into(),
                            ]).into()
                        ])
                        .align_y(iced::Alignment::Center)
                        .spacing(8),
                    )
                    .style(|theme: &iced::Theme| container::Style {
                        background: None,
                        text_color: None,
                        shadow: Default::default(),
                        border: Border {
                            color: theme.palette().primary,
                            width: 1.0,
                            radius: 18.into(),
                        },
                    })
                    .padding([8, 16])
                    .into()
                ])
                .spacing(8)
                .align_x(iced::Alignment::Center)
                .width(Length::Fill)
            }))
            .push(
                container(
                    rounded_button("New mapping", super::utils::RoundedButtonVariant::Secondary)
                    .on_press(ConfigureEvent::MappingItemAdded)
                )
                .center_x(Length::Fill)
            )
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
