use std::{cmp::max, time::Duration};

use iced::{
    mouse,
    widget::{
        canvas::{self, Cache, Canvas, Path, Text},
        component, Component,
    },
    Element, Font, Length, Point, Renderer, Size, Theme,
};

use crate::backend::stream::ActiveStream;

pub struct Graph<'a> {
    active_stream: &'a ActiveStream,
}

impl<'a> Graph<'a> {
    pub fn new(active_stream: &'a ActiveStream) -> Graph<'a> {
        Graph { active_stream }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum GraphEvent {}

#[derive(Debug, Default)]
pub struct GraphProgramState {
    grid_labels: Cache,
    graph: Cache,
}

pub fn graph<'a>(active_stream: &'a ActiveStream) -> Graph<'a> {
    Graph::new(&active_stream)
}

impl<'a, Message: Clone> Component<Message> for Graph<'a> {
    type State = ();

    type Event = GraphEvent;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Theme, Renderer> {
        Canvas::new(self as &Self)
            .width(Length::Fill)
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

impl<Message> canvas::Program<Message> for Graph<'_> {
    type State = GraphProgramState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        const MINOR_GRID_LINE_THICKNESS: f32 = 1.0;
        const MAJOR_GRID_LINE_THICKNESS: f32 = 2.0;
        const TEXT_LABEL_SIZE: f32 = 12.0;
        const TEXT_LABEL_HEIGHT: f32 = 16.0;
        const DATA_POINT_RADIUS: f32 = 2.0;
        const DATA_POINT_LINE_THICKNESS: f32 = 1.0;
        const LATENCY_LABEL_WIDTH: f32 = 36.0;
        const TEXT_PADDING: f32 = 4.0;
        const LABELS: &[Duration] = &[
            Duration::from_secs(60 * 5),
            Duration::from_secs(60 * 4),
            Duration::from_secs(60 * 3),
            Duration::from_secs(60 * 2),
            Duration::from_secs(60 * 1),
            Duration::from_secs(60 * 0),
        ];

        let visible_duration = Duration::from_secs(60 * 5);
        let time_to_x = |time: &Duration| {
            (1.0 - (time.as_millis() as f32 / visible_duration.as_millis() as f32))
                * (bounds.width - LATENCY_LABEL_WIDTH)
        };

        let time_ago_formatter = timeago::Formatter::new();
        let labels_text = LABELS
            .iter()
            .map(|label| {
                time_ago_formatter
                    .convert(label.clone())
                    .replace(" ago", "")
                    .replace("minute", "min")
                    .replace("second", "sec")
            })
            .collect::<Vec<_>>();

        let grid_labels = state.grid_labels.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();
            frame.fill_rectangle(
                Point {
                    x: time_to_x(&visible_duration),
                    y: frame.height() - MAJOR_GRID_LINE_THICKNESS - TEXT_LABEL_HEIGHT,
                },
                Size {
                    width: time_to_x(&Duration::ZERO) - time_to_x(&visible_duration),
                    height: MAJOR_GRID_LINE_THICKNESS,
                },
                palette.background.base.text,
            );

            for (label, label_text) in LABELS.iter().zip(labels_text) {
                let x = time_to_x(label);
                frame.fill_text(Text {
                    color: palette.background.base.text,
                    content: label_text,
                    font: Font::DEFAULT,
                    horizontal_alignment: if x < time_to_x(&visible_duration) + TEXT_LABEL_SIZE {
                        iced::alignment::Horizontal::Left
                    } else if x > frame.width() - TEXT_LABEL_SIZE {
                        iced::alignment::Horizontal::Right
                    } else {
                        iced::alignment::Horizontal::Center
                    },
                    position: Point {
                        x,
                        y: frame.height() - TEXT_LABEL_HEIGHT + TEXT_PADDING,
                    },
                    size: TEXT_LABEL_SIZE.into(),
                    vertical_alignment: iced::alignment::Vertical::Top,
                    ..Default::default()
                });
                frame.fill_rectangle(
                    Point {
                        x: x - MINOR_GRID_LINE_THICKNESS / 2.0,
                        y: 0.0,
                    },
                    Size {
                        width: MINOR_GRID_LINE_THICKNESS,
                        height: frame.height() - TEXT_LABEL_HEIGHT - MAJOR_GRID_LINE_THICKNESS,
                    },
                    palette.secondary.weak.color,
                )
            }

            frame.fill_rectangle(
                Point {
                    x: frame.width() - LATENCY_LABEL_WIDTH,
                    y: 0.0,
                },
                Size {
                    width: MAJOR_GRID_LINE_THICKNESS,
                    height: frame.height() - TEXT_LABEL_HEIGHT,
                },
                palette.background.base.text,
            );
        });

