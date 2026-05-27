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
pnpm dev
```

Debug `tauri dev` regenerates `apps/pitaya-desktop/src/bindings.ts` via **tauri-specta**.

```bash
cargo build -p pitaya-desktop
pnpm --filter pitaya-desktop check
pnpm --filter pitaya-desktop lint
```

### pnpm supply chain

`pnpm-workspace.yaml` sets `minimumReleaseAge: 1440` (24h) and `blockExoticSubdeps: true`. If a new toolchain package is younger than 24h, add a temporary `minimumReleaseAgeExclude` entry.

## License

Pitaya code is dual-licensed under either:

- MIT License
- Apache License, Version 2.0

at your option.

The Pitaya name, logo, icon, and brand assets are not covered by the code license. See `TRADEMARKS.md`.
