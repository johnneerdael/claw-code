#!/bin/sh

set -eu

usage() {
  cat <<'EOF'
Usage: build-pkg.sh --payload <dir> --identifier <id> --version <version> --output <pkg>
EOF
}

PAYLOAD_DIR=
IDENTIFIER=
VERSION=
OUTPUT_PKG=

while [ "$#" -gt 0 ]; do
  case "$1" in
    --payload)
      PAYLOAD_DIR="$2"
      shift 2
      ;;
    --identifier)
      IDENTIFIER="$2"
      shift 2
      ;;
    --version)
      VERSION="$2"
      shift 2
      ;;
    --output)
      OUTPUT_PKG="$2"
      shift 2
      ;;
    *)
      usage >&2
      exit 1
      ;;
  esac
done

if [ -z "$PAYLOAD_DIR" ] || [ -z "$IDENTIFIER" ] || [ -z "$VERSION" ] || [ -z "$OUTPUT_PKG" ]; then
  usage >&2
  exit 1
fi

PAYLOAD_DIR="$(cd "$PAYLOAD_DIR" && pwd)"
OUTPUT_PKG_DIR="$(dirname "$OUTPUT_PKG")"
mkdir -p "$OUTPUT_PKG_DIR"
OUTPUT_PKG="$(cd "$OUTPUT_PKG_DIR" && pwd)/$(basename "$OUTPUT_PKG")"

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/claw-macos-pkg.XXXXXX")"
trap 'rm -rf "$WORK_DIR"' EXIT INT TERM

PKG_ROOT="$WORK_DIR/root"
COMPONENT_PKG="$WORK_DIR/claw-component.pkg"
mkdir -p \
  "$PKG_ROOT/usr/local/bin" \
  "$PKG_ROOT/usr/local/etc/bash_completion.d" \
  "$PKG_ROOT/usr/local/share/zsh/site-functions"

cp "$PAYLOAD_DIR/bin/claw" "$PKG_ROOT/usr/local/bin/claw"
cp "$PAYLOAD_DIR/completions/bash/claw" "$PKG_ROOT/usr/local/etc/bash_completion.d/claw"
cp "$PAYLOAD_DIR/completions/zsh/_claw" "$PKG_ROOT/usr/local/share/zsh/site-functions/_claw"
chmod 755 "$PKG_ROOT/usr/local/bin/claw"

pkgbuild \
  --root "$PKG_ROOT" \
  --identifier "$IDENTIFIER" \
  --version "$VERSION" \
  --install-location / \
  "$COMPONENT_PKG" >&2

productbuild \
  --identifier "$IDENTIFIER" \
  --version "$VERSION" \
  --package "$COMPONENT_PKG" \
  "$OUTPUT_PKG" >&2

printf '%s\n' "$OUTPUT_PKG"
