# Game Boy 仕様まとめ

`docs/hot_to_proceed.md` の「1. 先に読む資料」を実際に読んだうえで、
DMG 向けエミュレータ実装に必要な仕様を絞って整理したメモです。

## 1. 前提

- 実装の初期ターゲットは DMG 互換に絞るのが妥当です。
- Game Boy は Sharp SM83 系の 8-bit CPU を中心に、RAM、PPU、Timer、Joypad などを SoC にまとめた構成です。
- アドレス空間は 16-bit、標準クロックは 4.194304 MHz です。
- 通常速度では 1 M-cycle = 4 T-cycle = 4 dots と考えて実装すると整理しやすいです。

## 2. メモリマップ

DMG 実装でまず押さえるべき領域は次の通りです。

| 範囲 | 用途 | 実装メモ |
| --- | --- | --- |
| `0000-3FFF` | ROM bank 0 | 固定バンク |
| `4000-7FFF` | switchable ROM | MBC 対応時に切り替え |
| `8000-9FFF` | VRAM | PPU 使用中は CPU から制約あり |
| `A000-BFFF` | External RAM | MBC ごとに有効化やバンク切り替えあり |
| `C000-DFFF` | WRAM | 本体 RAM |
| `E000-FDFF` | Echo RAM | `C000-DDFF` のミラー。基本は使わない前提でよい |
| `FE00-FE9F` | OAM | Sprite 属性 |
| `FEA0-FEFF` | unusable | 禁止領域として扱うのが安全 |
| `FF00-FF7F` | I/O registers | Joypad, Timer, LCD, DMA, Audio など |
| `FF80-FFFE` | HRAM | 高速ワークエリア |
| `FFFF` | `IE` | 割り込み有効化レジスタ |

補足:

- `0000-00FF` には `RST` ベクタと割り込みベクタが置かれます。
- 割り込みベクタは `0040`, `0048`, `0050`, `0058`, `0060` です。
- `0100-014F` はカートリッジヘッダです。エントリポイントや MBC 種別を読む必要があります。

## 3. CPU

### 3.1. レジスタ

- 16-bit レジスタは `AF`, `BC`, `DE`, `HL`, `SP`, `PC` です。
- `AF` の下位 8-bit はフラグで、意味があるのは `Z`, `N`, `H`, `C` の 4 bit だけです。

### 3.2. フラグ

- `Z`: 結果が 0 のときに立つ
- `N`: 直前が減算系なら立つ
- `H`: 下位 nibble の桁上がり/桁借りに使う
- `C`: 桁上がり/桁借り、rotate/shift の繰り出しに使う

`DAA` は `N/H/C` の扱いを間違えると破綻しやすいので、初期段階から特別扱い前提で実装した方がよいです。

### 3.3. 命令セットの見方

- CPU は可変長命令です。
- 命令デコードは「通常 opcode」と `CB` prefix の 2 段で考えると整理しやすいです。
- Z80 に似ていますが別物なので、Z80 と同一視しない方が安全です。
- オペコード表は実装時の引き当て表として有用ですが、細かな挙動は `Pan Docs` や実機準拠テストで裏取りする必要があります。

### 3.4. DMG 起動直後の代表値

DMG の初期値として最低限固定で持ってよい値:

- `PC = 0x0100`
- `SP = 0xFFFE`
- `A = 0x01`
- `B = 0x00`
- `C = 0x13`
- `D = 0x00`
- `E = 0xD8`
- `H = 0x01`
- `L = 0x4D`
- `IE = 0x00`

注意:

- `F` の一部ビットや一部 I/O レジスタは世代や起動経路で差が出るため、厳密再現が必要になるまでは DMG 前提で揃えるのが現実的です。

## 4. 割り込み

### 4.1. 基本構造

- `IME` は CPU 内部フラグで、割り込み全体の許可/禁止を制御します。
- `IE` は `FFFF`、`IF` は `FF0F` にあります。
- 割り込み優先度は `VBlank > LCD > Timer > Serial > Joypad` です。

### 4.2. 実装上の重要点

- ゲーム開始時点では `IME = 0` です。
- `ei` の効果は即時ではなく、1 命令遅れて反映されます。
- 割り込み受理時は、CPU が `IF` 対象ビットを下ろし、`IME` を 0 にし、現在の `PC` を push してベクタへ飛びます。
- 割り込み遷移全体は 5 M-cycle です。

