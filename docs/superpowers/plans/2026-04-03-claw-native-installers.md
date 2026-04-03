# Claw Native Installers Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native macOS and Windows installers for the Rust `claw` CLI, published automatically from tagged releases, with system-wide installation, PATH setup, and shell completions.

**Architecture:** Extend the existing Rust CLI with a non-interactive completion emitter, stage a normalized release payload once per target, and feed that payload into explicit native packaging steps for macOS `.pkg` and Windows `.msi`. Use `cargo-dist` only for archive/checksum/release orchestration, while keeping installer generation under repo-owned scripts and templates.

**Tech Stack:** Rust workspace, existing custom CLI parser, GitHub Actions, cargo-dist, macOS `pkgbuild`/`productbuild`, WiX, Python 3 helper scripts, PowerShell.

---

## File Map

### Existing files to modify

- `rust/crates/rusty-claude-cli/src/main.rs`
  - Add `claw completions <shell>` parsing, dispatch, help text, and focused tests already colocated in the file.
- `rust/crates/rusty-claude-cli/Cargo.toml`
  - Add any internal module wiring only if needed. Avoid new third-party dependencies unless implementation proves they are unavoidable.
- `rust/README.md`
  - Document completion generation, local packaging commands, and release workflow usage.
- `.github/workflows/rust-ci.yml`
  - Add packaging-related smoke coverage only if it fits the existing CI contract cleanly.

### New files to create

- `rust/crates/rusty-claude-cli/src/completions.rs`
  - Emit bash, zsh, and PowerShell completion scripts without introducing a new parser stack.
- `rust/crates/rusty-claude-cli/tests/completions_cli.rs`
  - Integration tests for `claw completions <shell>`.
- `rust/scripts/stage_release_payload.py`
  - Build the normalized staging tree for packaging from one built binary plus generated completions.
- `rust/scripts/verify_release_payload.py`
  - Deterministic payload checker used locally and in CI.
- `rust/packaging/macos/build-pkg.sh`
  - Assemble the macOS filesystem tree and invoke `pkgbuild`/`productbuild`.
- `rust/packaging/macos/inspect-pkg.sh`
  - Expand and verify the `.pkg` payload paths in CI.
- `rust/packaging/windows/claw.wxs`
  - WiX source template describing files, install root, and machine PATH mutation.
- `rust/packaging/windows/build-msi.ps1`
  - Build the MSI from a staged payload.
- `rust/packaging/windows/inspect-msi.ps1`
  - Inspect MSI tables/properties from PowerShell without extra third-party tooling.
- `rust/dist-workspace.toml`
  - Minimal cargo-dist release config for archives/checksums/release publication.
- `.github/workflows/rust-release.yml`
  - Tag-triggered build/package/publish workflow.

## Constraints To Preserve

- Do not add a clap-based parser or similar large CLI dependency just to generate completions. The current CLI parser is custom, so the completion emitter should be repo-owned and explicit.
- Keep unsigned installers as the default path, but leave obvious insertion points for later signing.
- Keep system-wide paths fixed:
  - macOS: `/usr/local/bin/claw`, `/usr/local/etc/bash_completion.d/claw`, `/usr/local/share/zsh/site-functions/_claw`
  - Windows: `C:\Program Files\Claw\bin\claw.exe` plus machine PATH entry for `C:\Program Files\Claw\bin`
- Raw archives/checksums must remain available even though native installers are the primary UX.

## Task 1: Add CLI Completion Emission

**Files:**
- Create: `rust/crates/rusty-claude-cli/src/completions.rs`
- Modify: `rust/crates/rusty-claude-cli/src/main.rs`
- Test: `rust/crates/rusty-claude-cli/tests/completions_cli.rs`

- [ ] **Step 1: Write the failing integration test for supported shells**

```rust
#[test]
fn bash_completions_emit_claw_entries() {
    let output = run_claw(["completions", "bash"]);
    assert!(output.status.success());
    assert!(output.stdout.contains("claw"));
    assert!(output.stdout.contains("complete"));
}

#[test]
fn powershell_completions_reject_unknown_shells() {
    let output = run_claw(["completions", "fish"]);
    assert!(!output.status.success());
    assert!(output.stderr.contains("supported shells"));
}
```

