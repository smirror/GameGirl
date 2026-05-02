<!-- generated-by: gsd-doc-writer -->
# テスト

GameGirl は Cargo 標準の Rust test harness を使います。現在のテストは、カートリッジ解析と ROM-only 挙動、Bus address routing、CLI binary の振る舞いを確認します。

## テストフレームワークとセットアップ

- テストフレームワークは Cargo 標準の Rust test harness です。
- unit test は実装ファイル横の `src/cartridge.rs` と `src/bus.rs` にあります。
- CLI integration test は `tests/cli.rs` にあります。
- `Cargo.toml` に third-party test framework は設定されていません。
- global setup file は不要です。

## テスト実行

すべて実行:

```bash
cargo test
```

cartridge に絞って実行:

```bash
cargo test cartridge
```

Bus に絞って実行:

```bash
cargo test bus
```

CLI integration test だけ実行:

```bash
cargo test --test cli
```

format と lint:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

同梱 ROM を CLI で読み込み・解析できるか確認:

```bash
scripts/verify_rom_loading.sh
```

## 新しいテストを書くとき

- pure module のテストは同じ source file の `#[cfg(test)] mod tests` に置きます。
- binary としての振る舞いは `tests/` 配下の integration test に置きます。
- `rejects_rom_shorter_than_header` や `reports_missing_rom_file` のように、何を確認しているか分かる snake_case 名にします。
- 可能な限り fixture は小さく保ちます。`tests/cli.rs` は巨大な外部 fixture ではなく、テスト内で temporary ROM bytes を作ります。
- `roms/` 配下の ROM suite は、対象の emulator 機能が実装済みで、結果が意味を持つ段階で使います。

現在の例:

| 領域 | ファイル | カバー範囲 |
|------|----------|------------|
| Cartridge validation と metadata | `src/cartridge.rs` | ヘッダー長、タイトル解析、ROM/RAM size code、既知/未知 type code、固定 ROM read/write。 |
| Bus routing | `src/bus.rs` | ROM delegation、WRAM/Echo RAM mirror、OAM、I/O、HRAM、IE、未使用領域、未マップ領域。 |
| CLI behavior | `tests/cli.rs` | valid ROM load、known MBC type load、`.GBC` 拡張子、引数なし、不正拡張子、missing file、短すぎる ROM、未対応 cartridge type でも header を表示すること。 |

## Coverage 要件

coverage threshold は設定されていません。`tarpaulin`、`llvm-cov`、`grcov`、coverage 設定ファイルもありません。

## CI 連携

| Workflow | Trigger | テスト関連の挙動 |
|----------|---------|------------------|
| `.github/workflows/rust.yml` | `main` への push / pull request で Rust source、tests、scripts、workflow、Cargo metadata、同梱 ROM が変わったとき | `cargo fmt --all`、`cargo clippy`、`cargo test --verbose`、`cargo build`、`scripts/verify_rom_loading.sh` を実行します。 |
| `.github/workflows/rust-clippy.yml` | push、pull request、月次 schedule | stable Rust toolchain と clippy を入れ、SARIF 結果を upload します。 |
| `.github/workflows/dependency-review.yml` | `main` への pull request | `actions/dependency-review-action@v4` を実行します。 |

`roms/blargg-gb-tests/`、`roms/mooneye/`、`roms/hello-world/` は CI で loadability を確認します。これらの ROM に対する emulator behavior の pass/fail 判定は今後の実装対象です。
