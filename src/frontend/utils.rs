use iced::{
    border::Radius,
    widget::{button, container, pick_list, svg, text_input, tooltip, Button, Container, Tooltip},
    Border, Element, Renderer, Theme,
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
                .style(move |theme: &Theme, _| svg::Style {
                    color: Some(match variant {
                        RoundedButtonVariant::Danger => theme.extended_palette().danger.base.text,
                        RoundedButtonVariant::Secondary => {
                            theme.extended_palette().secondary.base.text
                        }
                        RoundedButtonVariant::Primary => theme.extended_palette().primary.base.text,
                    }),
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

pub fn rounded_pick_list_style(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.extended_palette();
    let active = pick_list::Style {
        background: palette.background.base.color.into(),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 1.0,
            color: palette.background.strong.color,
        },
        handle_color: palette.background.weak.text,
        placeholder_color: palette.background.strong.color,
        text_color: palette.background.base.text,
    };
    match status {
        pick_list::Status::Active => active,
        pick_list::Status::Hovered => pick_list::Style {
            border: Border {
                color: palette.background.base.text,
                ..active.border
            },
            ..active
        },
        pick_list::Status::Opened => pick_list::Style {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
    }
}

pub fn rounded_text_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let mut style = text_input::default(theme, status);
    style.border.radius = BORDER_RADIUS.into();
    style
}
