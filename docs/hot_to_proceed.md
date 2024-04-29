# ゲームボーイの実装の進め方

## 1. ゲームボーイの仕様を理解する

-[ ] PanDocなどの資料を参考に読み進める
    - [The Ultimate Game Boy Talk (33c3)](https://www.youtube.com/watch?v=HyzD8pNlpwI)
    - [Pandocs](https://gbdev.io/pandocs/)
    - [Gameboy CPU (LR35902) instruction set](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html):
      ゲームボーイのCPU (LR35902) の全命令をまとめた表． 一覧性が高く，オペランドのフォーマットや命令のクロック数も記載されているため，
      命令デコーダの作成に重宝する．ただし，一部のクロック数は間違っているという罠がある．
    - [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf): CPU の各命令の詳しい動作が記載されている．フラグがセット・クリアされる条件など，
      上の表よりも詳しい．ただし，一部の命令 (DAA命令など) の動作はこの資料でもまだ情報が足りないため，他の資料をあたることになる．
    - [akatsuki105/gb-docs-ja](https://github.com/akatsuki105/gb-docs-ja)
    - [Gameboy CPU (LR35902) instruction set](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
      ：オペコード一覧表。CPUの実装はここを見ながらすることになりますが、たまに間違っています。
    - [Game Boy Programming Manual](https://web.archive.org/web/20150513170240/http://www.chrisantonellis.com:80/files/gameboy/gb-programming-manual.pdf)
      ：Nintendo in Americaが出した（？）公式の（？）仕様書。細かい処理などを確認するときに便利です。

- [ ] ゲームボーイの構造を理解し、エミュレータ用にどう再構築するか考える
  - [ ] ゲームボーイエミュレータのコード規模は中規模のため、単体テスト可能な実装ができるように作る必要がある
  - [ ] テストROMは実機で通ることが確認されているため、ドキュメントよりも信頼できる。Blargg’s test
    romsには複数のROMが含まれているが、中でも命令の動作をテストするcpu_instrsと，命令のクロック数をテストするinstr_timingは通らないと，
    実際のゲームは動かないと思う．逆に言えば，他のテストは通らなくても何とかなる．

## 2. ゲームボーイのCPUを実装する

## 3. ゲームボーイのメモリを実装する

## 4. データ連携を行っているBUSを実装する

- 循環参照（密結合）にならないように、パッケージ化する

## 5. ゲームボーイのPPUを実装する

## 6. ゲームボーイのAPUを実装する

- ゲームエンジンとAPUをどのように接続するかが問題になる
- 以下の記事が参考になる
    - [Game Boy Sound Emulation](https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html)