#!/usr/bin/env python3

from __future__ import annotations

import argparse
import shutil
import stat
import subprocess
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Stage a normalized release payload for native claw installers."
    )
    parser.add_argument("--binary", required=True, help="Path to the built claw binary")
    parser.add_argument("--target", required=True, help="Target triple for the binary")
    parser.add_argument("--output", required=True, help="Output directory for the staged payload")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    binary_path = Path(args.binary).resolve()
    output_dir = Path(args.output).resolve()

    if not binary_path.is_file():
        raise SystemExit(f"binary does not exist: {binary_path}")

    stage_payload(binary_path, args.target, output_dir)
    return 0


def stage_payload(binary_path: Path, target: str, output_dir: Path) -> None:
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    binary_name = "claw.exe" if "windows" in target else "claw"
    staged_binary = output_dir / "bin" / binary_name
    copy_file(binary_path, staged_binary, executable=True)

    write_completion(
        run_completion(binary_path, "bash"),
        output_dir / "completions" / "bash" / "claw",
    )
    write_completion(
        run_completion(binary_path, "zsh"),
        output_dir / "completions" / "zsh" / "_claw",
    )
    write_completion(
        run_completion(binary_path, "powershell"),
        output_dir / "completions" / "powershell" / "claw.ps1",
    )

    repo_root = Path(__file__).resolve().parents[1]
    copy_file(repo_root / "README.md", output_dir / "docs" / "README.md", executable=False)


def run_completion(binary_path: Path, shell: str) -> str:
    result = subprocess.run(
        [str(binary_path), "completions", shell],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout


def write_completion(contents: str, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(contents, encoding="utf-8")


def copy_file(source: Path, destination: Path, *, executable: bool) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, destination)
    mode = destination.stat().st_mode
    if executable:
        destination.chmod(mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)


if __name__ == "__main__":
    raise SystemExit(main())
