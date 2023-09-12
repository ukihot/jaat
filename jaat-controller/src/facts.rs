#![allow(non_snake_case)]
use dioxus::prelude::*;



pub fn LiveScorer(cx: Scope) -> Element {
    cx.render(rsx! {
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
