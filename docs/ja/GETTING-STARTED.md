<!-- generated-by: gsd-doc-writer -->
# Getting Started

このガイドでは、GameGirl をローカルで build し、テストを実行し、現在の CLI で ROM を読み込むところまで進めます。

## 前提条件

- Rust と Cargo が必要です。
- このリポジトリは厳密な Rust version を pin していません。`Cargo.toml` に `rust-version` はなく、toolchain pin ファイルもありません。
- `.github/workflows/rust-clippy.yml` では stable Rust toolchain を入れて clippy analysis を実行します。
- database、外部サービス、emulator frontend、環境変数設定は不要です。

## インストール手順

1. リポジトリを clone します。

   ```bash
   git clone https://github.com/smirror/GameGirl.git
   ```

2. プロジェクトディレクトリへ移動します。

   ```bash
   cd GameGirl
   ```

3. Rust crate を build します。

   ```bash
   cargo build
   ```

4. テストを実行します。

   ```bash
   cargo test
   ```

## 最初の実行

同梱の hello-world ROM fixture を使います。

```bash
cargo run -- roms/hello-world/hello-world.gb
```

現在の CLI は、読み込んだ ROM の byte 数とヘッダー情報を表示します。

```text
Loaded ROM: 32768 bytes
Header:
  title:
  cartridge_type: RomOnly (0x00)
  rom_size: 32768 bytes (code 0x00)
  ram_size: 0 bytes (code 0x00)
  cgb_flag: 0x00
  header_checksum: 0xE7
  global_checksum: 0x021B
  entry_point: C3 50 01 00
```

## よくあるセットアップ問題

| 症状 | 原因 | 対処 |
|------|------|------|
| `Usage: ... <rom.gb\|rom.gbc>` | ROM パスを渡していません。 | `--` の後に `.gb` または `.gbc` ファイルを渡します。 |
| `File must be a .gb or .gbc file` | 拡張子が未対応です。 | `.gb` / `.gbc` のファイルを使います。`.GBC` のような大文字拡張子も受け付けます。 |
| `could not read ROM ...` | ファイルが存在しない、または読めません。 | パスと権限を確認します。 |
| `ROM is too short ... expected at least 336 bytes` | ファイルがカートリッジヘッダー領域より短いです。 | 実 ROM または `0x150` bytes 以上の fixture を使います。 |
| `unsupported cartridge type ...` | ROM が未対応のカートリッジ種別です。 | 現時点では ROM-only カートリッジを使います。 |

## 次に読むもの

- `docs/ja/ARCHITECTURE.md`: モジュール構成とデータフロー。
- `docs/ja/DEVELOPMENT.md`: 開発コマンドと作業前後の確認。
- `docs/ja/TESTING.md`: テストコマンドと CI。
- `docs/ja/CONFIGURATION.md`: 実行時入力と設定ファイル。
