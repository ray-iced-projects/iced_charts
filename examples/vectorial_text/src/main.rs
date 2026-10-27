use iced::Pixels;
use iced::alignment;
use iced::mouse;
use iced::widget::canvas::Path;
use iced::widget::canvas::Stroke;
use iced::widget::scrollable;
use iced::widget::{canvas, checkbox, column, row, slider, space, text};
use iced::{Center, Fill, Point, Rectangle, Renderer, Theme, Vector, Widget};

pub fn main() -> iced::Result {

    iced::application(
        VectorialText::default,
        VectorialText::update,
        VectorialText::view,
    )
    .theme(Theme::Dark)
    .centered()
    .run()
}

#[derive(Default)]
struct VectorialText {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SizeChanged(f32),
    AngleChanged(f32),
    ScaleChanged(f32),
    ToggleJapanese(bool),
}

impl VectorialText {
    fn update(&mut self, message: Message) {
        match message {
            Message::SizeChanged(size) => {
                self.state.size = size;
            }
            Message::AngleChanged(angle) => {
                self.state.angle = angle;
            }
            Message::ScaleChanged(scale) => {
                self.state.scale = scale;
            }
            Message::ToggleJapanese(use_japanese) => {
                self.state.use_japanese = use_japanese;
            }
        }

        self.state.cache.clear();
    }

    fn view(&self) -> impl Widget<Message> {
        scrollable(
        canvas(&self.state).width(900.0).height(900.0).boxed()
        ).boxed()
    }
}

struct State {
    size: f32,
    angle: f32,
    scale: f32,
    use_japanese: bool,
    cache: canvas::Cache,
}

impl State {
    pub fn new() -> Self {
        Self {
            size: 40.0,
            angle: 0.0,
            scale: 1.0,
            use_japanese: false,
            cache: canvas::Cache::new(),
        }
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _tree: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let palette = theme.seed();

            let labels = ["Performance", "Reliability", "Security", "Usability", "Cost", "Support"];
            let points = [
                Point::new(362.0, 69.0),
                Point::new(602.2355, 207.70001),
                Point::new(602.2355, 485.1),
                Point::new(361.99997, 623.8),
                Point::new(121.76454, 485.09998),
                Point::new(121.76454, 207.70003)
            ];
            for point in points {
                let path = Path::circle(Point::new(point.x, point.y), 2.0);
                let color = iced::Color::WHITE;
                frame.fill(&path, color);
                
                frame.stroke(
                    &path,
                    Stroke::default().with_color(color)
                )
            }

            for (i, point) in points.iter().enumerate() {
                let mut align_x = text::Alignment::Left;
                let mut align_y = alignment::Vertical::Center;
                
                if ["Cost", "Support"].contains(&labels[i]) {
                    align_x = text::Alignment::Right;
                }
                frame.with_save(|f| {
                    f.translate(Vector::new(point.x, point.y));
                    f.fill_text(canvas::Text {
                        position: Point::new(0.0, 0.0),
                        color: palette.text,
                        size: Pixels(12.0),
                        content: labels[i].to_string(),
                        align_x,
                        align_y,
                        ..canvas::Text::default()
                    });
                })
            }
        });

        vec![geometry]
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}
