<!-- generated-by: gsd-doc-writer -->
# コントリビューション

GameGirl への改善ありがとうございます。プロジェクトはまだ初期段階なので、広い subsystem を一気に変えるより、小さくテストされた変更のほうが review しやすいです。

## 開発セットアップ

前提条件と最初の実行は `docs/ja/GETTING-STARTED.md` を参照してください。ローカル開発コマンド、style、Pull request 前の準備は `docs/ja/DEVELOPMENT.md` を参照してください。

## コーディング標準

- Rust 2021 と Cargo を使います。
- Rust code は `cargo fmt --all` で format します。
- Pull request 前に `cargo clippy --all-targets -- -D warnings` を実行します。
- 再利用可能な emulator logic は `src/` 配下の library module に置き、CLI 引数処理は `src/main.rs` に置きます。
- 新しい emulator 挙動にはテストを追加します。純粋な module logic は unit test、binary behavior は `tests/` 配下の integration test を優先します。

## PR ガイドライン

- Pull request は 1 つの挙動、subsystem、または documentation topic に絞ります。
- 何をなぜ変えたか説明します。
- 実行したコマンドを記載します。特に `cargo test`、`cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`。
- 挙動を変えた場合は、テストを追加または更新します。
- 明確なリスク削減や複雑さ削減がない限り、third-party dependency は追加しません。
- emulator behavior の変更では、検証に使った ROM fixture や hardware reference を書きます。

## Issue 報告

bug や feature request は GitHub Issues を使います。

bug の場合は以下を含めてください。

- 再現手順。
- 期待する挙動。
- 実際の挙動。
- 使った ROM path または fixture。
- CLI や test command の出力。
- Rust toolchain version。

feature request の場合は以下を含めてください。

- 関係する emulator subsystem。
- ユーザーから見える挙動、または compatibility goal。
- 期待挙動を明確にする test ROM、reference、既存実装。
