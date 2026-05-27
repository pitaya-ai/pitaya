# Pitaya

Linux-first, open-source meeting assistant.

## Harness (PIT-6)

Monorepo layout:

- `crates/` — Rust domain libraries and `pitaya-mcp` binary
- `apps/pitaya-desktop/` — Tauri 2 + React shell (engine starts in `tauri::setup`)
- `packages/` — shared TypeScript config and Tailwind theme

### Prerequisites

- Rust **1.88+** (`rust-toolchain.toml`)
- Node **22+** (`.node-version`), pnpm **10+**
- Linux deps for Tauri: see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Development

```bash
pnpm install
make dev          # or: pnpm dev
make verify       # fmt, clippy, tests, bindings, tsc, lint, build
make bindings     # regenerate src/bindings.ts from Rust commands
```

`tauri dev` (debug) also regenerates `apps/pitaya-desktop/src/bindings.ts` via **tauri-specta**. CI runs `make bindings` via the Rust test suite to keep bindings in sync.

### Roadmap phases

| Phase | Focus |
|-------|--------|
| **P1** | Engine, SQLite, MCP socket — no tray/UI |
| **P2** | PipeWire + local ASR |
| **P3** | Tauri daily driver (home, tray, transcript) |
| **P4–P5** | Intelligence + MCP tools |

See [ROADMAP.md](ROADMAP.md) and [ARCHITECTURE.md](ARCHITECTURE.md).

### pnpm supply chain

`pnpm-workspace.yaml` sets `minimumReleaseAge: 1440` (24h) and `blockExoticSubdeps: true`. If a new toolchain package is younger than 24h, add a temporary `minimumReleaseAgeExclude` entry.

## License

Pitaya code is dual-licensed under either:

- MIT License
- Apache License, Version 2.0

at your option.

The Pitaya name, logo, icon, and brand assets are not covered by the code license. See `TRADEMARKS.md`.
