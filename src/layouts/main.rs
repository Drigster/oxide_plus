use freya::{prelude::*, router::Outlet};

use crate::{
    app::Route,
    colors,
    components::{Navbar, Sidebar},
};

#[derive(PartialEq)]
pub struct MainLayout;
impl Component for MainLayout {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::percent(100.0))
            .height(Size::percent(100.0))
            .background(Color::from_hex("#1e1e1e").unwrap())
            .children([
                Navbar::new().into(),
                rect()
                    .width(Size::percent(100.0))
                    .height(Size::Fill)
                    .background(Color::from_hex(colors::BACKGROUND_DARK).unwrap())
                    .direction(Direction::Horizontal)
                    .children([
                        Sidebar::new().into(),
                        rect()
                            .height(Size::percent(100.0))
                            .width(Size::Fill)
                            .child(Outlet::<Route>::new())
                            .into(),
                    ])
                    .into(),
            ])
    }
}
