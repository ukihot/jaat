#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn MasterRecorder(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            // Since there is no need to manage the state of each and every word, use_state is not used.
            form { onsubmit: move |evt| { log::info!("{:?}", evt) },
                div { class: "grid_container",
                    div { class: "uniform",
                        label { r#for: "uniform_1", "背番号" }
                        input {
                            r#type: "text",
                            id: "uniform_1",
                            class: "player_info",
                            name: "uniform_number"
                        }
                    }
                    div { class: "name",
                        label { r#for: "name_1", "氏名" }
                        input {
                            r#type: "text",
                            id: "name_1",
                            class: "player_info",
                            name: "name"
                        }
                    }
                    div { class: "gender",
                        label { r#for: "gender_1", "性別" }
                        input {
                            r#type: "text",
                            id: "gender_1",
                            class: "player_info",
                            name: "gender"
                        }
                    }
                    div { class: "height",
                        label { r#for: "height_1", "身長" }
                        input {
                            r#type: "text",
                            id: "height_1",
                            class: "player_info",
                            name: "height"
                        }
                    }
                    div { class: "weight",
                        label { r#for: "weight_1", "体重" }
                        input {
                            r#type: "text",
                            id: "weight_1",
                            class: "player_info",
                            name: "weight"
                        }
                    }
                }
                input { r#type: "submit", value: "REGISTER" }
            }

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

            // Record members belonging to
            div {}
        }
    })
}
