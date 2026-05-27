import type { QueryClient } from "@tanstack/react-query";
import { createRootRouteWithContext, Link, Outlet } from "@tanstack/react-router";

export interface RouterContext {
  queryClient: QueryClient;
}

export const rootRoute = createRootRouteWithContext<RouterContext>()({
  component: RootLayout,
});

function RootLayout() {
  return (
    <div className="min-h-screen p-8">
      <nav className="mb-8 flex gap-4 text-sm">
        <Link
          to="/"
          className="text-primary hover:underline"
          activeProps={{ className: "underline" }}
        >
          Home
        </Link>
        <Link
          to="/settings"
          className="text-on-surface/80 hover:underline"
          activeProps={{ className: "text-primary underline" }}
        >
          Settings
        </Link>
      </nav>
      <Outlet />
    </div>
  );
}
