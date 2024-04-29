# test ROMs

This dir contains test ROMs for GameBoy (Color) emulator.
You can use these ROMs to test your emulator or check original src code to build.


### test ROM

- ~~[gitendo/helloworld](https://github.com/gitendo/helloworld)~~ (not worked & built)
- [Blargg’s test roms](https://gbdev.gg8.se/files/roms/blargg-gb-tests/) (テストROM): 各種資料を参考にエミュレータを実装していっても，本当に正しく実装
  できているのかわからないという問題がある．テストROMを使うことにより， エミュレータの動作を自動的に検証できる．
  テストROMは実機で通ることが確認されているため，ドキュメントよりも信頼できる． Blargg’s test
  romsには複数のROMが含まれているが，中でも命令の動作をテスト するcpu_instrsと，命令のクロック数をテストするinstr_timingは通らないと，
  実際のゲームは動かないと思う．逆に言えば，他のテストは通らなくても何とかなる．
- [Mooneye Test Suite](https://github.com/Gekkio/mooneye-test-suite)
    - build files: https://gekkio.fi/files/mooneye-test-suite/
- [hello world](https://github.com/gbdev/gb-asm-tutorial)

### Game ROM

- [tobu tobu girl](https://github.com/SimonLarsen/tobutobugirl)
- 
