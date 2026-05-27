# ADR-010: Dual transport

## Status

Accepted (v1)

## Context

Pitaya serves two clients: the React webview inside `pitaya-desktop`, and external agents via `pitaya-mcp` over a Unix socket.

## Decision

- **UI:** Tauri commands and events only. The webview never opens `pitaya.sock`.
- **MCP / tests:** Length-prefixed JSON on `$XDG_RUNTIME_DIR/pitaya/pitaya.sock` (mode `0600`).
- **Contracts:** `pitaya-core` DTOs are shared; TypeScript types for the UI come from `tauri-specta` at the command boundary.

## Consequences

- Thin Tauri adapters in `apps/pitaya-desktop/src-tauri` delegate to `pitaya-engine`.
- No duplicate IPC paths for the same user action.
