import type { QueryClient } from "@tanstack/react-query";
import {
  createRootRouteWithContext,
  createRoute,
  createRouter,
  Outlet,
} from "@tanstack/react-router";

import { engineStatusQueryKey, queryClient } from "./lib/query-client";
import { fetchEngineStatus } from "./lib/tauri";
import { HarnessHome } from "./pages/HarnessHome";

export interface RouterContext {
  queryClient: QueryClient;
}

const rootRoute = createRootRouteWithContext<RouterContext>()({
  component: () => (
    <div className="min-h-screen p-8">
      <Outlet />
    </div>
  ),
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  loader: ({ context }) =>
    context.queryClient.ensureQueryData({
      queryKey: engineStatusQueryKey,
      queryFn: fetchEngineStatus,
    }),
  component: HarnessHome,
});

const routeTree = rootRoute.addChildren([indexRoute]);

export const router = createRouter({
  routeTree,
  context: { queryClient },
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
