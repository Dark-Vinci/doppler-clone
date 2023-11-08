#[derive(Debug)]
pub struct AppData {
    pub store: String,
    pub downstream: String,
    pub secret: String,
}

impl AppData {
    pub fn new(
        store: String,
        downstream: String,
        secret: String,
    ) -> Self {
        Self {
            store,
            downstream,
            secret,
        }
    }
}
