#![allow(non_snake_case)]
mod style;
use dioxus::prelude::*;
use style::Style;

/// I do not prefer the variable names "A team" and "B team".
/// Here we deal with dogs and monkeys.
pub fn Controller(cx: Scope) -> Element {
    let dog_team_name: &UseState<String> = use_state(cx, || "".to_string());
    let dog1 = use_state(cx, || "".to_string());
    let mon_team_name: &UseState<String> = use_state(cx, || "".to_string());

    cx.render(rsx!(
        style { Style::TOGGLE_STYLES }
        h1 { "Jaat!" }
        form { onsubmit: move |event| { println!("Submitted! {event:?}") },
            input { name: "name" }
            input { name: "age" }
            input { name: "date" }
            input { r#type: "submit" }
        }
        input {
            // we tell the component what to render
            value: "{dog_team_name}",
            // and what to do when the value changes
            oninput: move |evt| dog_team_name.set(evt.value.clone())
        }
        div {
            p { "Members of {dog_team_name}" }
            input {
                // we tell the component what to render
                value: "{dog1}",
                // and what to do when the value changes
                oninput: move |evt| dog1.set(evt.value.clone())
            }
        }
        input {
            // we tell the component what to render
            value: "{mon_team_name}",
            // and what to do when the value changes
            oninput: move |evt| mon_team_name.set(evt.value.clone())
        }
        fieldset {
            legend { "FIRST ATTACK" }
            div {
                input { r#type: "radio", name: "f" }
                label { "{dog_team_name}" }
            }
            div {
                input { r#type: "radio", name: "f" }
                label { "{mon_team_name}" }
            }
        }
        p { class: "foo", "test" }
        div {
            label { class: "switch_label",
                input { r#type: "checkbox", class: "switch_input" }
                span { class: "switch_content" }
                span { class: "switch_circle" }
            }
        }
    ))
    // enter:チーム名
    // enter:先行後攻
    // enter:プレイヤー情報
    // display:7×2のボタン群
    // usecase:labelにplayerの名前を付与

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
