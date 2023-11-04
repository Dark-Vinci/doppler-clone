use crate::application::application::Applications;

#[derive(Debug)]
pub struct Controllers(Applications);

impl Controllers {
    fn new(app: Applications) -> Self {
        Self(app)
    }
}