#![allow(non_snake_case)]
mod style;
use dioxus::prelude::*;
use style::Style;

/// I do not prefer the variable names "A team" and "B team".
/// Here we deal with dogs and monkeys.
pub fn Controller(cx: Scope) -> Element {
    // Initialization of logger
    wasm_logger::init(wasm_logger::Config::default());

    cx.render(rsx!(
        // Load style sheet
        style { Style::RESET }
        style { Style::TEXTFORM_STYLES }
        style { Style::GRID_STYLES }
        style { Style::TOGGLE_STYLES }
        style { Style::RADIO_STYLES }

        h1 { "Jaat!" }

        // Master items
        div {
            // Since there is no need to manage the state of each and every word, use_state is not used.
            form { class: "grid_container", onsubmit: move |evt| { log::info!("{:?}", evt) },
                input { r#type: "text", class: "player_info", name: "uniform_number" }
                input { r#type: "text", class: "player_info", name: "name" }
                input { r#type: "text", class: "player_info", name: "gender" }
                input { r#type: "text", class: "player_info", name: "height" }
                input { r#type: "text", class: "player_info", name: "weight" }
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

        // Match Live Information
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
    ))
    // display:ボタン群の上で攻め側のチームをパネルで表示
    // display:raiderとantiで色分け
    // display:raid側チームのボタンは一つのみクリック可
    // usecase:raider記録

    // display:anti側チームのボタンは複数クリック可
    // usecase:anti記録

    // display:Bonus有無slider
    // usecase:is_bonus記録

    // enter:scenarioプルダウン
    // display:プルダウンの選択結果からis_success
    // display:pointの自動計算
    // display:commitボタンは新規か修正かでlabel切り替え
    // usecase:cycle番号、担当raider、関係anti、pointの記録

    // display:画面削除
    // display:cycleのインクリメント

    // display:戻るボタン
    // usecase:過去コミットの修正

    // display:試合終了
    // usecase:メトリクス指標の計算
    // display:リザルト画面
    // usecase: csv出力
}
