#!/bin/sh

set -eu

if [ "$#" -ne 1 ]; then
  echo "Usage: inspect-pkg.sh <pkg-path>" >&2
  exit 1
fi

PKG_PATH="$1"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/claw-macos-inspect.XXXXXX")"
trap 'rm -rf "$WORK_DIR"' EXIT INT TERM

PRODUCT_DIR="$WORK_DIR/product"
mkdir -p "$PRODUCT_DIR"
(cd "$PRODUCT_DIR" && xar -xf "$PKG_PATH")

COMPONENT_PKG="$(find "$PRODUCT_DIR" -name '*.pkg' -type d | head -n 1)"
if [ -z "$COMPONENT_PKG" ]; then
  echo "No component package found inside $PKG_PATH" >&2
  exit 1
fi

PAYLOAD_LIST="$WORK_DIR/payload.txt"
gzip -dc "$COMPONENT_PKG/Payload" | cpio -it >"$PAYLOAD_LIST" 2>/dev/null

grep -Fx './usr/local/bin/claw' "$PAYLOAD_LIST" >/dev/null
grep -Fx './usr/local/etc/bash_completion.d/claw' "$PAYLOAD_LIST" >/dev/null
grep -Fx './usr/local/share/zsh/site-functions/_claw' "$PAYLOAD_LIST" >/dev/null

cat "$PAYLOAD_LIST"
