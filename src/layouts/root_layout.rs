use freya::{prelude::*, radio::use_radio, router::Outlet};

use crate::{DataChannel, app::Route};

#[derive(PartialEq)]
pub struct RootLayout;
impl Component for RootLayout {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio(DataChannel::ToastsUpdate);
        let toasts = radio.slice_current(|s| &s.toasts);

        let binding = radio.slice(DataChannel::ModalUpdate, |s| &s.modal);
        let modal = binding.read().clone();

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
            .maybe_child(if modal.is_some() {
                Some(
                    rect()
                        .width(Size::Fill)
                        .height(Size::Fill)
                        .layer(Layer::RelativeOverlay(2))
                        .position(Position::new_absolute())
                        .main_align(Alignment::Center)
                        .cross_align(Alignment::Center)
                        .background(Color::from_hex("#00000080").unwrap())
                        .child(modal.unwrap()),
                )
            } else {
                None
            })
            .child(Outlet::<Route>::new())
    }
}
