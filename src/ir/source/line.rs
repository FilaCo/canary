use std::sync::Arc;

#[salsa::input(debug)]
pub struct Line {
    pub contents: Arc<str>,
}
