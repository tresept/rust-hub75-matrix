# rust-hub75-matrix

`rpi-rgb-led-matrix` を通じて、完成済みのRGB8フレームをHUB75パネルへ安全に出力する薄いRustラッパーです。PNGのデコード、文字描画、合成、アニメーションは本クレートの責務ではありません。

初期対応は Raspberry Pi 5 / Raspberry Pi OS Lite 64-bit / 64×32 HUB75 パネル（デイジーチェーン対応）です。C++ ABIや上流オプション構造体は公開せず、小さなCブリッジだけをFFI境界にしています。

## Setup

上流は固定コミットのgit submoduleです。clone後は次を実行してください。

```sh
git submodule update --init --recursive
sudo apt install build-essential
cargo build --release
```

GPIOへのアクセスには通常root権限が必要です。64×32パネル2枚（128×32）の例は次のとおりです。

```sh
sudo cargo run --release --example show_png -- image.png \
  --rows 32 --cols 64 --chain-length 2 --brightness 30 --rp1-backend rio
```

`--rp1-backend pio` でPi 5のRP1 PIOバックエンドを選べます。RIOが標準です。信号が速すぎて表示が乱れるパネルは`MatrixConfig::gpio_slowdown`を上げてください。パネルにより`multiplexing`、`row_address_type`、pixel mapper等の調整が必要です。

```rust
use rust_hub75_matrix::{Matrix, MatrixConfig, Rp1Backend};

let config = MatrixConfig {
    rows: 32, cols: 64, chain_length: 2, brightness: 30,
    rp1_backend: Rp1Backend::Rio,
    ..Default::default()
};
let mut matrix = Matrix::new(config)?;
let frame = vec![0_u8; matrix.width() * matrix.height() * 3];
matrix.present_rgb(&frame)?;
# Ok::<(), rust_hub75_matrix::Error>(())
```

フレームは行パディングなしの`R, G, B, R, G, B, ...`形式です。`present_rgb()`は論理キャンバスと完全に一致するバイト数だけを受け付け、オフスクリーンキャンバスへの転送後にVSyncで交換します。

## Examples

- `solid_color`: 単色表示。Ctrl+Cで消灯します。
- `show_png`: PNGをexample側でRGB8へ変換し、必要なら`--resize`でNearest Neighborリサイズします。`--duration`で表示秒数を指定できます。
- `animation`: 毎フレームRGB8バッファを生成して送る最小例です。

## License and upstream

このクレートはGPL-2.0-or-laterです。上流は[hzeller/rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix)（GPL-2.0-or-later）で、submoduleは`41809e40e912b7f278ad34046f20abf5609b2b07`へ固定されています。上流コードへの変更はありません。配布時は本クレートとリンクするアプリケーションにもGPLの条件が及ぶことを確認してください。