        state.graph.clear();
        let graph = state.graph.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let data = self.active_stream.latency_graph_data();
            let max_latency = data
                .samples
                .iter()
                .map(|x| x.latency)
                .reduce(|x, y| max(x, y))
                .unwrap_or(Duration::from_millis(1));
            let frame_height = frame.height();
            let latency_to_y = |latency: &Duration| {
                0.0 + (1.0 - (latency.as_millis() as f32 / max_latency.as_millis() as f32))
                    * (frame_height - TEXT_LABEL_HEIGHT - MAJOR_GRID_LINE_THICKNESS)
            };

            let label_count = (max_latency.as_millis() / 100) as usize;
            for i in 1..=label_count {
                let label = i * 100;
                let y = latency_to_y(&Duration::from_millis(label as u64));
                frame.fill_text(Text {
                    color: palette.background.base.text,
                    content: label.to_string(),
                    font: Font::DEFAULT,
                    vertical_alignment: if y < 0.0 + TEXT_LABEL_SIZE {
                        iced::alignment::Vertical::Top
                    } else if y > frame.height() - TEXT_LABEL_SIZE {
                        iced::alignment::Vertical::Bottom
                    } else {
                        iced::alignment::Vertical::Center
                    },
                    position: Point {
                        x: frame.width() - LATENCY_LABEL_WIDTH
                            + MAJOR_GRID_LINE_THICKNESS
                            + TEXT_PADDING,
                        y,
                    },
                    size: TEXT_LABEL_SIZE.into(),
                    horizontal_alignment: iced::alignment::Horizontal::Left,
                    ..Default::default()
                });
                frame.fill_rectangle(
                    Point {
                        x: 0.0,
                        y: y - MINOR_GRID_LINE_THICKNESS / 2.0,
                    },
                    Size {
                        width: frame.width() - LATENCY_LABEL_WIDTH - MAJOR_GRID_LINE_THICKNESS,
                        height: MAJOR_GRID_LINE_THICKNESS,
                    },
                    palette.secondary.weak.color,
                )
            }

            for serial_event in &data.serial_events {
                frame.fill(
                    &Path::circle(
                        Point {
                            x: time_to_x(&serial_event.timestamp.elapsed()),
                            y: frame.height()
                                - MAJOR_GRID_LINE_THICKNESS
                                - TEXT_LABEL_HEIGHT
                                - DATA_POINT_RADIUS / 2.0,
                        },
                        DATA_POINT_RADIUS,
                    ),
                    palette.primary.weak.color,
                );
            }

            let mut last_sample_point = None;
            for latency_sample in &data.samples {
                let point = Point {
                    x: time_to_x(&latency_sample.timestamp.elapsed()),
                    y: latency_to_y(&latency_sample.latency),
                };
                frame.fill(
                    &Path::circle(point, DATA_POINT_RADIUS),
                    palette.primary.base.color,
                );
                if let Some(last_sample_point) = last_sample_point {
                    frame.stroke(
                        &Path::line(last_sample_point, point),
                        canvas::stroke::Stroke {
                            style: canvas::Style::Solid(palette.primary.weak.color),
                            width: DATA_POINT_LINE_THICKNESS,
                            ..Default::default()
                        },
                    );
                }
                last_sample_point = Some(point);
            }
        });

        vec![grid_labels, graph]
    }
}

impl<'a, Message: Clone> From<Graph<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: Graph<'a>) -> Self {
        component(header)
    }
}
