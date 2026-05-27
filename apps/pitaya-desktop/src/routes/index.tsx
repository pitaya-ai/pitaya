import { createRoute } from "@tanstack/react-router";

import { engineStatusQueryKey } from "../lib/query-client";
import { fetchEngineStatus } from "../lib/tauri";
import { HarnessHome } from "../pages/HarnessHome";
import { rootRoute } from "./root";

export const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  loader: ({ context }) =>
    context.queryClient.ensureQueryData({
      queryKey: engineStatusQueryKey,
      queryFn: fetchEngineStatus,
    }),
  component: HarnessHome,
});
