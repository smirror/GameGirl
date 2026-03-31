# ゲームボーイの実装の進め方

## 1. ゲームボーイの仕様を理解する

### 1.1. 資料を読み込む
- [ ] PanDocなどの資料を参考に読み進める
    - [The Ultimate Game Boy Talk (33c3)](https://www.youtube.com/watch?v=HyzD8pNlpwI)
    - [Pandocs](https://gbdev.io/pandocs/)
    - [Gameboy CPU (LR35902) instruction set](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html):
      ゲームボーイのCPU (LR35902) の全命令をまとめた表． 一覧性が高く，オペランドのフォーマットや命令のクロック数も記載されているため，
      命令デコーダの作成に重宝する．ただし，一部のクロック数は間違っているという罠がある．
    - [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf): CPU の各命令の詳しい動作が記載されている．フラグがセット・クリアされる条件など，
      上の表よりも詳しい．ただし，一部の命令 (DAA命令など) の動作はこの資料でもまだ情報が足りないため，他の資料をあたることになる．
    - [gbdev/awesome-gbdev](https://github.com/gbdev/awesome-gbdev)
      ：ゲームボーイ開発に関するリソースが網羅されている。
    - [akatsuki105/gb-docs-ja](https://github.com/akatsuki105/gb-docs-ja)
    - [Game Boy Programming Manual](https://web.archive.org/web/20150513170240/http://www.chrisantonellis.com:80/files/gameboy/gb-programming-manual.pdf)
      ：Nintendo in Americaが出した（？）公式の（？）仕様書。細かい処理などを確認するときに便利です。
    - [Game Boy: Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)
      ：ハードウェアの物理的な特性まで踏み込んだ詳細な技術リファレンス。

### 1.2. 設計と方針の検討
- [ ] ゲームボーイの構造を理解し、エミュレータ用にどう再構築するか考える
    - [ ] ゲームボーイエミュレータのコード規模は中規模のため、単体テスト可能な実装ができるように作る必要がある
    - [ ] [blargg-gb-tests](https://github/smirror/GmaeGirl/roms/blargg-gb-tests) :テストROMは実機で通ることが確認されているため、ドキュメントよりも信頼できる。
    - [ ] 循環参照（密結合）にならないように、バスを中心とした設計やパッケージ構成（Traitによる抽象化など）を検討する

## 2. 実装のロードマップ

ゲームボーイエミュレータの実装は、以下の順序で進めるのが一般的です。

### 2.1. メモリバス (Bus) とメモリの実装
- [ ] メモリアドレスマップの実装
    - 各デバイス（ROM, RAM, PPU, APU, Joypad, Timer等）のアドレス範囲を定義する
- [ ] 16bitアドレス、8bitデータの読み書きインターフェース（Read/Write）を定義する
- [ ] メモリバスを通じてCPUからメモリへアクセスできるようにする

### 2.2. CPU (LR35902) の実装
- [ ] レジスタ（A, B, C, D, E, H, L, F, SP, PC）の定義
- [ ] 命令デコーダの実装
- [ ] 各命令（Load, Arithmetic, Bit, Jump, Call, Return等）のロジック実装
- [ ] [blargg-gb-tests/cpu_instrs](https://github.com/smirror/GameGirl/tree/main/roms/blargg-gb-tests/cpu_instrs) を利用したデバッグ
    - 各命令が正しく動作することを確認する

### 2.3. タイマーと割り込みの実装
- [ ] タイマー（DIV, TIMA, TMA, TAC）の実装
- [ ] 割り込みフラグ（IF）と割り込み有効化（IE）の制御
- [ ] [blargg-gb-tests/instr_timing](https://github.com/smirror/GameGirl/tree/main/roms/blargg-gb-tests/instr_timing) を利用したクロック精度の確認

### 2.4. PPU (Picture Processing Unit) の実装
- [ ] 背景（Background）の描画実装
- [ ] ウィンドウ（Window）の描画実装
- [ ] オブジェクト（Sprite/OAM）の描画実装
- [ ] VRAM, OAMへのアクセス制御
- [ ] 描画タイミング（Mode 0, 1, 2, 3）と VBlank 割り込みの実装

### 2.5. 入力 (Joypad) の実装
- [ ] ボタン入力（十字キー、A, B, Select, Start）の状態管理
- [ ] 入力状態の変化に伴う割り込みの実装

### 2.6. カートリッジ (MBC) の実装
- [ ] MBC0 (No MBC), MBC1, MBC3 などのメモリバンクコントローラの実装
- [ ] 外部RAM（SRAM）の保存・読み込み

### 2.7. APU (Audio Processing Unit) の実装
- [ ] 各チャンネル（Pulse 1, Pulse 2, Wave, Noise）の実装
- [ ] 波形データの出力とゲームエンジン（オーディオ出力）への接続

## 3. 参考ブログ・実装例（READMEより抜粋）
- [ゲームボーイのエミュレータを自作した話](https://keichi.dev/post/write-yourself-a-game-boy-emulator/)
    - 全体像の把握に最適。実装のフェーズ分けが参考になる。
- [ゲームボーイを作る（１）](https://www.tech-diningyo.info/entry/2021/07/10/222140)
    - PPUのタイミングやレジスタの詳細が詳しい。
- [OCaml でゲームボーイエミュレータを書いた話](https://qiita.com/linoscope/items/244d931aaae07df2c27e)
    - 複雑な割り込みやタイマーの挙動、デバッグ手法について詳しい。
- [AQBoy: Yet Another Game Boy Emulator 開発記](https://hackmd.io/@anqou/HJcvRrwy9)
    - 実装中にハマりやすいポイント（DAA命令、タイマーのインクリメントタイミング等）がまとめられている。