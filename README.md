# GameGirl

## Reference

### Docs

- [Pandocs](https://gbdev.io/pandocs/)
- [Gameboy CPU (LR35902) instruction set](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html):
  ゲームボーイのCPU (LR35902) の全命令をまとめた表． 一覧性が高く，オペランドのフォーマットや命令のクロック数も記載されているため，
  命令デコーダの作成に重宝する．ただし，一部のクロック数は間違っているという罠がある．
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf): CPU の各命令の詳しい動作が記載されている．フラグがセット・クリアされる条件など，
  上の表よりも詳しい．ただし，一部の命令 (DAA命令など) の動作はこの資料でもまだ情報が足りないため，他の資料をあたることになる．
- [gbdev/awesome-gbdev](https://github.com/gbdev/awesome-gbdev)
- [akatsuki105/gb-docs-ja](https://github.com/akatsuki105/gb-docs-ja)
- [Game Boy Programming Manual](https://web.archive.org/web/20150513170240/http://www.chrisantonellis.com:80/files/gameboy/gb-programming-manual.pdf)
  ：Nintendo in Americaが出した（？）公式の（？）仕様書。細かい処理などを確認するときに便利です。
- [Game Boy: Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)

### src

- Rust
    - [tanakh/tgbr](https://github.com/tanakh/tgbr)
    - [keichi/gbr](https://github.com/keichi/gbr)
    - [mohanson/gameboy](https://github.com/mohanson/gameboy)
- Go
    - [akatsuki105/worldwide](https://github.com/akatsuki105/worldwide)
    - [mohanson/dwangb](https://github.com/akashin/dwangb)
- Ruby
    - [sacckey/rubyboy](https://github.com/sacckey/rubyboy)
- OCaml
    - [linoscope/CAMLBOY](https://github.com/linoscope/CAMLBOY]

### ROM

[roms](https//github.com/smirror/GameGirl/tree/main/roms)

### Blog

#### 日本語
- [ゲームボーイのエミュレータを自作した話](https://keichi.dev/post/write-yourself-a-game-boy-emulator/)
- [ゲームボーイを作る（１）](https://www.tech-diningyo.info/entry/2021/07/10/222140)
- [ゲームボーイのエミュレータをGoで作った話](https://zenn.dev/akatsuki/articles/ec95ab95f0e89ea8c38f)
- [C++でゲームボーイエミュレータを自作しています](https://voidproc.com/blog/archives/664)
- [脱・初級者のための自作GBエミュレータ開発](https://www.docswell.com/s/linoscope/ZNRRXL-game-boy-emulator-ocaml)
- [OCaml でゲームボーイエミュレータを書いた話](https://qiita.com/linoscope/items/244d931aaae07df2c27e)
- [Rubyでゲームボーイのエミュレータを作った](https://zenn.dev/sacckey/articles/05b6eb6ea89662)
- [脱・初級者のための自作GBエミュレータ開発](https://www.docswell.com/s/linoscope/ZNRRXL-game-boy-emulator-ocaml)
- [AQBoy: Yet Another Game Boy Emulator 開発記](https://hackmd.io/@anqou/HJcvRrwy9)
- [自作ゲームボーイエミュレータメモ](https://qiita.com/kmtoki/items/578e8e57ab0e76590d6d)
- [Rustでゲームボーイエミュレータを自作した話](https://mjhd.hatenablog.com/entry/2021/04/14/221813)

#### English
- [Game Boy Sound Emulation](https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html)
- [GameBoy Emulation in JavaScript](https://imrannazar.com/series/gameboy-emulation-in-javascript)

#### 中文
- [GameBoy 模拟器](http://accu.cc/content/gameboy/preface/)
  - While I don't have full fluency in reading Chinese, it appears that this blog offers valuable insights into GameBoy emulation and an intriguing history of emulator development.