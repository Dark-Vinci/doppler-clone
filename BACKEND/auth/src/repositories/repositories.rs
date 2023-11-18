use std::fmt::{Debug, Display, Formatter};

use crate::repositories::business::BusinessRepository;

#[derive(Debug)]
pub struct Repositories {
    business: BusinessRepository,
}

impl Display for Repositories {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Repositories {
    fn new(db: String) -> Self {
        let business_repository = BusinessRepository::new(db);

        Self {
            business: business_repository,
        }
    }
}
