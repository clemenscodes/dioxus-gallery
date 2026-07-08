use crate::percent::Percent;

pub(crate) const DEFAULT_VIEWPORT_WIDTH: u32 = 1440;
pub(crate) const DEFAULT_VIEWPORT_HEIGHT: u32 = 900;

/// The full shell state that survives a reload, encoded in the page query
/// string. Defaults match the shell's own defaults so an untouched gallery
/// serializes to the bare `gallery` marker with nothing else.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GalleryView {
    story: Option<String>,
    query: String,
    viewport_width: u32,
    viewport_height: u32,
    list_hidden: bool,
}

impl Default for GalleryView {
    fn default() -> Self {
        Self {
            story: None,
            query: String::new(),
            viewport_width: DEFAULT_VIEWPORT_WIDTH,
            viewport_height: DEFAULT_VIEWPORT_HEIGHT,
            list_hidden: false,
        }
    }
}

impl GalleryView {
    pub fn new(
        story: Option<String>,
        query: String,
        viewport_width: u32,
        viewport_height: u32,
        list_hidden: bool,
    ) -> Self {
        Self {
            story,
            query,
            viewport_width,
            viewport_height,
            list_hidden,
        }
    }

    pub fn story(&self) -> Option<&str> {
        self.story.as_deref()
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn viewport_width(&self) -> u32 {
        self.viewport_width
    }

    pub fn viewport_height(&self) -> u32 {
        self.viewport_height
    }

    pub fn list_hidden(&self) -> bool {
        self.list_hidden
    }

    /// Serializes to the query string body (no leading `?`). Default and empty
    /// values are omitted so links stay short and readable.
    pub fn to_query(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        let marker = String::from("gallery");
        parts.push(marker);
        if let Some(story) = &self.story {
            let encoded = Percent::encode(story);
            let pair = format!("story={encoded}");
            parts.push(pair);
        }
        if !self.query.is_empty() {
            let encoded = Percent::encode(&self.query);
            let pair = format!("q={encoded}");
            parts.push(pair);
        }
        if self.viewport_width != DEFAULT_VIEWPORT_WIDTH {
            let width = self.viewport_width;
            let pair = format!("w={width}");
            parts.push(pair);
        }
        if self.viewport_height != DEFAULT_VIEWPORT_HEIGHT {
            let height = self.viewport_height;
            let pair = format!("h={height}");
            parts.push(pair);
        }
        if self.list_hidden {
            let pair = String::from("list=hidden");
            parts.push(pair);
        }
        parts.join("&")
    }
}
