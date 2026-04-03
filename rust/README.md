# 🦞 Claw Code — Rust Implementation

A high-performance Rust rewrite of the Claw Code CLI agent harness. Built for speed, safety, and native tool execution.

## Quick Start

```bash
# Build
cd rust/
cargo build --release

# Run interactive REPL
./target/release/claw

# One-shot prompt
./target/release/claw prompt "explain this codebase"

# With specific model
./target/release/claw --model sonnet prompt "fix the bug in main.rs"

# Emit installable shell completions
./target/release/claw completions bash
./target/release/claw completions zsh
./target/release/claw completions powershell
```

## Native Installer Packaging

Tagged GitHub releases are intended to publish native installers plus raw fallback archives:

- macOS unsigned `.pkg` for `x86_64-apple-darwin`
- macOS unsigned `.pkg` for `aarch64-apple-darwin`
- Windows unsigned `.msi` for `x86_64-pc-windows-msvc`

Unsigned installer caveats:

- macOS Gatekeeper will warn before opening the `.pkg`
- Windows SmartScreen may warn before launching the `.msi`

### Local macOS packaging

```bash
cd rust/

cargo build --release -p rusty-claude-cli --bin claw

python3 scripts/stage_release_payload.py \
  --binary target/release/claw \
  --target x86_64-apple-darwin \
  --output dist/local-macos/payload

python3 scripts/verify_release_payload.py \
  --payload dist/local-macos/payload \
  --target x86_64-apple-darwin

sh packaging/macos/build-pkg.sh \
  --payload dist/local-macos/payload \
  --identifier dev.instructkr.claw \
  --version 0.1.0 \
  --output dist/local-macos/claw.pkg

sh packaging/macos/inspect-pkg.sh dist/local-macos/claw.pkg
```

### Local Windows packaging

```powershell
cd rust

cargo build --release -p rusty-claude-cli --bin claw --target x86_64-pc-windows-msvc

python scripts/stage_release_payload.py `
  --binary target/x86_64-pc-windows-msvc/release/claw.exe `
  --target x86_64-pc-windows-msvc `
  --output dist/x86_64-pc-windows-msvc/payload

python scripts/verify_release_payload.py `
  --payload dist/x86_64-pc-windows-msvc/payload `
  --target x86_64-pc-windows-msvc

./packaging/windows/build-msi.ps1 `
  -PayloadDir dist/x86_64-pc-windows-msvc/payload `
  -Version 0.1.0 `
  -OutFile dist/x86_64-pc-windows-msvc/claw.msi

./packaging/windows/inspect-msi.ps1 `
  -MsiPath dist/x86_64-pc-windows-msvc/claw.msi
```

## Configuration

Set your API credentials:

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
# Or use a proxy
export ANTHROPIC_BASE_URL="https://your-proxy.com"
```

Or authenticate via OAuth:

```bash
claw login
```

## Mock parity harness

The workspace now includes a deterministic Anthropic-compatible mock service and a clean-environment CLI harness for end-to-end parity checks.

```bash
cd rust/

# Run the scripted clean-environment harness
./scripts/run_mock_parity_harness.sh

# Or start the mock service manually for ad hoc CLI runs
cargo run -p mock-anthropic-service -- --bind 127.0.0.1:0
```

Harness coverage:

- `streaming_text`
- `read_file_roundtrip`
- `grep_chunk_assembly`
- `write_file_allowed`
- `write_file_denied`
- `multi_tool_turn_roundtrip`
- `bash_stdout_roundtrip`
- `bash_permission_prompt_approved`
- `bash_permission_prompt_denied`
- `plugin_tool_roundtrip`

Primary artifacts:

- `crates/mock-anthropic-service/` — reusable mock Anthropic-compatible service
- `crates/rusty-claude-cli/tests/mock_parity_harness.rs` — clean-env CLI harness
- `scripts/run_mock_parity_harness.sh` — reproducible wrapper
- `scripts/run_mock_parity_diff.py` — scenario checklist + PARITY mapping runner
- `mock_parity_scenarios.json` — scenario-to-PARITY manifest

## Features

