import "./src/env.mjs";
import { withSentryConfig } from "@sentry/nextjs";

/** @type {import('next').NextConfig} */
const nextConfig = {
  // instrumentation.ts is stable since Next 15; the experimental
  // `instrumentationHook` flag was removed.
  transpilePackages: ["@rust-v1/supabase"],
};

export default withSentryConfig(nextConfig, {
  silent: !process.env.CI,
  telemetry: false,
  widenClientFileUpload: true,
  hideSourceMaps: true,
  webpack: {
    treeshake: {
      removeDebugLogging: true,
    },
  },
  tunnelRoute: "/monitoring",
});
