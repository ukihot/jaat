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
            fieldset { class: "first_atack", onchange: move |evt| { log::info!("{:?}", evt) },
                legend { "FIRST ATTACK?" }
                div { class: "radio_area",
                    input {
                        id: "dog_first",
                        r#type: "radio",
                        name: "first_atack",
                        value: "0",
                        checked: ""
                    }
                    label { r#for: "dog_first", "dog_team_name" }
                }
                div { class: "radio_area",
                    input { id: "mon_first", r#type: "radio", name: "first_atack", value: "1" }
                    label { r#for: "mon_first", "mon_team_name" }
                }
            }
            // Since there is no need to manage the state of each and every word, use_state is not used.
            form { onsubmit: move |evt| { log::info!("{:?}", evt) },
                (1..=9).map(|n| {
                    rsx! {
                        div { class: "grid_container",
                        div { class: "uniform",
                        label { r#for:"uniform_{n}", "背番号" }
                        input {
                            r#type: "text",
                            id: "uniform_{n}",
                            class: "player_info",
                            name: "uniform_number"
                        }
                    }
                    div { class: "name",
                    label { r#for: "name_{n}", "氏名" }
                    input {
                        r#type: "text",
                        id: "name_{n}",
                        class: "player_info",
                        name: "name"
                    }
                }
                div { class: "gender",
                    label { r#for: "gender_{n}", "性別" }
                    input {
                        r#type: "text",
                        id: "gender_{n}",
                        class: "player_info",
                        name: "gender"
                    }
                }
                div { class: "height",
                    label { r#for: "height{n}", "身長" }
                    input {
                        r#type: "text",
                        id: "height_{n}",
                        class: "player_info",
                        name: "height"
                    }
                }
                div { class: "weight",
                    label { r#for: "weight{n}", "体重" }
                    input {
                        r#type: "text",
                        id: "weight_{n}",
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
