use std::cmp;

use freya::prelude::*;

use crate::utils::text_utils::number_to_letters;

#[derive(Clone, PartialEq)]
pub struct Grid {
    image_width: u32,
    image_height: u32,
    map_size: f32,
    margin: f32,
    zoom: f32,
}

impl Grid {
    pub fn new(image_width: u32, image_height: u32, map_size: f32, margin: f32) -> Self {
        Self {
            image_width,
            image_height,
            map_size,
            margin,
            zoom: 1.0,
        }
    }

    pub fn on_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }
}

impl Render for Grid {
    fn render(&self) -> impl IntoElement {
        let grid_size = 146.25;
        let text_margin = 4.0;

        let cells = cmp::max(1, ((self.map_size / grid_size).round() - 1.0) as u32);

        let active_area_size = self.image_width as f32 - (self.margin * 2.0);

        let converted_grid_size = active_area_size / cells as f32;

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .children(
                (0..=cells)
                    .map(|i| {
                        let x = self.margin + i as f32 * converted_grid_size;
                        rect()
                            .position(Position::new_absolute().left(x).top(self.margin))
                            .width(Size::px(1.0))
                            .height(Size::px(active_area_size))
                            .background(Color::from_hex("#19191980").unwrap())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
            .children(
                (0..=cells)
                    .map(|i| {
                        let y = self.margin + i as f32 * converted_grid_size;
                        rect()
                            .position(Position::new_absolute().left(self.margin).top(y))
                            .height(Size::px(1.0))
                            .width(Size::px(active_area_size))
                            .background(Color::from_hex("#19191980").unwrap())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
            .children(
                (0..=cells - 1)
                    .flat_map(|i| {
                        (0..=cells - 1).map(move |j| {
                            label()
                                // Magic numbers :)
                                .font_size(-4.44 * self.zoom + 19.11)
                                .font_weight(FontWeight::BOLD)
                                .color(Color::from_hex("#19191980").unwrap())
                                .text(format!("{}{}", number_to_letters(i), j))
                                .position(
                                    Position::new_absolute()
                                        .left(
                                            (i as f32 * converted_grid_size)
                                                + self.margin
                                                + text_margin,
                                        )
                                        .top(
                                            (j as f32 * converted_grid_size)
                                                + self.margin
                                                + text_margin,
                                        ),
                                )
                                .into()
                        })
                    })
                    .collect::<Vec<Element>>(),
            )
    }
}
