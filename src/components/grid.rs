use std::cmp;

use freya::prelude::*;

use crate::utils::number_to_letters;

#[derive(Clone, PartialEq)]
pub struct Grid {
    map_size: f32,
    margin: f32,
    zoom: Option<Readable<f32>>,
}

impl Grid {
    pub fn new(map_size: f32, margin: f32) -> Self {
        Self {
            map_size,
            margin,
            zoom: None,
        }
    }

    pub fn zoom(mut self, zoom: impl Into<Readable<f32>>) -> Self {
        self.zoom = Some(zoom.into());
        self
    }
}

const GRID_CELL_SIZE: f32 = 146.25;

impl Component for Grid {
    fn render(&self) -> impl IntoElement {
        let text_margin = 4.0;

        let active_area_size = self.map_size as f32 - (self.margin * 2.0);

        let cells = cmp::max(1, ((active_area_size / GRID_CELL_SIZE).floor()) as u32);

        let converted_grid_size = active_area_size / cells as f32;

        let zoom = use_hook(|| {
            self.zoom
                .clone()
                .unwrap_or_else(|| Readable::from_value(1.0))
        });

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .children(
                (0..=cells)
                    .map(|i| {
                        let x = self.margin + i as f32 * converted_grid_size - 1.0;
                        rect()
                            .position(Position::new_absolute().left(x).top(self.margin))
                            .width(Size::px(1.5))
                            .height(Size::px(active_area_size))
                            .background(Color::from_hex("#19191980").unwrap())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
            .children(
                (0..=cells)
                    .map(|i| {
                        let y = self.margin + i as f32 * converted_grid_size - 1.0;
                        rect()
                            .position(Position::new_absolute().left(self.margin).top(y))
                            .height(Size::px(1.5))
                            .width(Size::px(active_area_size))
                            .background(Color::from_hex("#19191980").unwrap())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
            .children(
                (0..=cells - 1)
                    .flat_map(|i| {
                        (0..=cells - 1).map({
                            let zoom = zoom.clone();
                            move |j| {
                                label()
                                    // Magic numbers :)
                                    .font_size(-4.44 * *zoom.read() + 19.11)
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
                            }
                        })
                    })
                    .collect::<Vec<Element>>(),
            )
    }
}