- [ ] **Step 2: Run the new test to verify it fails**

Run: `cargo test -p rusty-claude-cli --test completions_cli -- --nocapture`
Expected: FAIL because `completions` is not a recognized command yet.

- [ ] **Step 3: Implement the minimal completion emitter**

```rust
pub enum CompletionShell {
    Bash,
    Zsh,
    PowerShell,
}

pub fn render(shell: CompletionShell) -> &'static str {
    match shell {
        CompletionShell::Bash => include_str!("completions/bash.sh"),
        CompletionShell::Zsh => include_str!("completions/zsh.sh"),
        CompletionShell::PowerShell => include_str!("completions/claw.ps1"),
    }
}
```

Implementation notes:
- Keep the implementation explicit and template-based.
- Derive the command list from the existing help/command registry where practical, but do not block on fully dynamic generation if static templating is simpler and testable.
- Add a `CliAction::Completions { shell }`-style branch in `main.rs`.
- Update `print_help_to` so `claw --help` advertises the new subcommand.

- [ ] **Step 4: Run the completion tests and targeted CLI tests**

Run: `cargo test -p rusty-claude-cli --test completions_cli`
Expected: PASS

Run: `cargo test -p rusty-claude-cli parses_login_and_logout_subcommands parses_prompt_subcommand init_help_mentions_direct_subcommand`
Expected: PASS and no regression in top-level subcommand parsing/help text.

- [ ] **Step 5: Commit**

```bash
git add rust/crates/rusty-claude-cli/src/main.rs \
  rust/crates/rusty-claude-cli/src/completions.rs \
  rust/crates/rusty-claude-cli/tests/completions_cli.rs
git commit -m "Expose shell completion scripts from the claw CLI"
```

## Task 2: Add Shared Release Payload Staging

**Files:**
- Create: `rust/scripts/stage_release_payload.py`
- Create: `rust/scripts/verify_release_payload.py`
- Test: `rust/crates/rusty-claude-cli/tests/release_payload_smoke.rs`

- [ ] **Step 1: Write the failing payload smoke test**

```rust
#[test]
fn staging_script_writes_binary_and_completion_assets() {
    let temp = tempdir().unwrap();
    let status = Command::new("python3")
        .args([
            "scripts/stage_release_payload.py",
            "--binary", temp_bin,
            "--target", "x86_64-apple-darwin",
            "--output", temp_out,
        ])
        .current_dir(workspace_root())
        .status()
        .unwrap();
    assert!(status.success());
    assert!(temp_out.join("bin/claw").exists());
    assert!(temp_out.join("completions/bash/claw").exists());
}
```

- [ ] **Step 2: Run the smoke test to verify it fails**

Run: `cargo test -p rusty-claude-cli --test release_payload_smoke -- --nocapture`
Expected: FAIL because the staging script does not exist.

- [ ] **Step 3: Implement the staging and verification scripts**

```python
def stage_payload(binary_path: Path, target: str, out_dir: Path) -> None:
    copy_binary(binary_path, out_dir / "bin")
    write_completion(run_completion(binary_path, "bash"), out_dir / "completions" / "bash" / "claw")
    write_completion(run_completion(binary_path, "zsh"), out_dir / "completions" / "zsh" / "_claw")
    write_completion(run_completion(binary_path, "powershell"), out_dir / "completions" / "powershell" / "claw.ps1")
    copy_docs(out_dir)
```

Implementation notes:
- The staging script should invoke the built binary for completions instead of duplicating completion generation logic.
- Keep output layout normalized across platforms:
  - `bin/`
  - `completions/bash/`
  - `completions/zsh/`
  - `completions/powershell/`
  - `docs/`
- `verify_release_payload.py` should fail with precise messages when required files are missing.

- [ ] **Step 4: Run the smoke and verification tests**

Run: `cargo test -p rusty-claude-cli --test release_payload_smoke`
Expected: PASS

Run: `python3 rust/scripts/verify_release_payload.py --payload /tmp/example-payload --target x86_64-apple-darwin`
Expected: PASS for a valid payload, FAIL with clear path-specific errors for invalid payloads.

