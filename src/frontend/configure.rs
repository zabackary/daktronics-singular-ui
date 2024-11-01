use iced::{
    border,
    widget::{
        checkbox, column, container, horizontal_rule, pick_list, row, scrollable, svg, text,
        text_input,
    },
    Border, Element, Length, Renderer, Theme,
};

use crate::backend::{
    mapping::transformation::Transformation,
    profile::{Profile, ProfileCompositionMapping},
    sports::DynamicSportType,
};

use super::utils::{
    icon_button, rounded_button, rounded_pick_list_style, rounded_text_input_style, BORDER_RADIUS,
};

#[derive(Clone, Debug)]
pub enum ProfileCompositionMessage {
    ItemAdded,
    ItemRemoved(usize),
    ItemEnabledUpdated(usize, bool),
    ItemSourceFieldUpdated(usize, String),
    ItemTransformationUpdated(usize, Transformation),
    ItemDestinationFieldUpdated(usize, String),
    SubcompNameUpdated(String),
    CheckboxNameUpdated(String),
}

trait ProfileCompositionMappingExt {
    fn view<'a>(
        &'a self,
        sport_type_keys: &'a Vec<String>,
    ) -> iced::Element<'a, ProfileCompositionMessage>;
    fn update(&mut self, message: ProfileCompositionMessage);
}

