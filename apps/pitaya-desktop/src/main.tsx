import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import {
  createRootRoute,
  createRoute,
  createRouter,
  Link,
  Outlet,
  RouterProvider,
} from "@tanstack/react-router";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "@pitaya/tailwind/theme.css";
import "./index.css";

import { HarnessHome } from "./pages/HarnessHome";
import { SettingsPlaceholder } from "./pages/SettingsPlaceholder";

const queryClient = new QueryClient();

const rootRoute = createRootRoute({
  component: RootLayout,
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: HarnessHome,
});

const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/settings",
  component: SettingsPlaceholder,
});

const routeTree = rootRoute.addChildren([indexRoute, settingsRoute]);

const router = createRouter({
  routeTree,
  context: { queryClient },
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

function RootLayout() {
  return (
    <div className="min-h-screen p-8">
      <nav className="mb-8 flex gap-4 text-sm">
        <Link to="/" className="text-primary hover:underline">
          Home
        </Link>
        <Link to="/settings" className="text-on-surface/80 hover:underline">
          Settings
        </Link>
      </nav>
      <Outlet />
    </div>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </StrictMode>,
);
