#![allow(non_snake_case)]
mod components;
mod dimensions;
mod facts;
mod styles;
use dioxus::prelude::*;
use jaat_data::{raid::raid_data::RaidRepositoryImpl, Database};
use jaat_usecase::query::raid_services::RaidService;
use std::sync::Arc;
use styles::Style;

pub fn presentation(cx: Scope) -> Element {
    // Initialization of logger
    wasm_logger::init(wasm_logger::Config::default());

    cx.render(rsx! {
        // Load style sheet
        style { Style::RESET }
        style { Style::TEXTFORM_STYLES }
        style { Style::GRID_STYLES }
        style { Style::TOGGLE_STYLES }
        style { Style::RADIO_STYLES }

        dimensions::MasterRecorder {}
        facts::LiveScorer {}
    })
}


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
