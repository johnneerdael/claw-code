#!/usr/bin/env python3

from __future__ import annotations

import argparse
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify a staged claw release payload contains required assets."
    )
    parser.add_argument("--payload", required=True, help="Payload directory to verify")
    parser.add_argument("--target", required=True, help="Target triple for the payload")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    payload_dir = Path(args.payload).resolve()

    missing = required_paths(payload_dir, args.target)
    missing = [path for path in missing if not path.is_file()]
    if missing:
        missing_lines = "\n".join(f"- missing required file: {path}" for path in missing)
        raise SystemExit(f"release payload verification failed\n{missing_lines}")

    print(f"release payload verified for {args.target}")
    return 0


def required_paths(payload_dir: Path, target: str) -> list[Path]:
    binary_name = "claw.exe" if "windows" in target else "claw"
    return [
        payload_dir / "bin" / binary_name,
        payload_dir / "completions" / "bash" / "claw",
        payload_dir / "completions" / "zsh" / "_claw",
        payload_dir / "completions" / "powershell" / "claw.ps1",
        payload_dir / "docs" / "README.md",
    ]


if __name__ == "__main__":
    raise SystemExit(main())