impl ProfileCompositionMappingExt for ProfileCompositionMapping {
    fn view<'a>(
        &'a self,
        sport_type_keys: &'a Vec<String>,
    ) -> iced::Element<'a, ProfileCompositionMessage> {
        column([
            row([
                column([
                    text("Subcomp name")
                        .style(|theme: &Theme| text::Style {
                            color: Some(theme.palette().text.scale_alpha(0.6)),
                        })
                        .into(),
                    text_input("Score Bug", &self.subcomp_name)
                        .width(Length::Fill)
                        .padding(8)
                        .on_input(ProfileCompositionMessage::SubcompNameUpdated)
                        .style(rounded_text_input_style)
                        .into(),
                ])
                .spacing(4)
                .into(),
                column([
                    text("Only apply if control node checked (blank to disable)")
                        .style(|theme: &Theme| text::Style {
                            color: Some(theme.palette().text.scale_alpha(0.6)),
                        })
                        .into(),
                    text_input(
                        "Boolean control node name",
                        &self.enabled_checkbox_name.as_deref().unwrap_or(""),
                    )
                    .width(Length::Fill)
                    .padding(8)
                    .on_input(ProfileCompositionMessage::CheckboxNameUpdated)
                    .style(rounded_text_input_style)
                    .into(),
                ])
                .spacing(4)
                .into(),
            ])
            .spacing(8)
            .into(),
            column(self.mapping.items.iter().enumerate().map(|(i, item)| {
                row([
                    Element::from(icon_button(
                        include_bytes!("../../assets/icon_delete.svg"),
                        "Remove mapping",
                        Some(()),
                        super::utils::RoundedButtonVariant::Danger,
                    ))
                    .map(move |_| ProfileCompositionMessage::ItemRemoved(i)),
                    Element::from(icon_button(
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
                        super::utils::RoundedButtonVariant::Secondary,
                    ))
                    .map(move |_| ProfileCompositionMessage::ItemEnabledUpdated(i, !item.enabled))
                    .into(),
                    pick_list(
                        sport_type_keys.as_ref(),
                        Some(&item.source_field),
                        move |new| ProfileCompositionMessage::ItemSourceFieldUpdated(i, new),
                    )
                    .width(Length::Fill)
                    .padding(8)
                    .style(rounded_pick_list_style)
                    .into(),
                    pick_list(Transformation::ALL, Some(item.transformation), move |new| {
                        ProfileCompositionMessage::ItemTransformationUpdated(i, new)
                    })
                    .width(Length::Fill)
                    .padding(8)
                    .style(rounded_pick_list_style)
                    .into(),
                    text_input("Destination field", &item.destination_field)
                        .width(Length::Fill)
                        .padding(8)
                        .on_input(move |new| {
                            ProfileCompositionMessage::ItemDestinationFieldUpdated(i, new)
                        })
                        .style(rounded_text_input_style)
                        .into(),
                ])
                .spacing(8)
                .align_y(iced::Alignment::Center)
                .into()
            }))
            .spacing(8)
            .into(),
            container(
                rounded_button("New mapping", super::utils::RoundedButtonVariant::Secondary)
                    .on_press(ProfileCompositionMessage::ItemAdded),
            )
            .center_x(Length::Fill)
            .into(),
        ])
        .spacing(8)
        .into()
    }

    fn update(&mut self, message: ProfileCompositionMessage) {
        match message {
            ProfileCompositionMessage::ItemAdded => {
                self.mapping.items.push(Default::default());
            }
            ProfileCompositionMessage::ItemRemoved(i) => {
                self.mapping.items.remove(i);
            }
            ProfileCompositionMessage::ItemEnabledUpdated(i, new) => {
                self.mapping.items[i].enabled = new;
            }
            ProfileCompositionMessage::ItemSourceFieldUpdated(i, new) => {
                self.mapping.items[i].source_field = new;
            }
            ProfileCompositionMessage::ItemTransformationUpdated(i, new) => {
                self.mapping.items[i].transformation = new;
            }
            ProfileCompositionMessage::ItemDestinationFieldUpdated(i, new) => {
                self.mapping.items[i].destination_field = new;
            }
            ProfileCompositionMessage::SubcompNameUpdated(new) => {
                self.subcomp_name = new;
            }
            ProfileCompositionMessage::CheckboxNameUpdated(new) => {
                self.enabled_checkbox_name = if new.is_empty() { None } else { Some(new) }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Update {
    None,
    RefreshSports(DynamicSportType),
}

pub trait ProfileConfigureExt {
    fn update(&mut self, message: ConfigureMessage) -> Update;
    fn view<'a>(&'a self, sport_type_keys: &'a Vec<String>) -> iced::Element<'a, ConfigureMessage>;
}

#[derive(Clone, Debug)]
pub enum ConfigureMessage {
    DataStreamUrlUpdated(String),
    SportTypeUpdated(DynamicSportType),
    MultipleRequestsUpdated(bool),
    ExcludeIncompleteDataUpdated(bool),
    MappingMessage(usize, ProfileCompositionMessage),
    AddSubcompMapping,
}

impl ProfileConfigureExt for Profile {
    fn update(&mut self, message: ConfigureMessage) -> Update {
        match message {
            ConfigureMessage::AddSubcompMapping => {
                self.mappings.push(Default::default());
                Update::None
            }
            ConfigureMessage::DataStreamUrlUpdated(new) => {
                self.data_stream_url = new;
                Update::None
            }
            ConfigureMessage::ExcludeIncompleteDataUpdated(new) => {
                self.exclude_incomplete_data = new;
                Update::None
            }
            ConfigureMessage::MappingMessage(i, msg) => {
                self.mappings[i].update(msg);
                Update::None
            }
            ConfigureMessage::MultipleRequestsUpdated(new) => {
                self.multiple_requests = new;
                Update::None
            }
            ConfigureMessage::SportTypeUpdated(new) => {
                self.sport_type = Some(new);
                Update::RefreshSports(new)
            }
        }
    }

    fn view<'a>(
        &'a self,
        sport_type_keys: &'a Vec<String>,
    ) -> Element<'a, ConfigureMessage, Theme, Renderer> {
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
                            &self.data_stream_url,
                        )
                        .width(Length::Fill)
                        .padding(8)
                        .on_input(ConfigureMessage::DataStreamUrlUpdated)
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
                            self.sport_type.as_ref(),
                            ConfigureMessage::SportTypeUpdated,
                        )
                        .width(Length::Fill)
                        .padding(8)
                        .style(rounded_pick_list_style)
                        .into(),
                    ])
                    .spacing(4)
                    .into(),
                ])
                .spacing(8)
                .into(),
                checkbox("Allow concurrent updates to the server", self.multiple_requests)
                    .on_toggle(ConfigureMessage::MultipleRequestsUpdated)
                    .into(),
                checkbox("Exclude incomplete data from payload instead of erroring", self.exclude_incomplete_data)
                    .on_toggle(ConfigureMessage::ExcludeIncompleteDataUpdated)
                    .into(),
                horizontal_rule(2.0).into(),
                column(
                    self.mappings
                        .iter()
                        .enumerate()
                        .map(|(i, mapping)|
                            container(mapping
                                .view(&sport_type_keys)
                                .map(move |msg|ConfigureMessage::MappingMessage(i, msg))
                            )
                            .style(|theme| container::Style {
                                background: Some(theme.extended_palette().background.weak.color.scale_alpha(0.4).into()),
                                border: border::rounded(12.0),
                                ..Default::default()
                            })
                            .padding(12.0)
                            .into()
                        )
                )
                .spacing(8)
                .into()
            ])
            .push_maybe(self.mappings.is_empty().then(|| {
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
                                    text("A mapping converts the map of data produced from the control console into a format intended for each subcomp in your Singular composition. Daktronics Singular UI provides transformations useful for parsing times and period names to help you convert the raw data into the right type of text. Press the button below to create a new one.")
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
                    rounded_button("New subcomp mapping", super::utils::RoundedButtonVariant::Secondary)
                    .on_press(ConfigureMessage::AddSubcompMapping)
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
}
