import { useQuery } from "@tanstack/react-query";

import { engineStatusQueryKey } from "../lib/query-client";
import { fetchEngineStatus } from "../lib/tauri";

export function HarnessHome() {
  const status = useQuery({
    queryKey: engineStatusQueryKey,
    queryFn: fetchEngineStatus,
  });

  return (
    <main className="mx-auto max-w-lg rounded-lg border border-on-surface/20 bg-surface p-8">
      <h1 className="text-3xl text-primary">Pitaya harness</h1>
      <p className="mt-4 text-on-surface/80">
        Engine stub is running in-process. Full product UI not yet implemented.
      </p>
      <dl className="mt-6 space-y-2 text-sm">
        <div className="flex justify-between gap-4">
          <dt className="text-on-surface/60">Engine state</dt>
          <dd className="font-medium">
            {status.isLoading && "…"}
            {status.isError && (
              <span className="text-error">
                {status.error instanceof Error ? status.error.message : "Failed to load status"}
              </span>
            )}
            {status.isSuccess && status.data.state}
          </dd>
        </div>
        <div className="flex justify-between gap-4">
          <dt className="text-on-surface/60">Version</dt>
          <dd className="font-medium">{status.isSuccess ? status.data.version : "—"}</dd>
        </div>
      </dl>
    </main>
  );
}
