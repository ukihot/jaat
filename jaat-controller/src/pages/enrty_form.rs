#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::styles::Style;

pub fn EntryForm(cx: Scope) -> Element {
    cx.render(rsx! {
        // Load style sheet
        style { Style::RESET }
        style { Style::TEXTFORM_STYLES }
        style { Style::GRID_STYLES }
        style { Style::TOGGLE_STYLES }
        style { Style::RADIO_STYLES }
        div {
            // Which team won rock-scissors-paper

            // Since there is no need to manage the state of each and every word, use_state is not used.
            form { onsubmit: move |evt| { log::info!("{:?}", evt) }, class: "grid_form",
                (1..=18).map(|n| {

                    let team = if n > 9 { "mon" } else { "dog" };
                    rsx! {
                        div { class: "grid_form_details",
                        div { class: "uniform",
                        label { r#for:"{team}_uniform_{n}", "背番号" }
                        input {
                            r#type: "text",
                            id: "{team}_uniform_{n}",
                            class: "player_info",
                            name: "uniform_number"
                        }
                    }
                    div { class: "name",
                    label { r#for: "{team}_{team}_name_{n}", "氏名" }
                    input {
                        r#type: "text",
                        id: "{team}_name_{n}",
                        class: "player_info",
                        name: "name"
                    }
                }
                div { class: "gender",
                    label { r#for: "{team}_gender_{n}", "性別" }
                    input {
                        r#type: "text",
                        id: "{team}_gender_{n}",
                        class: "player_info",
                        name: "gender"
                    }
                }
                div { class: "height",
                    label { r#for: "{team}_height{n}", "身長" }
                    input {
                        r#type: "text",
                        id: "{team}_height_{n}",
                        class: "player_info",
                        name: "height"
                    }
                }
                div { class: "weight",
                    label { r#for: "{team}_weight{n}", "体重" }
                    input {
                        r#type: "text",
                        id: "{team}_weight_{n}",
                        class: "player_info",
                        name: "weight"
                    }
                }
            }
        }
    }),
                input { r#type: "submit", value: "REGISTER" }
            }

            Link { to: "/live", "Go" }
        }
    })
}
