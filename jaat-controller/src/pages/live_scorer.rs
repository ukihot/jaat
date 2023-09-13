#![allow(non_snake_case)]
use crate::styles::Style;
use dioxus::prelude::*;

pub fn LiveScorer(cx: Scope) -> Element {
    cx.render(rsx! {
        // Load style sheet
        style { Style::RESET }
        style { Style::TEXTFORM_STYLES }
        style { Style::GRID_STYLES }
        style { Style::TOGGLE_STYLES }
        style { Style::RADIO_STYLES }

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
    })
}
