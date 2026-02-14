use freya::{prelude::*, radio::use_radio, router::Outlet};

use crate::{DataChannel, app::Route};

#[derive(PartialEq)]
pub struct RootLayout;
impl Component for RootLayout {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio(DataChannel::ToastsUpdate);
        let toasts = radio.slice_current(|s| &s.toasts);

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .child(
                rect()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .layer(Layer::RelativeOverlay(1))
                    .margin(Gaps::new(175.0, 0.0, 0.0, 0.0))
                    .position(Position::new_absolute())
                    .cross_align(Alignment::End)
                    .spacing(8.0)
                    .children(
                        toasts
                            .read()
                            .values()
                            .into_iter()
                            .map(|toast| toast.clone().into())
                            .collect::<Vec<Element>>(),
                    ),
            )
            .child(Outlet::<Route>::new())
    }
}
