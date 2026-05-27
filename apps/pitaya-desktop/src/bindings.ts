// Placeholder until `pnpm tauri dev` exports real bindings (debug build).
// This file is gitignored; committed stub for CI typecheck.

export const commands = {
  getEngineStatus: async (): Promise<{ state: string; version: string }> => ({
    state: "idle",
    version: "0.1.0",
  }),
};
