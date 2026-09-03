pub struct Window {
    id: u32,
    title: String,
}

impl Window {
    pub fn new(id: u32, title: &str) -> Self {
        Self { id, title: title.to_string() }
    }
}
