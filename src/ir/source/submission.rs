use std::sync::Arc;

#[salsa::input(debug)]
pub struct Submission {
    pub contents: Arc<str>,
}
