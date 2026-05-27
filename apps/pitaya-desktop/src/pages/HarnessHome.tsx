import { useQuery } from "@tanstack/react-query";
import { motion } from "motion/react";

import { engineStatusQueryKey } from "../lib/query-client";
import { fetchEngineStatus } from "../lib/tauri";

export function HarnessHome() {
  const status = useQuery({
    queryKey: engineStatusQueryKey,
    queryFn: fetchEngineStatus,
  });

  return (
    <motion.main
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      className="mx-auto max-w-lg rounded-lg border border-outline-variant/40 bg-surface-container/80 p-8 backdrop-blur-xl"
    >
      <h1 className="font-display text-3xl text-primary">Pitaya harness</h1>
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
    </motion.main>
  );
}
