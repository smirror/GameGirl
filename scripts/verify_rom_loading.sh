#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BINARY="${GAME_GIRL_BIN:-$ROOT_DIR/target/debug/game_girl}"

if [[ ! -x "$BINARY" ]]; then
    echo "Building game_girl..."
    cargo build
fi

if [[ ! -x "$BINARY" ]]; then
    echo "GameGirl binary is not executable: $BINARY"
    exit 1
fi

stdout_file="$(mktemp "${TMPDIR:-/tmp}/gamegirl-rom-stdout.XXXXXX")"
stderr_file="$(mktemp "${TMPDIR:-/tmp}/gamegirl-rom-stderr.XXXXXX")"
trap 'rm -f "$stdout_file" "$stderr_file"' EXIT

failed=0
total=0

while IFS= read -r rom; do
    total=$((total + 1))
    echo "Verifying $rom..."

    if [[ "${VERBOSE:-0}" == "1" ]]; then
        if ! "$BINARY" "$rom"; then
            echo "Error: Failed to load $rom"
            failed=$((failed + 1))
        fi
        continue
    fi

    if ! "$BINARY" "$rom" >"$stdout_file" 2>"$stderr_file"; then
        cat "$stdout_file"
        cat "$stderr_file" >&2
        echo "Error: Failed to load $rom"
        failed=$((failed + 1))
    fi
done < <(find roms -type f \( -name "*.gb" -o -name "*.gbc" \) -print | sort)

if [[ "$total" -eq 0 ]]; then
    echo "No ROMs found under roms/"
    exit 1
fi

if [[ "$failed" -ne 0 ]]; then
    echo "$failed ROMs failed to load"
    exit 1
fi

echo "Verified $total ROMs"
