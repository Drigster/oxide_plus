use std::path::PathBuf;

use freya::prelude::*;

use crate::utils::image_utils::get_cached_image;

#[derive(Clone, PartialEq)]
pub struct CachedImage {
    image_uri: String,
    width: Size,
    height: Size,
}

impl CachedImage {
    pub fn new(uri: String) -> Self {
        Self {
            image_uri: uri,
            width: Size::default(),
            height: Size::default(),
        }
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }
}

impl Render for CachedImage {
    fn render(&self) -> impl IntoElement {
        let mut path = use_state(|| None::<PathBuf>);

        let image_uri = self.image_uri.clone();

        use_hook(|| {
            spawn(async move {
                match get_cached_image(image_uri.clone()).await {
                    Ok(image_path) => {
                        path.set(Some(image_path));
                    }
                    Err(e) => {
                        println!("Image failed to load: {} - Error: {}", image_uri, e);
                    }
                }
            });
        });

        if let Some(image_path) = path.read().clone() {
            ImageViewer::new(image_path)
                .width(self.width.clone())
                .height(self.height.clone())
                .into_element()
        } else {
            rect()
                .width(self.width.clone())
                .height(self.height.clone())
                .background(Color::TRANSPARENT)
                .center()
                .into()
        }
    }
}
