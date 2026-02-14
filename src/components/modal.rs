use freya::{prelude::*, radio::use_radio};

use crate::{
    DataChannel, colors,
    components::{Button, CachedImage},
    utils::{ServerData, save_server},
};

#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum ModalType {
    ServerPair(ServerData),
}

#[derive(PartialEq, Clone)]
pub struct Modal {
    pub modal_type: ModalType,
}

impl Modal {
    pub fn new(overlay_type: ModalType) -> Self {
        Self {
            modal_type: overlay_type,
        }
    }
}

impl Component for Modal {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(DataChannel::ModalUpdate);
        let mut modal = radio.slice_mut_current(|s| &mut s.modal);

        match &self.modal_type {
            ModalType::ServerPair(server_pair_settings) => {
                rect()
                    .width(Size::px(600.0))
                    .padding(8.0)
                    .spacing(8.0)
                    .corner_radius(8.0)
                    .background(Color::from_hex(colors::BACKGROUND_DARK).unwrap())
                    .maybe_child(if !server_pair_settings.logo.trim().is_empty() {
                        Some(
                            rect()
                                .width(Size::px(96.0))
                                .height(Size::px(96.0))
                                .padding(8.0)
                                .corner_radius(9999.0)
                                .position(Position::new_absolute().top(246.0).right(0.0))
                                .background(Color::from_hex(colors::BACKGROUND_DARK).unwrap())
                                .main_align(Alignment::Center)
                                .cross_align(Alignment::Center)
                                .layer(Layer::Relative(5))
                                .child(
                                    rect()
                                        .width(Size::Fill)
                                        .height(Size::Fill)
                                        .corner_radius(9999.0)
                                        .overflow(Overflow::Clip)
                                        .child(
                                            CachedImage::new(server_pair_settings.logo.clone())
                                                .width(Size::Fill)
                                                .height(Size::Fill)
                                                .aspect_ratio(AspectRatio::Max),
                                        ),
                                ),
                        )
                    } else {
                        None
                    })
                    .children([
                        rect()
                            .width(Size::Fill)
                            .height(Size::px(290.0))
                            .corner_radius(8.0)
                            .main_align(Alignment::Center)
                            .cross_align(Alignment::Center)
                            .background(Color::from_hex(colors::BACKGROUND).unwrap())
                            .overflow(Overflow::Clip)
                            .maybe_child(if !server_pair_settings.img.is_empty() {
                                Some(
                                    CachedImage::new(server_pair_settings.img.clone())
                                        .width(Size::Fill)
                                        .height(Size::Fill)
                                        .aspect_ratio(AspectRatio::Max),
                                )
                            } else {
                                None
                            })
                            .into(),
                        rect()
                            .width(Size::Fill)
                            .padding(8.0)
                            .corner_radius(8.0)
                            .background(Color::from_hex(colors::BACKGROUND).unwrap())
                            .direction(Direction::Horizontal)
                            .main_align(Alignment::SpaceBetween)
                            .cross_align(Alignment::Center)
                            .child(
                                label()
                                    .font_size(24.0)
                                    .color(Color::from_hex(colors::TEXT).unwrap())
                                    .text(server_pair_settings.name.clone()),
                            )
                            .into(),
                        rect()
                            .width(Size::Fill)
                            .height(Size::px(250.0))
                            .corner_radius(8.0)
                            .background(Color::from_hex(colors::BACKGROUND).unwrap())
                            .direction(Direction::Horizontal)
                            .main_align(Alignment::SpaceBetween)
                            .cross_align(Alignment::Center)
                            .child(
                                ScrollView::new()
                                    .width(Size::Fill)
                                    .height(Size::Fill)
                                    .child(
                                        rect().padding(8.0).child(
                                            label()
                                                .font_size(16.0)
                                                .color(Color::from_hex(colors::TEXT).unwrap())
                                                .text(server_pair_settings.desc.clone()),
                                        ),
                                    ),
                            )
                            .into(),
                        rect()
                            .width(Size::Fill)
                            .padding(8.0)
                            .corner_radius(8.0)
                            .background(Color::from_hex(colors::BACKGROUND).unwrap())
                            .direction(Direction::Horizontal)
                            .main_align(Alignment::SpaceBetween)
                            .cross_align(Alignment::Center)
                            .children([
                                Button::new()
                                    .height(Size::px(36.0))
                                    .padding(8.0)
                                    .corner_radius(8.0)
                                    .background(Color::from_hex(colors::BACKGROUND_DARK).unwrap())
                                    .text("Cancel")
                                    .on_press(move |_| {
                                        *modal.write() = None;
                                    })
                                    .into(),
                                Button::new()
                                    .height(Size::px(36.0))
                                    .padding(8.0)
                                    .corner_radius(8.0)
                                    .background(Color::from_hex(colors::SELECT).unwrap())
                                    .icon_color(Color::from_hex(colors::TEXT).unwrap())
                                    // .icon(Bytes::from_static(include_bytes!(
                                    //     "../assets/MDI/plus-circle.svg"
                                    // )))
                                    .text("Pair server")
                                    .on_press({
                                        let server_pair_settings = server_pair_settings.clone();
                                        move |_| {
                                            *modal.write() = None;
                                            radio
                                                .write_channel(DataChannel::ServersUpdate)
                                                .servers
                                                .insert(
                                                    server_pair_settings.id.clone(),
                                                    server_pair_settings.clone(),
                                                );
                                            radio
                                                .write_channel(DataChannel::SelectedServerUpdate)
                                                .selected_server =
                                                Some(server_pair_settings.clone());
                                            match save_server(server_pair_settings.clone()) {
                                                Ok(_) => {
                                                    println!("Server data saved successfully.");
                                                }
                                                Err(err) => {
                                                    println!("Error saving server data: {}", err);
                                                }
                                            }
                                        }
                                    })
                                    .into(),
                            ])
                            .into(),
                    ])
            }
        }
    }
}
