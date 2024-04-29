# GameGirl

## Reference

### Docs

- [Pandocs](http://gbdev.gg8.se/wiki/articles/Pan_Docs)
    - [Original](http://marc.rawer.de/Gameboy/Docs) (but not updated)
- [Gameboy CPU (LR35902) instruction set](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html):
  ゲームボーイのCPU (LR35902) の全命令をまとめた表． 一覧性が高く，オペランドのフォーマットや命令のクロック数も記載されているため，
  命令デコーダの作成に重宝する．ただし，一部のクロック数は間違っているという罠がある．
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf): CPU の各命令の詳しい動作が記載されている．フラグがセット・クリアされる条件など，
  上の表よりも詳しい．ただし，一部の命令 (DAA命令など) の動作はこの資料でもまだ情報が足りないため，他の資料をあたることになる．
- [gbdev/awesome-gbdev](https://github.com/gbdev/awesome-gbdev)
- [akatsuki105/gb-docs-ja](https://github.com/akatsuki105/gb-docs-ja)
- [Gameboy CPU (LR35902) instruction set](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)：オペコード一覧表。CPUの実装はここを見ながらすることになりますが、たまに間違っています。
- [Game Boy Programming Manual](https://web.archive.org/web/20150513170240/http://www.chrisantonellis.com:80/files/gameboy/gb-programming-manual.pdf)：Nintendo in Americaが出した（？）公式の（？）仕様書。細かい処理などを確認するときに便利です。

## src

### Rust

- [tanakh/tgbr](https://github.com/tanakh/tgbr)
- [keichi/gbr](https://github.com/keichi/gbr)

### Go

- [akatsuki105/worldwide](https://github.com/akatsuki105/worldwide)
- [mohanson/dwangb](https://github.com/akashin/dwangb)

### Ruby
- [sacckey/rubyboy](https://github.com/sacckey/rubyboy)

## test ROM

- [gitendo/helloworld](https://github.com/gitendo/helloworld)
- [Blargg’s test roms](https://gbdev.gg8.se/files/roms/blargg-gb-tests/) (テストROM): 各種資料を参考にエミュレータを実装していっても，本当に正しく実装
  できているのかわからないという問題がある．テストROMを使うことにより， エミュレータの動作を自動的に検証できる．
  テストROMは実機で通ることが確認されているため，ドキュメントよりも信頼できる． Blargg’s test
  romsには複数のROMが含まれているが，中でも命令の動作をテスト するcpu_instrsと，命令のクロック数をテストするinstr_timingは通らないと，
  実際のゲームは動かないと思う．逆に言えば，他のテストは通らなくても何とかなる．
- [tobu tobu girl](https://github.com/SimonLarsen/tobutobugirl)

## Blog

- [ゲームボーイのエミュレータを自作した話](https://keichi.dev/post/write-yourself-a-game-boy-emulator/)
- [ゲームボーイを作る（１）](https://www.tech-diningyo.info/entry/2021/07/10/222140)
- [ゲームボーイのエミュレータをGoで作った話](https://zenn.dev/akatsuki/articles/ec95ab95f0e89ea8c38f)
- [C++でゲームボーイエミュレータを自作しています](https://voidproc.com/blog/archives/664)
- [脱・初級者のための自作GBエミュレータ開発](https://www.docswell.com/s/linoscope/ZNRRXL-game-boy-emulator-ocaml)
- [OCaml でゲームボーイエミュレータを書いた話](https://qiita.com/linoscope/items/244d931aaae07df2c27e)
- [Rubyでゲームボーイのエミュレータを作った](https://zenn.dev/sacckey/articles/05b6eb6ea89662)
- [脱・初級者のための自作GBエミュレータ開発](https://www.docswell.com/s/linoscope/ZNRRXL-game-boy-emulator-ocaml)
- [AQBoy: Yet Another Game Boy Emulator 開発記](https://hackmd.io/@anqou/HJcvRrwy9)