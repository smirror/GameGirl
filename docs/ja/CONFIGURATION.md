<!-- generated-by: gsd-doc-writer -->
# 設定

GameGirl には、現時点で複雑な実行時設定はありません。主な設定は Cargo metadata、リポジトリ設定、GitHub Actions、そして CLI に渡す ROM パスです。

## 環境変数

`src/main.rs`、`src/cartridge.rs`、`src/bus.rs` はアプリケーション用の環境変数を読みません。

| 変数 | 必須 | デフォルト | 説明 |
|------|------|------------|------|
| なし | いいえ | なし | 現在の Rust コードは環境変数を参照していません。 |

## 設定ファイル

| ファイル | 形式 | 用途 |
|----------|------|------|
| `Cargo.toml` | TOML | package 名、version、edition、license、repository、readme、keywords、dependencies を定義します。 |
| `Cargo.lock` | Cargo lockfile | 依存グラフを記録します。現在、外部 Rust crate はありません。 |
| `.editorconfig` | EditorConfig | UTF-8、LF、最終改行、基本 4 spaces、YAML 2 spaces、Markdown の trailing spaces 設定を定義します。 |
| `renovate.json` | JSON | Renovate 設定です。 |
| `.github/labeler.yml` | YAML | 変更ファイルに応じた GitHub label を定義します。 |
| `.github/dependabot.yml` | YAML | GitHub Actions と Go module の週次更新チェックを定義します。 |
| `.github/auto_assign.yml` | YAML | Pull request の auto assign を設定します。 |
| `.github/workflows/rust.yml` | YAML | Rust source 変更時に format、clippy、test を実行します。 |
| `.github/workflows/rust-clippy.yml` | YAML | clippy SARIF analysis を実行します。 |
| `.github/workflows/dependency-review.yml` | YAML | Pull request で dependency review を実行します。 |

`Cargo.toml` の主要部分は次の通りです。

```toml
[package]
name = "game_girl"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "A GameBoy emulator written in Rust"

[dependencies]
```

## 必須設定と任意設定

CLI 実行時に必須なのは、最初の位置引数として渡す ROM パスです。

| 設定 | 必須 | 検証内容 | 失敗時の挙動 |
|------|------|----------|--------------|
| ROM パス引数 | はい | 最初の位置引数として存在すること。 | `Usage: {program} <rom.gb\|rom.gbc>` を表示して失敗終了します。 |
| ROM 拡張子 | はい | `.gb` または `.gbc`。大文字小文字は区別しません。 | `File must be a .gb or .gbc file` を表示して失敗終了します。 |
| ROM ファイル読み込み | はい | `std::fs::read` で読めること。 | 読み込み失敗を表示して失敗終了します。 |
| ROM ヘッダー長 | はい | 少なくとも `MIN_CARTRIDGE_HEADER_LEN` bytes あること。 | `CartridgeError::TooShort` を表示します。 |
| カートリッジ種別 | はい | `0x00` は ROM-only として受け付けます。 | その他は `CartridgeError::UnsupportedCartridgeType` を返します。 |
| ROM サイズコード | はい | `0x00` から `0x08` をサポートします。 | その他は `CartridgeError::UnsupportedRomSize` を返します。 |
| RAM サイズコード | はい | `0x00` から `0x05` をサポートします。 | その他は `CartridgeError::UnsupportedRamSize` を返します。 |

## デフォルト値

| 項目 | デフォルト | 場所 |
|------|------------|------|
| Cargo edition | Rust 2021 | `Cargo.toml` |
| Runtime dependencies | 外部 Rust dependency なし | `Cargo.toml` |
| Bus WRAM | zero-filled `[u8; 0x2000]` | `src/bus.rs` |
| Bus OAM | zero-filled `[u8; 0xA0]` | `src/bus.rs` |
| Bus I/O registers | zero-filled `[u8; 0x80]` | `src/bus.rs` |
| Bus HRAM | zero-filled `[u8; 0x7F]` | `src/bus.rs` |
| Interrupt enable byte | `0` | `src/bus.rs` |
| 未使用・未マップ領域の read | `0xFF` | `src/bus.rs` |
| ROM-only cartridge write | 無視 | `src/cartridge.rs` |

## 環境別 override

development、staging、production、test 専用のアプリケーション設定ファイルはありません。`.env`、`.env.example`、`.env.development`、`.env.production`、Docker、deployment platform 用設定もありません。

CI の挙動は `.github/workflows/` 配下の YAML で管理されています。
