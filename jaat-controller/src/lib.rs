#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Controller(cx: Scope) -> Element {
    let mut count_a: &UseState<i32> = use_state(cx, || 0);
    let mut count_b: &UseState<i32> = use_state(cx, || 0);

    cx.render(rsx!(
        h1 { "Counter_a: {count_a}" }
        button { onclick: move |_| count_a += 1, "a++" }
        button { onclick: move |_| count_a -= 1, "a--" }
        h1 { "Counter_b: {count_b}" }
        button { onclick: move |_| count_b += 1, "b++" }
        button { onclick: move |_| count_b -= 1, "b--" }
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
