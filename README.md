# GameGirl

## Reference

### Docs

- [Pandocs](http://gbdev.gg8.se/wiki/articles/Pan_Docs)
    - [Original](http://marc.rawer.de/Gameboy/Docs) (but not updated)
- [Gameboy CPU (LR35902) instruction set](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html):
  ゲームボーイのCPU (LR35902) の全命令をまとめた表． 一覧性が高く，オペランドのフォーマットや命令のクロック数も記載されているため，
  命令デコーダの作成に重宝する．ただし，一部のクロック数は間違っているという 罠がある．
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf): CPU の各命令の詳しい動作が記載されている．フラグがセット・クリアされる条件など，
  上の表よりも詳し い．ただし，一部の命令 (DAA命令など) の動作はこの資料でも まだ情報が足りないため，他の資料をあたることになる．
- [gbdev/awesome-gbdev](https://github.com/gbdev/awesome-gbdev)
- [akatsuki105/gb-docs-ja](https://github.com/akatsuki105/gb-docs-ja)

## src

### Rust

- [tanakh/tgbr](https://github.com/tanakh/tgbr)
- [keichi/gbr](https://github.com/keichi/gbr)

### Go

- [akatsuki105/worldwide](https://github.com/akatsuki105/worldwide)
- [mohanson/dwangb](https://github.com/akashin/dwangb)

## test ROM

- [gitendo/helloworld](https://github.com/gitendo/helloworld)
- [Blargg’s test roms](https://gbdev.gg8.se/files/roms/blargg-gb-tests/) (テストROM): 各種資料を参考にエミュレータを実装していっても，本当に正しく実装
  できているのかわからないという問題がある．テストROMを使うことにより， エミュレータの動作を自動的に検証できる．
  テストROMは実機で通ることが確認されているため，ドキュメントよりも信頼できる． Blargg’s test
  romsには複数のROMが含まれているが，中でも命令の動作をテスト するcpu_instrsと，命令のクロック数をテストするinstr_timingは通らないと，
  実際のゲームは動かないと思う．逆に言えば，他のテストは通らなくても何とかなる．

## Blog

- [ゲームボーイのエミュレータを自作した話](https://keichi.dev/post/write-yourself-a-game-boy-emulator/)
