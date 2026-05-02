# External Integrations

**Analysis Date:** 2026-05-02

## APIs & External Services

**Runtime Services:**
- None. `src/main.rs` performs local argument parsing and local file reads only.

**External APIs:**
- None used by the application code.

## Data Storage

**Databases:**
- None.

**File Storage:**
- Local filesystem only.
  - `src/main.rs` expects a local `.gb` or `.gbc` path as its first argument.
  - Current implementation reads the file with `fs::read_to_string`, which is not suitable for binary ROM data.

**Caching:**
- None.

## Authentication & Identity

**Auth Provider:**
- None.

**OAuth Integrations:**
- None.

## Monitoring & Observability

**Error Tracking:**
- None.

**Analytics:**
- None.

**Logs:**
- Console output only through `println!` and `eprintln!` in `src/main.rs`.

## CI/CD & Repository Automation

**Hosting:**
- GitHub repository metadata is configured in `Cargo.toml` as `https://github.com/smirror/GameGirl`.

**CI Pipeline:**
- GitHub Actions:
  - `.github/workflows/rust.yml` runs formatting, clippy, and tests for Rust source changes.
  - `.github/workflows/rust-clippy.yml` runs scheduled and PR/push clippy SARIF analysis.
  - `.github/workflows/dependency-review.yml` runs dependency review on pull requests.
  - `.github/workflows/label.yml` labels pull requests using `.github/labeler.yml`.
  - `.github/workflows/assign.yml` adds reviewers using `.github/auto_assign.yml`.
  - `.github/workflows/greetings.yml` posts first interaction messages.
- Dependabot:
  - `.github/dependabot.yml` tracks GitHub Actions weekly.
  - It also configures a Go module ecosystem at `/`, although this repository currently has no `go.mod`.

## Environment Configuration

**Development:**
- Required env vars: none.
- Secrets location: none required by application code.
- Test fixtures: ROMs are checked into `roms/`.

**Staging:**
- Not defined.

**Production:**
- Not defined.

## Webhooks & Callbacks

**Incoming:**
- None.

**Outgoing:**
- None.

---

*Integration audit: 2026-05-02*
*Update when adding/removing external services*
