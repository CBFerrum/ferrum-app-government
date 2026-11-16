// Government's own interface at /government/: the officers' queues, with every record in the
// records frame's lists and forms. The same release draws its page of the workspace's portal, "Your
// services" at /portal/services/ (Ferrum docs/plan/apps-ui.md §9.1).

import { useEffect } from "react";
import { Link, Outlet, useRouteError } from "react-router";
import { bootstrap, isolate, shell, t } from "@ferrum/client";
import { EmptyState, ErrorState } from "@ferrum/kit";
import { mountApp, RequireAuth, setFrameMode } from "@ferrum/records/frame";
import "./styles.css";

const page = (name: "FormView" | "ListView" | "ReportView") => () => import("./records").then((m) => ({ Component: m[name] }));

function RouteError() {
  return <ErrorState error={useRouteError()} onRetry={() => window.location.reload()} />;
}

function NotFound({ back = t("Go to the queues") }: { back?: string }) {
  return (
    <EmptyState title={t("Page not found")}>
      <Link to="/">{isolate(back)}</Link>
    </EmptyState>
  );
}

function Started() {
  useEffect(() => shell().ready(), []);
  return null;
}

/** The portal's page, when the platform serves this release there. */
const PORTAL = "portal/services";
const onPortal = bootstrap()?.app?.entry === PORTAL;

if (!onPortal) setFrameMode("app", ["government"]);

const portalRoutes = [
  {
    path: "/",
    element: (
      <main className="portal-page">
        <Started />
        <Outlet />
      </main>
    ),
    errorElement: <RouteError />,
    children: [
      { index: true, lazy: () => import("./Portal").then((m) => ({ Component: m.PortalServices })) },
      { path: "*", get element() { return <NotFound back={t("Go to your services")} />; } },
    ],
  },
];

const staffRoutes = [
  {
    path: "/",
    element: (
      <>
        <Started />
        <RequireAuth />
      </>
    ),
    errorElement: <RouteError />,
    children: [
      { index: true, lazy: () => import("./Home").then((m) => ({ Component: m.Home })) },
      { path: "e/:entity", lazy: page("ListView") },
      { path: "e/:entity/:id", lazy: page("FormView") },
      { path: "r/:name", lazy: page("ReportView") },
      { path: "*", element: <NotFound /> },
    ],
  },
];

mountApp({
  catalogs: { ar: () => import("./locales/ar.json"), hi: () => import("./locales/hi.json") },
  ...(onPortal ? { route: PORTAL, routes: portalRoutes } : { route: "government", routes: staffRoutes }),
});
