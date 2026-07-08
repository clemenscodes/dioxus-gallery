use crate::percent::Percent;
use crate::view::{DEFAULT_VIEWPORT_HEIGHT, DEFAULT_VIEWPORT_WIDTH, GalleryView};

/// The parsed launch intent read from the page query string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GalleryMode {
    Shell { view: GalleryView },
    Frame { story: String },
}

impl GalleryMode {
    pub fn from_query(query: &str) -> Option<Self> {
        let trimmed = query.strip_prefix('?').unwrap_or(query);
        let mut has_gallery = false;
        let mut gallery_value: Option<String> = None;
        let mut story: Option<String> = None;
        let mut search_query = String::new();
        let mut viewport_width = DEFAULT_VIEWPORT_WIDTH;
        let mut viewport_height = DEFAULT_VIEWPORT_HEIGHT;
        let mut list_hidden = false;
        for pair in trimmed.split('&') {
            if pair.is_empty() {
                continue;
            }
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or_default();
            let value = parts.next();
            match key {
                "gallery" => {
                    has_gallery = true;
                    gallery_value = value.map(str::to_string);
                }
                "story" => {
                    story = value.map(Percent::decode);
                }
                "q" => {
                    if let Some(raw) = value {
                        search_query = Percent::decode(raw);
                    }
                }
                "w" => {
                    let parsed = value.and_then(|raw| raw.parse::<u32>().ok());
                    if let Some(width) = parsed {
                        viewport_width = width;
                    }
                }
                "h" => {
                    let parsed = value.and_then(|raw| raw.parse::<u32>().ok());
                    if let Some(height) = parsed {
                        viewport_height = height;
                    }
                }
                "list" if value == Some("hidden") => {
                    list_hidden = true;
                }
                _ => {}
            }
        }
        if !has_gallery {
            return None;
        }
        match gallery_value.as_deref() {
            Some("frame") => story.map(|story| Self::Frame { story }),
            _ => {
                let view = GalleryView::new(
                    story,
                    search_query,
                    viewport_width,
                    viewport_height,
                    list_hidden,
                );
                Some(Self::Shell { view })
            }
        }
    }
}
