use freya::{prelude::*, router::prelude::Outlet};

use crate::app::Route;

#[derive(PartialEq)]
pub struct RouteChangeRecieverLayout;
impl Component for RouteChangeRecieverLayout {
    fn render(&self) -> impl IntoElement {
        use_future(move || async move {});

        Outlet::<Route>::new()
    }
}
