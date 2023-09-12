use std::sync::Arc;

use jaat_core::repository::raid_repository::IRaidRepository;

pub struct RaidService {
    repository: Arc<dyn IRaidRepository>,
}

impl RaidService {
    pub fn new(repository: Arc<dyn IRaidRepository>) -> Self {
        Self { repository }
    }

    pub fn call_foo(&self) -> String {
        self.repository.call_foo().unwrap()
    }
}