- [ ] **Step 5: Commit**

```bash
git add rust/scripts/stage_release_payload.py \
  rust/scripts/verify_release_payload.py \
  rust/crates/rusty-claude-cli/tests/release_payload_smoke.rs
git commit -m "Stage a normalized release payload for native installers"
```

## Task 3: Package And Inspect The macOS Installer

**Files:**
- Create: `rust/packaging/macos/build-pkg.sh`
- Create: `rust/packaging/macos/inspect-pkg.sh`
- Modify: `.github/workflows/rust-release.yml`

- [ ] **Step 1: Write the failing macOS packaging inspection path in CI or local smoke script**

```bash
./rust/packaging/macos/build-pkg.sh \
  --payload /tmp/claw-release/payload \
  --identifier dev.instructkr.claw \
  --version 0.1.0 \
  --output /tmp/claw-release/claw.pkg
```

Expected initial result: FAIL because the packaging script does not exist.

- [ ] **Step 2: Implement the package builder**

```bash
pkgbuild \
  --root "$PKG_ROOT" \
  --identifier "$IDENTIFIER" \
  --version "$VERSION" \
  --install-location / \
  "$WORK_DIR/claw-component.pkg"

productbuild \
  --package "$WORK_DIR/claw-component.pkg" \
  "$OUTPUT_PKG"
```

Implementation notes:
- Build the package root from the normalized payload, mapping:
  - `bin/claw` -> `/usr/local/bin/claw`
  - `completions/bash/claw` -> `/usr/local/etc/bash_completion.d/claw`
  - `completions/zsh/_claw` -> `/usr/local/share/zsh/site-functions/_claw`
- Keep the script parameterized by version and output path.

- [ ] **Step 3: Implement package inspection**

```bash
pkgutil --expand-full "$PKG_PATH" "$EXPANDED_DIR"
test -f "$EXPANDED_DIR/Payload/usr/local/bin/claw"
test -f "$EXPANDED_DIR/Payload/usr/local/etc/bash_completion.d/claw"
test -f "$EXPANDED_DIR/Payload/usr/local/share/zsh/site-functions/_claw"
```

- [ ] **Step 4: Run macOS packaging smoke verification**

Run on macOS: `./rust/packaging/macos/build-pkg.sh ... && ./rust/packaging/macos/inspect-pkg.sh /tmp/claw-release/claw.pkg`
Expected: PASS and exact destination paths found in the expanded payload.

- [ ] **Step 5: Commit**

```bash
git add rust/packaging/macos/build-pkg.sh \
  rust/packaging/macos/inspect-pkg.sh
git commit -m "Package the claw CLI as a native macOS pkg"
```

## Task 4: Package And Inspect The Windows Installer

**Files:**
- Create: `rust/packaging/windows/claw.wxs`
- Create: `rust/packaging/windows/build-msi.ps1`
- Create: `rust/packaging/windows/inspect-msi.ps1`
- Modify: `.github/workflows/rust-release.yml`

- [ ] **Step 1: Write the failing Windows packaging invocation**

```powershell
pwsh -File rust/packaging/windows/build-msi.ps1 `
  -PayloadDir C:\temp\claw\payload `
  -Version 0.1.0 `
  -OutFile C:\temp\claw\claw.msi
```

Expected initial result: FAIL because the packaging assets do not exist.

- [ ] **Step 2: Implement the WiX template and build script**

```xml
<Directory Id="ProgramFiles64Folder">
  <Directory Id="INSTALLROOT" Name="Claw">
    <Directory Id="BIN_DIR" Name="bin">
      <Component Id="ClawBinary" Guid="*">
        <File Id="ClawExe" Source="$(var.PayloadDir)\bin\claw.exe" KeyPath="yes" />
      </Component>
    </Directory>
  </Directory>
