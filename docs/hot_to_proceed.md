# ゲームボーイ実装の進め方

このドキュメントは、[README.md](../README.md) にある参照資料と実装例を起点に、
Game Boy エミュレータ実装をどう進めるかを整理したものです。

## 1. 先に読む資料

調査結果は [gameboy_architecture_summary.md](./gameboy_architecture_summary.md) にまとめた。

### 1.1. 仕様と命令セット
- [Pandocs](https://gbdev.io/pandocs/)
  - まず最初に読む基礎資料。メモリマップ、レジスタ、PPU、タイマ、割り込みの全体像を掴む。
- [Gameboy CPU (LR35902) instruction set](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
  - 命令デコーダを作るときの一覧表として便利。ただし一部のクロック数は誤りがあるので、他資料で確認する。
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
  - 各命令の詳しい動作やフラグ更新条件を確認する用途で使う。
- [Game Boy Programming Manual](https://web.archive.org/web/20150513170240/http://www.chrisantonellis.com:80/files/gameboy/gb-programming-manual.pdf)
  - 公式寄りの資料として、レジスタや周辺機能の挙動確認に役立つ。
- [Game Boy: Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)
  - さらに細かいハードウェア挙動を確認したいときの詳細リファレンス。

### 1.2. 参考リンク集
- [gbdev/awesome-gbdev](https://github.com/gbdev/awesome-gbdev)
  - 追加資料やツール、テスト ROM を探す入口として使う。
- [akatsuki105/gb-docs-ja](https://github.com/akatsuki105/gb-docs-ja)
  - 英語資料で詰まった箇所の補助として参照する。

## 2. 参考にする実装

README にまとまっている既存実装は、設計の比較対象として一度眺めておくとよいです。
特に「Bus をどう切るか」「CPU と周辺機器の依存をどう持たせるか」を見る価値があります。

### 2.1. Rust
- [tanakh/tgbr](https://github.com/tanakh/tgbr)
- [keichi/gbr](https://github.com/keichi/gbr)

### 2.2. Go
- [akatsuki105/worldwide](https://github.com/akatsuki105/worldwide)
- [mohanson/dwangb](https://github.com/akashin/dwangb)

### 2.3. Ruby
- [sacckey/rubyboy](https://github.com/sacckey/rubyboy)

## 3. テスト ROM と検証方針

- [roms](../roms/)
  - このリポジトリに検証用 ROM がまとまっている。
- [blargg-gb-tests](../roms/blargg-gb-tests/)
  - CPU 命令、タイミング、周辺機器の基本検証に使う。
- [mooneye](../roms/mooneye/)
  - blargg だけでは拾いきれない細かなハードウェア挙動の検証に使う。

テスト ROM は仕様書より信頼できる場面があります。実装中に資料同士で記述が食い違ったら、
まずテスト ROM の結果を基準に確認する方が堅いです。

## 4. 実装方針

- Bus を中心に据えて、CPU / PPU / Timer / Joypad / Cartridge を疎結合にする。
- 単体テストしやすいように、レジスタ操作・命令実行・メモリアクセスを小さく分離する。
- CPU の命令実装を先に固め、その後にタイマ・割り込み・PPU を積み上げる。
- 仕様を読むだけで確定しない部分は、既存実装とテスト ROM の両方で裏を取る。

## 5. 実装のロードマップ

### 5.1. メモリバス (Bus) とメモリ
- [ ] メモリアドレスマップを定義する
- [ ] 16-bit address / 8-bit data の read/write インターフェースを作る
- [ ] CPU から各デバイスへ Bus 経由でアクセスできるようにする

### 5.2. CPU (LR35902)
- [ ] レジスタ `A, B, C, D, E, H, L, F, SP, PC` を定義する
- [ ] 命令フェッチ、デコード、実行の流れを実装する
- [ ] Load / Arithmetic / Bit / Jump / Call / Return を順に実装する
- [ ] [cpu_instrs](../roms/blargg-gb-tests/cpu_instrs/) で命令実装を検証する

### 5.3. タイマと割り込み
- [ ] `DIV`, `TIMA`, `TMA`, `TAC` を実装する
- [ ] `IF`, `IE`, `IME` を含む割り込み制御を実装する
- [ ] [instr_timing](../roms/blargg-gb-tests/instr_timing/) でクロック精度を確認する

### 5.4. PPU
- [ ] Background の描画を実装する
- [ ] Window の描画を実装する
- [ ] Sprite / OAM の描画を実装する
- [ ] VRAM / OAM のアクセス制御を実装する
- [ ] Mode 0-3 と VBlank 割り込みを実装する

### 5.5. 入力 (Joypad)
- [ ] 十字キー、A、B、Select、Start の状態管理を実装する
- [ ] 入力変化に伴う割り込みを実装する

### 5.6. カートリッジ (MBC)
- [ ] MBC0, MBC1, MBC3 など主要 MBC を実装する
- [ ] 外部 RAM の保存と読み込みを実装する

### 5.7. APU
- [ ] Pulse 1 / Pulse 2 / Wave / Noise を順に実装する
- [ ] オーディオ出力に接続する

## 6. 参考ブログ

README にまとまっているブログは、詰まりやすいポイントの当たりを付けるのに向いています。

- [ゲームボーイのエミュレータを自作した話](https://keichi.dev/post/write-yourself-a-game-boy-emulator/)
- [ゲームボーイを作る（１）](https://www.tech-diningyo.info/entry/2021/07/10/222140)
- [ゲームボーイのエミュレータをGoで作った話](https://zenn.dev/akatsuki/articles/ec95ab95f0e89ea8c38f)
- [C++でゲームボーイエミュレータを自作しています](https://voidproc.com/blog/archives/664)
- [脱・初級者のための自作GBエミュレータ開発](https://www.docswell.com/s/linoscope/ZNRRXL-game-boy-emulator-ocaml)
- [OCaml でゲームボーイエミュレータを書いた話](https://qiita.com/linoscope/items/244d931aaae07df2c27e)
- [Rubyでゲームボーイのエミュレータを作った](https://zenn.dev/sacckey/articles/05b6eb6ea89662)
- [AQBoy: Yet Another Game Boy Emulator 開発記](https://hackmd.io/@anqou/HJcvRrwy9)
- [GameBoy Emulation in JavaScript](https://imrannazar.com/series/gameboy-emulation-in-javascript)
- [自作ゲームボーイエミュレータメモ](https://qiita.com/kmtoki/items/578e8e57ab0e76590d6d)
