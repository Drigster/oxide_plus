use std::path::PathBuf;

use freya::prelude::*;

use crate::utils::get_cached_image;

#[derive(Clone, PartialEq)]
pub struct CachedImage {
    image_uri: String,
    width: Size,
    height: Size,
    aspect_ratio: AspectRatio,
}

#[allow(dead_code)]
impl CachedImage {
    pub fn new(uri: String) -> Self {
        Self {
            image_uri: uri,
            width: Size::default(),
            height: Size::default(),
            aspect_ratio: AspectRatio::default(),
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

    pub fn aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }
}

impl Component for CachedImage {
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
                        eprintln!("Image failed to load: {} - Error: {}", image_uri, e);
                    }
                }
            });
        });

        if let Some(image_path) = path.read().clone() {
            ImageViewer::new(image_path)
                .width(self.width.clone())
                .height(self.height.clone())
                .aspect_ratio(self.aspect_ratio.clone())
                .into_element()
        } else {
            rect()
                .width(self.width.clone())
                .height(self.height.clone())
                .background(Color::TRANSPARENT)
                .center()
                .into_element()
        }
    }
}
