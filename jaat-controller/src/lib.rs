#![allow(non_snake_case)]
mod dimensions;
mod facts;
mod styles;
use dioxus::prelude::*;
use styles::Style;

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
        dimensions::MasterRecorder {}
        facts::LiveScorer {}
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
