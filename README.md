# Rewriting Project Claw Code

<p align="center">
  <strong>⭐ The fastest repo in history to surpass 50K stars, reaching the milestone in just 2 hours after publication ⭐</strong>
</p>

<p align="center">
  <a href="https://star-history.com/#instructkr/claw-code&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=instructkr/claw-code&type=Date&theme=dark" />
      <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=instructkr/claw-code&type=Date" />
      <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=instructkr/claw-code&type=Date" width="600" />
    </picture>
  </a>
</p>

<p align="center">
  <img src="assets/clawd-hero.jpeg" alt="Claw" width="300" />
</p>

<p align="center">
  <strong>Better Harness Tools, not merely storing the archive of leaked Claude Code</strong>
</p>

<p align="center">
  <a href="https://github.com/sponsors/instructkr"><img src="https://img.shields.io/badge/Sponsor-%E2%9D%A4-pink?logo=github&style=for-the-badge" alt="Sponsor on GitHub" /></a>
</p>

> [!IMPORTANT]
> The active Rust workspace now lives in [`rust/`](./rust). Start with [`USAGE.md`](./USAGE.md) for build, auth, CLI, session, and parity-harness workflows, then use [`rust/README.md`](./rust/README.md) for crate-level details.

> If you find this work useful, consider [sponsoring @instructkr on GitHub](https://github.com/sponsors/instructkr) to support continued open-source harness engineering research.

---

## Backstory

At 4 AM on March 31, 2026, the Claude Code source was exposed, and the entire dev community was in a frenzy. What followed wasn't a typical human-led engineering effort—it was the first large-scale autonomous coding operation orchestrated entirely by AI agents.

This repository is maintained by **lobsters and claws**—autonomous AI coding agents created by **Bellman** ([@bellman_ych](https://x.com/bellman_ych)) and **Yeongyu** ([@q_yeon_gyu_kim](https://x.com/q_yeon_gyu_kim)). The entire Python port, from reading the original harness structure to producing a working tree with tests, was driven through [oh-my-codex (OmX)](https://github.com/Yeachan-Heo/oh-my-codex) using `$team` mode for parallel code review and `$ralph` mode for persistent execution loops with architect-level verification. No human wrote the core implementation—every line was generated, reviewed, and refined by autonomous agents.

The result is a clean-room Python rewrite that captures the architectural patterns of Claude Code's agent harness without copying any proprietary source. This isn't merely a port; it's a demonstration of what autonomous coding harnesses can achieve when directed by clear architectural vision and rigorous agent orchestration.

We are actively pushing this further. The basic Python foundation is already in place and functional, but we're just getting started. **Stay tuned—a much more capable version is on the way.**

https://github.com/ultraworkers/claw-code

![Tweet screenshot](assets/tweet-screenshot.png)

## The Creators

**claw-code** is autonomously built and maintained by **UltraWorkers** — a collective of AI coding agents (lobsters and claws) orchestrated by:

- **Bellman** ([@bellman_ych](https://x.com/bellman_ych)) — Creator of [oh-my-codex (OmX)](https://github.com/Yeachan-Heo/oh-my-codex), the agent orchestration layer that drives all development
- **Yeongyu** ([@q_yeon_gyu_kim](https://x.com/q_yeon_gyu_kim)) — Architect of the claw-code harness and parallel agent workflows

The entire codebase—from initial port to ongoing feature development—is generated through autonomous agent sessions using OmX `$team` and `$ralph` modes. No human writes the core implementation; humans provide architectural direction, agents execute.

### Featured Coverage

Sigrid Jin has been deeply involved in harness engineering—studying how agent systems wire tools, orchestrate tasks, and manage runtime context. Coverage from *The Wall Street Journal* (March 21, 2026) documents the rise of autonomous coding:

> AI startup worker Sigrid Jin, who attended the Seoul dinner, single-handedly used 25 billion of Claude Code tokens last year. At the time, usage limits were looser, allowing early enthusiasts to reach tens of billions of tokens at a very low cost.
>
> "It was basically like a sharing party," Jin said. "There were lawyers, there were doctors, there were dentists. They did not have software engineering backgrounds."
>
> — *The Wall Street Journal*, March 21, 2026, [*"The Trillion Dollar Race to Automate Our Entire Lives"*](https://lnkd.in/gs9td3qd)

![WSJ Feature](assets/wsj-feature.png)

---

## Porting Status

The main source tree is now Python-first.

- `src/` contains the active Python porting workspace
- `tests/` verifies the current Python workspace
- the exposed snapshot is no longer part of the tracked repository state

The current Python workspace is not yet a complete one-to-one replacement for the original system, but the primary implementation surface is now Python.

## Why this rewrite exists

I originally studied the exposed codebase to understand its harness, tool wiring, and agent workflow. After spending more time with the legal and ethical questions—and after reading the essay linked below—I did not want the exposed snapshot itself to remain the main tracked source tree.

This repository now focuses on Python porting work instead.

## Repository Layout

```text
.
├── src/                                # Python porting workspace
│   ├── __init__.py
│   ├── commands.py
│   ├── main.py
│   ├── models.py
│   ├── port_manifest.py
│   ├── query_engine.py
│   ├── task.py
│   └── tools.py
├── tests/                              # Python verification
├── assets/omx/                         # OmX workflow screenshots
├── 2026-03-09-is-legal-the-same-as-legitimate-ai-reimplementation-and-the-erosion-of-copyleft.md
└── README.md
```

## Python Workspace Overview

The new Python `src/` tree currently provides:

- **`port_manifest.py`** — summarizes the current Python workspace structure
- **`models.py`** — dataclasses for subsystems, modules, and backlog state
- **`commands.py`** — Python-side command port metadata
- **`tools.py`** — Python-side tool port metadata
- **`query_engine.py`** — renders a Python porting summary from the active workspace
- **`main.py`** — a CLI entrypoint for manifest and summary output

## Quickstart

Render the Python porting summary:

```bash
python3 -m src.main summary
```

Print the current Python workspace manifest:

```bash
python3 -m src.main manifest
```

List the current Python modules:

```bash
python3 -m src.main subsystems --limit 16
```

Run verification:

```bash
python3 -m unittest discover -s tests -v
```

Run the parity audit against the local ignored archive (when present):

```bash
python3 -m src.main parity-audit
```

Inspect mirrored command/tool inventories:

```bash
python3 -m src.main commands --limit 10
python3 -m src.main tools --limit 10
```

## Current Parity Checkpoint

The port now mirrors the archived root-entry file surface, top-level subsystem names, and command/tool inventories much more closely than before. However, it is **not yet** a full runtime-equivalent replacement for the original TypeScript system; the Python tree still contains fewer executable runtime slices than the archived source.


## Built with `oh-my-codex`

The restructuring and documentation work on this repository was AI-assisted and orchestrated with Yeachan Heo's [oh-my-codex (OmX)](https://github.com/Yeachan-Heo/oh-my-codex), layered on top of Codex.

- **`$team` mode:** used for coordinated parallel review and architectural feedback
- **`$ralph` mode:** used for persistent execution, verification, and completion discipline
- **Codex-driven workflow:** used to turn the main `src/` tree into a Python-first porting workspace

### OmX workflow screenshots

![OmX workflow screenshot 1](assets/omx/omx-readme-review-1.png)

*Ralph/team orchestration view while the README and essay context were being reviewed in terminal panes.*

![OmX workflow screenshot 2](assets/omx/omx-readme-review-2.png)

*Split-pane review and verification flow during the final README wording pass.*

## Community

<p align="center">
  <a href="https://discord.gg/6ztZB9jvWq"><img src="https://img.shields.io/badge/UltraWorkers-Discord-5865F2?logo=discord&style=for-the-badge" alt="UltraWorkers Discord" /></a>
</p>

Join the [**UltraWorkers Discord**](https://discord.gg/6ztZB9jvWq) — the community around clawhip, oh-my-openagent, oh-my-claudecode, oh-my-codex, and claw-code. Come chat about LLMs, harness engineering, agent workflows, and autonomous software development.

[![Discord](https://img.shields.io/badge/Join%20Discord-UltraWorkers-5865F2?logo=discord&style=for-the-badge)](https://discord.gg/6ztZB9jvWq)

## Star History

See the chart at the top of this README.

## Ownership / Affiliation Disclaimer

- This repository does **not** claim ownership of the original Claude Code source material.
- This repository is **not affiliated with, endorsed by, or maintained by Anthropic**.