</Directory>
```

Implementation notes:
- Include a WiX `Environment` entry to append `C:\Program Files\Claw\bin` to the machine PATH.
- Install the PowerShell completion file to `C:\Program Files\Claw\completions\claw.ps1`.
- Use candle/light or WiX v4 equivalent consistently in the script.

- [ ] **Step 3: Implement MSI inspection**

```powershell
$installer = New-Object -ComObject WindowsInstaller.Installer
$database = $installer.GetType().InvokeMember("OpenDatabase", "InvokeMethod", $null, $installer, @($msiPath, 0))
# Query File, Directory, and Environment tables
```

Inspection assertions:
- file table includes `claw.exe`
- directory table includes `ProgramFilesFolder`/`Claw`/`bin`
- environment table includes machine PATH mutation for the bin directory

- [ ] **Step 4: Run Windows packaging smoke verification**

Run on Windows: `pwsh -File rust/packaging/windows/build-msi.ps1 ...`
Expected: MSI created

Run on Windows: `pwsh -File rust/packaging/windows/inspect-msi.ps1 -MsiPath C:\temp\claw\claw.msi`
Expected: PASS and explicit confirmation of file placement and PATH mutation.

- [ ] **Step 5: Commit**

```bash
git add rust/packaging/windows/claw.wxs \
  rust/packaging/windows/build-msi.ps1 \
  rust/packaging/windows/inspect-msi.ps1
git commit -m "Package the claw CLI as a native Windows MSI"
```

## Task 5: Wire cargo-dist, Release CI, And Docs

**Files:**
- Create: `rust/dist-workspace.toml`
- Create: `.github/workflows/rust-release.yml`
- Modify: `.github/workflows/rust-ci.yml`
- Modify: `rust/README.md`

- [ ] **Step 1: Write the failing release workflow expectations**

Create a checklist in the PR or local notes and validate these conditions are currently unmet:
- no `rust-release.yml`
- no `dist-workspace.toml`
- no tagged-release artifact publication path

Expected initial result: all unmet.

- [ ] **Step 2: Add minimal cargo-dist configuration**

```toml
[dist]
cargo-dist-version = "..."
ci = ["github"]
targets = [
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-pc-windows-msvc",
]
installers = []
```

Implementation notes:
- Keep `installers = []` or the equivalent archive-only posture so native installers remain repo-owned.
- Use cargo-dist for archives/checksums/release publication rather than for `.msi` generation.

- [ ] **Step 3: Add the tagged release workflow**

```yaml
on:
  push:
    tags:
      - "v*"

jobs:
  release-macos:
  release-windows:
  publish:
```

Workflow responsibilities:
- run workspace tests before packaging
- build target binaries
- stage payloads
- build `.pkg` and `.msi`
- inspect produced installers
- upload native installers plus raw archives/checksums to the GitHub Release

- [ ] **Step 4: Document local release operations**

Add to `rust/README.md`:
- how to build completions locally
- how to build a local payload
- how to build a local macOS `.pkg`
- how to build a local Windows `.msi`
- unsigned installer caveats

- [ ] **Step 5: Run final verification**

Run: `cargo test --workspace`
Expected: PASS

Run: local payload verification script against a staged payload
Expected: PASS

Run in CI on tag or via workflow dispatch dry run where possible
Expected: native installers plus raw archives are produced for supported targets.

- [ ] **Step 6: Commit**

```bash
git add rust/dist-workspace.toml \
  .github/workflows/rust-release.yml \
  .github/workflows/rust-ci.yml \
  rust/README.md
git commit -m "Automate native claw installer releases from tags"
```

## Final Integration Check

- [ ] Confirm `cargo test --workspace` passes after all tasks.
- [ ] Confirm `claw completions bash|zsh|powershell` works from a release build.
- [ ] Confirm macOS package inspection finds `/usr/local/bin/claw` and the completion destinations.
- [ ] Confirm Windows MSI inspection finds `claw.exe` and the machine PATH mutation entry.
- [ ] Confirm the release workflow uploads `.pkg`, `.msi`, and raw archives/checksums.

## Notes For The Implementer

- Prefer repo-owned templates and scripts over clever generator abstractions.
- Keep each task diff small and reversible.
- Do not add signing yet; only leave clean hooks for later.
- If `cargo-dist` configuration constraints force a different file location or format, preserve the same architecture and update docs/tests in the same task.
