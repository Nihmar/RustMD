use iced::widget::{PaneGrid, column, container, pane_grid, text};
use iced::{Color, Element, Length, Size, window};

#[derive(Debug, Clone)]
enum Message {
    PaneResized(pane_grid::ResizeEvent),
}

struct State {
    panes: pane_grid::State<PaneType>,
}

enum PaneType {
    Left,
    Center,
    Right,
}

impl Default for State {
    fn default() -> Self {
        let (mut panes, left) = pane_grid::State::new(PaneType::Left);

        let (center, split1) = panes
            .split(pane_grid::Axis::Vertical, left, PaneType::Center)
            .unwrap();

        let (_right, _split2) = panes
            .split(pane_grid::Axis::Vertical, center, PaneType::Right)
            .unwrap();

        // Set initial widths as ratios
        panes.resize(split1, 0.2); // left takes 20%
        // right will be whatever remains after center

        Self { panes }
    }
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::PaneResized(event) => {
            state.panes.resize(event.split, event.ratio);
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    PaneGrid::new(&state.panes, |_pane, content, _| {
        let body: Element<_> = match content {
            PaneType::Left => container(
                column![
                    text("Files").style(|_| text::Style {
                        color: Some(Color::from_rgb8(200, 200, 200)),
                    }),
                    // file tree items...
                ]
                .spacing(4)
                .padding(12),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color::from_rgb8(25, 25, 25).into()),
                ..Default::default()
            })
            .into(),

            PaneType::Center => container(
                column![
                    text("Main Panel").size(28).style(|_| text::Style {
                        color: Some(Color::from_rgb8(220, 220, 220)),
                    }),
                    // editor content...
                ]
                .spacing(8)
                .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color::from_rgb8(30, 30, 30).into()),
                ..Default::default()
            })
            .into(),

            PaneType::Right => container(
                column![text("Outline").style(|_| text::Style {
                    color: Some(Color::from_rgb8(200, 200, 200)),
                }),]
                .spacing(4)
                .padding(12),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color::from_rgb8(25, 25, 25).into()),
                ..Default::default()
            })
            .into(),
        };

        pane_grid::Content::new(body)
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .on_resize(6, Message::PaneResized)
    .into()
}

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("RustMD")
        .style(|_state, _theme| iced::theme::Style {
            background_color: Color::from_rgb8(25, 25, 25),
            text_color: Color::from_rgb8(200, 200, 200),
        })
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(400.0, 300.0)),
            resizable: true,
            ..Default::default()
        })
        .run()
}
