import { commands, type EngineStatusDto, type PitayaError } from "../bindings";

export type { EngineState, EngineStatusDto, PitayaError } from "../bindings";

export { commands };

function unwrapCommandResult<T>(result: { status: "ok"; data: T } | { status: "error"; error: PitayaError }): T {
  if (result.status === "error") {
    const message =
      result.error.code === "Internal"
        ? "internal error"
        : `${result.error.code}: ${"message" in result.error ? result.error.message : ""}`;
    throw new Error(message);
  }
  return result.data;
}

export async function fetchEngineStatus(): Promise<EngineStatusDto> {
  return unwrapCommandResult(await commands.getEngineStatus());
}
