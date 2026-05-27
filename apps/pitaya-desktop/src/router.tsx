import { createRouter } from "@tanstack/react-router";

import { queryClient } from "./lib/query-client";
import { indexRoute } from "./routes/index";
import { rootRoute } from "./routes/root";
import { settingsRoute } from "./routes/settings";

const routeTree = rootRoute.addChildren([indexRoute, settingsRoute]);

export const router = createRouter({
  routeTree,
  context: { queryClient },
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
