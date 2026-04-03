# Claw Native Installer Design

Date: 2026-04-03
Status: Draft for review
Scope: Rust CLI packaging and release automation

## Goal

Ship native installable artifacts for the Rust `claw` CLI on macOS and Windows, published automatically from tagged releases, with system-wide installation and `PATH` availability after install.

## Non-goals

- Linux native packages in this iteration
- Code signing in this iteration
- Full live install/uninstall end-to-end OS automation in CI
- Shipping additional runtime services beyond the `claw` CLI and shell completion assets

## Current State

The Rust workspace currently supports local builds and tests, but it does not include a release pipeline or native installer definitions. The documented build path is `cargo build --release`, and the generated executable is the `claw` binary from the `rusty-claude-cli` crate.

## User Requirements

- Native installers, not just archives or bootstrap scripts
- System-wide installation
- `claw` available on `PATH` after installation
- CI-driven release publishing for tagged releases
- Shell completions installed automatically
- Unsigned installers are acceptable for v1

## Recommended Approach

Use a hybrid packaging pipeline:

- Use `cargo-dist` for release orchestration, archive/checksum generation, and GitHub release publishing
- Use explicit native packaging steps for:
  - macOS `.pkg`
  - Windows `.msi`

This avoids overbuilding a custom release system while keeping control over native installation paths, system `PATH` behavior, and shell completion placement.

## Release Artifacts

Each tagged release should publish:

- macOS unsigned `.pkg` for `x86_64-apple-darwin`
- macOS unsigned `.pkg` for `aarch64-apple-darwin`
- Windows unsigned `.msi` for `x86_64-pc-windows-msvc`
- Raw platform archives and checksums for debugging and manual fallback

## Installation Layout

### macOS

- Binary destination: `/usr/local/bin/claw`
- Bash completion: `/usr/local/etc/bash_completion.d/claw`
- Zsh completion: `/usr/local/share/zsh/site-functions/_claw`

The package should require elevation and install into standard system-wide CLI locations.

### Windows

- Primary install root: `C:\\Program Files\\Claw`
- Binary directory on `PATH`: `C:\\Program Files\\Claw\\bin`
- Executable destination: `C:\\Program Files\\Claw\\bin\\claw.exe`
- PowerShell completion file: `C:\\Program Files\\Claw\\completions\\claw.ps1`

The MSI should add the binary directory to the machine-wide `PATH`. This is preferred over a shim-based approach because it is simpler, more transparent, and aligns with standard MSI behavior for CLI tools.

## Completion Strategy

The current codebase supports interactive REPL completion but does not expose a CLI command that emits installable shell completion scripts. A new non-interactive command should be added:

```text
claw completions <shell>
```

Supported initial shells:

- `bash`
- `zsh`
- `powershell`

The release pipeline should invoke this command after building the binary and stage the resulting completion files into the installer payload.

## Packaging Pipeline

### Shared staging model

Each platform build should produce a normalized staging directory containing:

- the built binary
- generated completion files
- top-level docs such as `README.md` and license material

This staging directory becomes the source of truth for native packaging, which keeps macOS and Windows installers aligned and reduces duplication.

### macOS packaging

Use native macOS packaging tools:

- `pkgbuild`
- `productbuild`

The workflow should:

1. build the target binary
2. generate completions
3. assemble the staging tree into the target filesystem layout
4. create an unsigned `.pkg`

The packaging implementation should be structured so code-signing hooks can be added later without changing the payload layout.

### Windows packaging

Use WiX to build an MSI from the staged payload. The MSI should:

- install `claw.exe` into the machine-wide binary directory
- install the PowerShell completion asset
- mutate system `PATH` to include the install bin directory

The WiX source should be generated or templated from stable package metadata so that future version bumps do not require manual editing of multiple installer files.

## CI and Release Flow

Tagged releases should trigger a GitHub Actions workflow that:

1. builds release binaries for the supported targets
2. runs the Rust workspace test suite before packaging
3. generates completion files from the built CLI
4. stages platform payloads
5. builds native installers
6. builds raw archives and checksums
7. publishes all artifacts to the GitHub Release for the tag

The release workflow should keep the native installer steps explicit even if `cargo-dist` handles archive publishing and release metadata.

## Verification Strategy

Verification should be layered:

1. unit tests for the new `claw completions` command
2. packaging smoke tests that validate staged payload contents
3. CI assertions that expected installer artifacts are produced for tagged releases
4. artifact inspection tests:
   - macOS package payload contains the expected destination paths
   - Windows MSI metadata includes the expected install directory and `PATH` mutation

For v1, deterministic artifact inspection is sufficient. Full live installer execution on real operating systems can be added later if release friction justifies it.

## Risks

### Unsigned installer warnings

- macOS Gatekeeper will warn on unsigned `.pkg` installers
- Windows SmartScreen may warn on unsigned `.msi` installers

This is acceptable for v1, but the design should preserve a clean signing insertion point.

### Completion portability

Shell completion installation paths are relatively standard, but shell-specific environment differences can still exist. The initial implementation should target the common system-wide locations above and avoid broader shell-profile mutation.

### Cross-platform packaging complexity

Native installer generation requires platform-native tooling. This increases CI surface area, but it is justified because native installation is an explicit product requirement.

## Implementation Outline

1. Add `claw completions <shell>` command support.
2. Add tests that prove the generated completion output exists and is shell-appropriate.
3. Add a shared release staging script or small packaging helper module.
4. Add macOS `.pkg` packaging assets and workflow steps.
5. Add Windows `.msi` packaging assets and workflow steps.
6. Add tagged-release GitHub Actions workflow.
7. Document local packaging and release operation in the Rust README or release docs.

## Decision Summary

Chosen:

- Hybrid packaging with `cargo-dist` plus explicit native installer steps
- System-wide installation on both platforms
- Direct machine `PATH` mutation on Windows through MSI
- Completion generation from the CLI itself
- Tagged-release GitHub Actions automation
- Unsigned installers in v1

Rejected:

- Shell/bootstrap-only installation: does not satisfy native installer requirement
- Fully custom release orchestration: unnecessary maintenance overhead
- Per-user install scope: conflicts with system-wide requirement