この部分は `HALT` 周りの不具合やタイミングと密結合なので、後で `mooneye` でも検証する前提で実装した方がよいです。

## 5. Timer

### 5.1. レジスタ

- `DIV (FF04)`
- `TIMA (FF05)`
- `TMA (FF06)`
- `TAC (FF07)`

### 5.2. 実装の考え方

- `DIV` は単なる独立カウンタではなく、内部 system counter の一部を見せていると考えると実装しやすいです。
- `TAC` はその内部カウンタの特定 bit を選び、その falling edge で `TIMA` を進めます。
- したがって、`DIV` 書き込みで counter をリセットしたときや、`TAC` の周波数選択を変えたときに即座に 1 tick 発生するケースがあります。

### 5.3. 見落としやすい挙動

- `TIMA` overflow 時は、その場でただちに `TMA` が入るのではなく、1 M-cycle 後に reload と `IF.Timer` 設定が起きます。
- overflow 直後の 1 M-cycle は `TIMA == 0x00` になります。
- この 1 M-cycle の隙間で `TIMA` / `TMA` へ書いたときの挙動は特殊です。

Timer は後から直すと CPU テスト全体に波及するので、最初から「内部 counter ベース」で実装するのが無難です。

## 6. PPU

### 6.1. フレーム構造

- 1 frame は 154 scanlines
- 可視行は最初の 144 lines
- 1 frame は 70224 dots
- 垂直同期は約 59.7 Hz
- 解像度は `160x144`

### 6.2. PPU mode

| Mode | 内容 | 長さ |
| --- | --- | --- |
| `2` | OAM scan | 80 dots |
| `3` | drawing | 172-289 dots |
| `0` | HBlank | `376 - mode3` dots |
| `1` | VBlank | 4560 dots |

### 6.3. CPU からのアクセス制約

- VRAM は Mode `0-2` でアクセス可能
- OAM は基本的に Mode `0-1` でアクセス可能
- Mode `3` 中は VRAM/OAM とも CPU から触れない前提でよいです
- アクセス不可時の write は無視、read は不定値扱いです

この制約があるため、PPU 未実装の段階でも「今の mode に応じてメモリアクセスを拒否する」形だけ先に作っておく価値があります。

### 6.4. Sprite の最低限仕様

- OBJ は `8x8` または `8x16`
- 同時に保持できる OBJ は最大 40
- 1 scanline に描画できる OBJ は最大 10

## 7. 実装時の優先順位

仕様を読んだうえで、最初の実装優先順位は次でよいです。

1. DMG のメモリマップを固定値で作る
2. CPU レジスタ、命令 fetch/decode/execute を作る
3. `IE` / `IF` / `IME` と timer をつなぐ
4. PPU の mode 遷移だけ先に作る
5. VRAM/OAM アクセス制約を反映する
6. その後に背景描画、sprite 描画、MBC へ進む

## 8. ソースの使い分け

- `Pan Docs`
  - まず最優先で見る基準資料。メモリマップ、割り込み、PPU、タイマの仕様確認に使う。
- `Game Boy: Complete Technical Reference`
  - クロック、CPU コアの立ち位置、世代差、電気的な背景を確認したいときに使う。
- `Gameboy CPU (LR35902) instruction set`
  - opcode の一覧表として使う。命令の網羅確認に便利。
- `Game Boy CPU Manual`
  - 命令ごとの動作確認の補助に使う。ただし最終判断は `Pan Docs` とテスト ROM を優先する。

## 9. 参考元

- [Pan Docs: Memory Map](https://gbdev.io/pandocs/Memory_Map.html)
- [Pan Docs: CPU Registers and Flags](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
- [Pan Docs: CPU Instruction Set](https://gbdev.io/pandocs/CPU_Instruction_Set.html)
- [Pan Docs: Interrupts](https://gbdev.io/pandocs/Interrupts.html)
- [Pan Docs: Timer obscure behaviour](https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html)
- [Pan Docs: Accessing VRAM and OAM](https://gbdev.io/pandocs/Accessing_VRAM_and_OAM.html)
- [Pan Docs: Rendering overview](https://gbdev.io/pandocs/Rendering.html)
- [Pan Docs: Power-Up Sequence](https://gbdev.io/pandocs/Power_Up_Sequence.html)
- [Pan Docs: Specifications](https://gbdev.io/pandocs/Specifications.html)
- [Game Boy: Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)
- [Gameboy CPU (LR35902) instruction set](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
