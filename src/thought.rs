#[derive(Clone)]
pub struct Thought {
    pub contents: String,
    pub title: String,
    // And so on
}

impl Thought {
    pub fn new(title: String) -> Self {
        Self {
            title,
            contents: String::new(),
        }
    }
}