use crate::percent::Percent;

/// Builds the iframe source URL for a story. Pure, so the gallery never reads
/// the browser to construct it; the host supplies the page path.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FramePath {
    base_path: String,
}

impl FramePath {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn src(&self, story_id: &str) -> String {
        let encoded = Percent::encode(story_id);
        format!("{}?gallery=frame&story={}", self.base_path, encoded)
    }
}
