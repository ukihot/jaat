#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use jaat_controller::pages::{enrty_form::EntryForm, live_scorer::LiveScorer};

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    EntryForm {},
    #[route("/live")]
    LiveScorer {},
}

pub fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}
