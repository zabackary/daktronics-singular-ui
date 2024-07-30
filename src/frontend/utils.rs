use iced::{
    border::Radius,
    widget::{button, container, svg, tooltip, Button, Container, Tooltip},
    Element, Renderer, Theme,
};

pub const BORDER_RADIUS: f32 = 8.0;

#[derive(Debug, Clone, Copy)]
pub enum RoundedButtonVariant {
    Danger,
    Secondary,
    Primary,
}

pub fn icon_button<'a, Message: Clone + 'a>(
    bytes: &'static [u8],
    label: &'a str,
    on_press_maybe: Option<Message>,
    variant: RoundedButtonVariant,
) -> Tooltip<'a, Message, Theme, Renderer> {
    tooltip(
        button(
            svg(svg::Handle::from_memory(bytes))
                .style(|theme: &Theme, _| svg::Style {
                    color: Some(theme.palette().text),
                })
                .content_fit(iced::ContentFit::Fill),
        )
        .style(move |theme, status| {
            let mut style = match variant {
                RoundedButtonVariant::Danger => button::danger(theme, status),
                RoundedButtonVariant::Secondary => button::secondary(theme, status),
                RoundedButtonVariant::Primary => button::primary(theme, status),
            };
            style.border.radius = Radius::from(BORDER_RADIUS);
            style
        })
        .on_press_maybe(on_press_maybe)
        .padding(8)
        .width(36)
        .height(36),
        container(label).style(container::rounded_box).padding(4),
        tooltip::Position::Bottom,
    )
}

pub fn rounded_button<'a, Message>(
    label: impl Into<Element<'a, Message>>,
    variant: RoundedButtonVariant,
) -> Button<'a, Message, Theme, Renderer> {
    button(label.into()).style(move |theme, status| {
        let mut style = match variant {
            RoundedButtonVariant::Danger => button::danger(theme, status),
            RoundedButtonVariant::Secondary => button::secondary(theme, status),
            RoundedButtonVariant::Primary => button::primary(theme, status),
        };
        style.border.radius = 999.into();
        style
    })
}

pub fn rounded_pane<'a, Message>(child: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    container(child).style(|theme| {
        let mut style = container::rounded_box(theme);
        style.border.radius = BORDER_RADIUS.into();
        style
    })
}