| Feature | Status |
|---------|--------|
| Anthropic API + streaming | ✅ |
| OAuth login/logout | ✅ |
| Interactive REPL (rustyline) | ✅ |
| Tool system (bash, read, write, edit, grep, glob) | ✅ |
| Web tools (search, fetch) | ✅ |
| Sub-agent orchestration | ✅ |
| Todo tracking | ✅ |
| Notebook editing | ✅ |
| CLAUDE.md / project memory | ✅ |
| Config file hierarchy (.claude.json) | ✅ |
| Permission system | ✅ |
| MCP server lifecycle | ✅ |
| Session persistence + resume | ✅ |
| Extended thinking (thinking blocks) | ✅ |
| Cost tracking + usage display | ✅ |
| Git integration | ✅ |
| Markdown terminal rendering (ANSI) | ✅ |
| Model aliases (opus/sonnet/haiku) | ✅ |
| Slash commands (/status, /compact, /clear, etc.) | ✅ |
| Hooks (PreToolUse/PostToolUse) | 🔧 Config only |
| Plugin system | 📋 Planned |
| Skills registry | 📋 Planned |

## Model Aliases

Short names resolve to the latest model versions:

| Alias | Resolves To |
|-------|------------|
| `opus` | `claude-opus-4-6` |
| `sonnet` | `claude-sonnet-4-6` |
| `haiku` | `claude-haiku-4-5-20251213` |

## CLI Flags

```
claw [OPTIONS] [COMMAND]

Options:
  --model MODEL                    Set the model (alias or full name)
  --dangerously-skip-permissions   Skip all permission checks
  --permission-mode MODE           Set read-only, workspace-write, or danger-full-access
  --allowedTools TOOLS             Restrict enabled tools
  --output-format FORMAT           Output format (text or json)
  --version, -V                    Print version info

Commands:
  prompt <text>      One-shot prompt (non-interactive)
  login              Authenticate via OAuth
  logout             Clear stored credentials
  init               Initialize project config
  doctor             Check environment health
  self-update        Update to latest version
```

## Slash Commands (REPL)

Tab completion now expands not just slash command names, but also common workflow arguments like model aliases, permission modes, and recent session IDs.

| Command | Description |
|---------|-------------|
| `/help` | Show help |
| `/status` | Show session status (model, tokens, cost) |
| `/cost` | Show cost breakdown |
| `/compact` | Compact conversation history |
| `/clear` | Clear conversation |
| `/model [name]` | Show or switch model |
| `/permissions` | Show or switch permission mode |
| `/config [section]` | Show config (env, hooks, model) |
| `/memory` | Show CLAUDE.md contents |
| `/diff` | Show git diff |
| `/export [path]` | Export conversation |
| `/session [id]` | Resume a previous session |
| `/version` | Show version |

## Workspace Layout

```
rust/
├── Cargo.toml              # Workspace root
├── Cargo.lock
└── crates/
    ├── api/                # Anthropic API client + SSE streaming
    ├── commands/           # Shared slash-command registry
    ├── compat-harness/     # TS manifest extraction harness
    ├── mock-anthropic-service/ # Deterministic local Anthropic-compatible mock
    ├── runtime/            # Session, config, permissions, MCP, prompts
    ├── rusty-claude-cli/   # Main CLI binary (`claw`)
    └── tools/              # Built-in tool implementations
```

### Crate Responsibilities

- **api** — HTTP client, SSE stream parser, request/response types, auth (API key + OAuth bearer)
- **commands** — Slash command definitions and help text generation
- **compat-harness** — Extracts tool/prompt manifests from upstream TS source
- **mock-anthropic-service** — Deterministic `/v1/messages` mock for CLI parity tests and local harness runs
- **runtime** — `ConversationRuntime` agentic loop, `ConfigLoader` hierarchy, `Session` persistence, permission policy, MCP client, system prompt assembly, usage tracking
- **rusty-claude-cli** — REPL, one-shot prompt, streaming display, tool call rendering, CLI argument parsing
- **tools** — Tool specs + execution: Bash, ReadFile, WriteFile, EditFile, GlobSearch, GrepSearch, WebSearch, WebFetch, Agent, TodoWrite, NotebookEdit, Skill, ToolSearch, REPL runtimes

## Stats

- **~20K lines** of Rust
- **7 crates** in workspace
- **Binary name:** `claw`
- **Default model:** `claude-opus-4-6`
- **Default permissions:** `danger-full-access`

## License

See repository root.
