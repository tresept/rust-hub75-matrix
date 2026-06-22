# GIF Example Design

## Goal

既存の画像表示exampleでPNGに加えてアニメーションGIFを表示できるようにする。画像デコードはクレート本体ではなくdev-dependencyを使うexampleの責務に保つ。

## Design

`examples/show_png.rs`を`examples/show_image.rs`へ置き換える。CLIは既存のパネル設定、`--resize`、`--duration`を維持し、入力画像を`image`クレートの`ImageReader`で判別する。

静止画像は現在と同じくRGB8へ変換して一度だけVSync表示する。GIFでは`image::codecs::gif::GifDecoder`でフレームを収集し、各フレームをキャンバスサイズへ検証またはNearest Neighborでリサイズした後、フレーム固有の遅延時間で順番に`Matrix::present_rgb()`へ渡す。アニメーションは`--duration`を指定された場合はその時点で、指定なしの場合はCtrl+Cで終了する。

Ctrl+Cは`ctrlc`ハンドラがatomic flagを更新する。通常の終了経路では`Matrix::clear()`後に100ms待機し、黒フレームをラッチする。静止画像とGIFはこの終了処理を共有する。

## Constraints

- ライブラリ本体にPNG/GIF依存を追加しない。
- GIFデコード機能は`image`のdev-dependency featureとして有効化する。
- 入力サイズ不一致は`--resize`なしではエラーにする。
- フレーム遅延はGIFのメタデータに従い、0msは10msとして扱う。
- 実機のGPIOテストはRaspberry Piで行う。
