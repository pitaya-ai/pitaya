import { createRoute } from "@tanstack/react-router";

import { SettingsPlaceholder } from "../pages/SettingsPlaceholder";
import { rootRoute } from "./root";

export const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/settings",
  component: SettingsPlaceholder,
});
