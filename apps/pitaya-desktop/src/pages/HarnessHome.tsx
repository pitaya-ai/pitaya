import { useQuery } from "@tanstack/react-query";
import { motion } from "motion/react";

import { commands } from "../lib/tauri";

export function HarnessHome() {
  const status = useQuery({
    queryKey: ["engine-status"],
    queryFn: () => commands.getEngineStatus(),
  });

  return (
    <motion.main
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      className="mx-auto max-w-lg rounded-lg border border-white/10 bg-surface-container/80 p-8 backdrop-blur-xl"
    >
      <h1 className="font-display text-3xl text-primary">Pitaya harness</h1>
      <p className="mt-4 text-on-surface/80">
        Engine stub is running in-process. Daily-driver UI ships in P3.
      </p>
      <dl className="mt-6 space-y-2 text-sm">
        <div className="flex justify-between gap-4">
          <dt className="text-on-surface/60">Engine state</dt>
          <dd className="font-medium">
            {status.isLoading ? "…" : (status.data?.state ?? String(status.error))}
          </dd>
        </div>
        <div className="flex justify-between gap-4">
          <dt className="text-on-surface/60">Version</dt>
          <dd className="font-medium">{status.data?.version ?? "—"}</dd>
        </div>
      </dl>
    </motion.main>
  );
}
