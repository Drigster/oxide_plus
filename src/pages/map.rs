use freya::prelude::*;

use crate::components::Map as MapComponent;

#[derive(PartialEq)]
pub struct Map {}
impl Render for Map {
    fn render(&self) -> Element {
        rect()
            .padding(8.0)
            .child(
                MapComponent::new()
                    .grid(true)
                    .teammates(true)
                    .deaths(true)
                    .monuments(true)
                    .shops(true),
            )
            .into()
    }
}

fn get_text_size_concise(scale: f32) -> f32 {
    return 8.864 / scale + 2.446;
}
