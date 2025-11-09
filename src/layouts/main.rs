use freya::prelude::*;
use freya_router::prelude::outlet;

use crate::{
    app::Route,
    components::{Navbar, Sidebar},
};

#[derive(PartialEq)]
pub struct MainLayout;
impl Render for MainLayout {
    fn render(&self) -> Element {
        rect()
            .width(Size::percent(100.0))
            .height(Size::percent(100.0))
            .background(Color::from_hex("#1e1e1e").unwrap())
            .children([
                Navbar::new().into(),
                rect()
                    .width(Size::percent(100.0))
                    .height(Size::Fill)
                    .background_linear_gradient(
                        LinearGradient::new()
                            .angle(0.0)
                            .stop((Color::from_hex("#1D1D1B").unwrap(), 0.0))
                            .stop((Color::from_hex("#0E0E0D").unwrap(), 100.0)),
                    )
                    .direction(Direction::Horizontal)
                    .children([
                        Sidebar::new().into(),
                        rect()
                            .height(Size::percent(100.0))
                            .width(Size::Fill)
                            .padding(8.0)
                            .child(outlet::<Route>())
                            .into(),
                    ])
                    .into(),
            ])
            .into()
    }
}
