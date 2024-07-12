use iced::{
    border::Radius,
    widget::{button, svg, Button},
    Renderer, Theme,
};

pub const BORDER_RADIUS: f32 = 8.0;

pub fn icon_button<Message>(bytes: &'static [u8]) -> Button<'static, Message, Theme, Renderer> {
    button(
        svg(svg::Handle::from_memory(bytes))
            .style(|theme: &Theme, _| svg::Style {
                color: Some(theme.palette().text),
            })
            .content_fit(iced::ContentFit::Fill),
    )
    .style(|theme, status| {
        let mut style = button::secondary(theme, status);
        style.border.radius = Radius::from(BORDER_RADIUS);
        style
    })
    .padding(8)
    .width(36)
    .height(36)
}
