#![allow(non_snake_case)]
mod components;
pub mod pages;
mod styles;
use jaat_data::{raid::raid_data::RaidRepositoryImpl, Database};
use jaat_usecase::query::raid_services::RaidService;
use std::sync::Arc;

// View (UI) realizes UseCase via Controller
// The controller is responsible for converting input values and is prohibited from taking on any other heavy responsibility.
pub struct Controller {
    pub raid_service: RaidService,
}

impl Default for Controller {
    fn default() -> Self {
        let data = Database::new();
        let raid_service = RaidService::new(Arc::new(RaidRepositoryImpl::new(data)));
        Self { raid_service }
    }
}
