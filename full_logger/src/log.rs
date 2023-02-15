pub struct Log {
    pub class: String,
    pub content: String,
    pub name: String,
}

impl Log {
    pub fn new(class: String, content: String, name: String) -> Self {
        Self {
            class,
            content,
            name,
        }
    }
}
