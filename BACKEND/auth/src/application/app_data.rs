#[derive(Debug)]
pub struct AppData {
    store: String,
    downstream: String,
    secret: String,
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