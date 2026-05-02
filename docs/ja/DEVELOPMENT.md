<!-- generated-by: gsd-doc-writer -->
# 開発

GameGirl は Rust 2021 の Cargo project です。現在は、小さくテストしやすい emulator core module と薄い CLI wrapper を中心に開発しています。

## ローカルセットアップ

1. リポジトリを fork または clone します。
2. crate を build します。

   ```bash
   cargo build
   ```

3. 変更前後にテストを実行します。

   ```bash
   cargo test
   ```

4. Pull request 前に format と lint を確認します。

   ```bash
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   ```

`.env`、local service、database、generated source step は不要です。

## 開発コマンド

| コマンド | 説明 |
|----------|------|
| `cargo build` | binary crate と library crate を compile します。 |
| `cargo run -- roms/hello-world/hello-world.gb` | 同梱の hello-world ROM fixture で CLI を実行します。 |
| `cargo test` | unit test、integration test、doc test を実行します。 |
| `cargo test cartridge` | 名前または module path に `cartridge` を含むテストを実行します。 |
| `cargo test bus` | 名前または module path に `bus` を含むテストを実行します。 |
| `cargo test --test cli` | `tests/cli.rs` の CLI integration test を実行します。 |
| `cargo fmt --all` | rustfmt で Rust code を整形します。 |
| `cargo fmt --all -- --check` | ファイルを書き換えずに format 差分を確認します。 |
| `cargo clippy --all-targets -- -D warnings` | 全 target に clippy を実行し、warning を error として扱います。 |

## コードスタイル

- Rust code は rustfmt 標準に従います。`.editorconfig` が editor の基本設定を持ち、`.github/workflows/rust.yml` は `cargo fmt --all` を実行します。
- lint には clippy を使います。`.github/workflows/rust.yml` は `cargo clippy` を実行し、`.github/workflows/rust-clippy.yml` は SARIF analysis を実行します。
- CLI の引数処理や表示は `src/main.rs` に置き、再利用可能な emulator logic は `src/lib.rs` から公開する module に置きます。
- emulator の純粋な処理には小さい unit test を、binary としての振る舞いには integration test を優先します。
- `Cargo.toml` の `[dependencies]` は現在空です。意味のあるリスク削減や複雑さ削減がない限り、依存追加は避けます。

## ブランチ規約

明文化された branch naming rule はまだありません。`feature/add-cpu` や `fix/rom-loading-error` のように、作業内容が分かる短い名前を使います。

## PR 前の確認

- Pull request は 1 つの subsystem、挙動、またはドキュメント topic に絞ります。
- review 前に `cargo fmt --all`、`cargo clippy --all-targets -- -D warnings`、`cargo test` を実行します。
- cartridge、bus、CPU、timer、PPU、CLI の挙動を変える場合はテストを追加・更新します。
- 検証に使った ROM fixture や focused test を説明します。
- `.github/workflows/rust.yml`、`.github/workflows/rust-clippy.yml`、`.github/workflows/dependency-review.yml` の CI 結果を確認します。
