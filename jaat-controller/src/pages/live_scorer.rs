#![allow(non_snake_case)]
use crate::styles::Style;
use dioxus::prelude::*;

pub fn LiveScorer(cx: Scope) -> Element {
    let raid_id: &UseState<u8> = use_state(cx, || 1);
    cx.render(rsx! {
        // Load style sheet
        style { Style::RESET }
        style { Style::TEXTFORM_STYLES }
        style { Style::GRID_STYLES }
        style { Style::TOGGLE_STYLES }
        style { Style::RADIO_STYLES }
        h1 { "ID:{raid_id}" }
        div {
            label { class: "switch_label",
                input {
                    r#type: "checkbox",
                    class: "switch_input",
                    onchange: |evt| log::info!("{:?}", evt)
                }
                span { class: "switch_content" }
                span { class: "switch_circle" }
            }
        }
        div { class: "scorer-footer",
            button { onclick: move |_| raid_id.set(raid_id - 1), "<< REVERT" }
            button { onclick: move |_| raid_id.set(raid_id + 1), "COMMIT >>" }
        }
    })
}
