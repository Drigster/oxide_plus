use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
    pub avatar_url: String,
}

impl TeamMember {
    pub fn new(name: String, role: String, avatar_url: String) -> Self {
        Self {
            name,
            role,
            avatar_url,
        }
    }
}

impl Component for TeamMember {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .padding(8.0)
            .background(Color::from_hex("#000000").unwrap())
            .direction(Direction::Horizontal)
            .children([
                rect()
                    .width(Size::px(300.0))
                    .height(Size::Fill)
                    .children([rect()
                        .width(Size::Fill)
                        .height(Size::px(48.0))
                        .margin(8.0)
                        .border(
                            Border::new()
                                .width(1.0)
                                .fill(Color::from_hex("#FFFFFF").unwrap()),
                        )
                        .into()])
                    .into(),
                rect().width(Size::Fill).height(Size::Fill).into(),
            ])
    }
}

#[derive(PartialEq)]
pub struct Team {}
impl Component for Team {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .corner_radius(8.0)
            .padding(8.0)
            .background(Color::from_hex("#000000").unwrap())
            .direction(Direction::Horizontal)
            .children([
                rect()
                    .width(Size::px(300.0))
                    .height(Size::Fill)
                    .children([TeamMember::new(
                        "Drigster".to_string(),
                        "Developer".to_string(),
                        "".to_string(),
                    )
                    .into()])
                    .into(),
                rect().width(Size::Fill).height(Size::Fill).into(),
            ])
    }
}
