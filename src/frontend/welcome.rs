use super::utils::{self, rounded_button};
use iced::{
    widget::{column, container, row, svg, text},
    Alignment, Length,
};

pub fn view<'a, Message: Clone + 'a>(
    import_profile_message: Message,
    new_profile_message: Message,
    open_github_message: Message,
) -> iced::widget::Stack<'a, Message> {
    iced::widget::stack([
        container(
            svg(svg::Handle::from_memory(include_bytes!(
                "../../assets/splash.svg"
            )))
            .width(700)
            .height(350)
            .content_fit(iced::ContentFit::Cover)
            .opacity(0.8),
        )
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Bottom)
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
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
                    .on_press(import_profile_message)
                    .into(),
                    rounded_button(
                        text("New profile").size(18),
                        utils::RoundedButtonVariant::Primary,
                    )
                    .on_press(new_profile_message)
                    .into(),
                    rounded_button(
                        text("Open GitHub source").size(18),
                        utils::RoundedButtonVariant::Secondary,
                    )
                    .on_press(open_github_message)
                    .into(),
                ])
                .spacing(8)
                .into(),
            ])
            .spacing(16)
            .align_x(Alignment::Start),
        )
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
    ])
}
