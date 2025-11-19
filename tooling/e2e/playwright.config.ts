import { defineConfig, devices } from "@playwright/test";

const API_URL = process.env.API_URL || "http://localhost:3002";
const WEB_URL = process.env.WEB_URL || "http://localhost:3001";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: process.env.CI ? "github" : "html",
  use: {
    baseURL: WEB_URL,
    trace: "on-first-retry",
    screenshot: "only-on-failure",
    video: "retain-on-failure",
  },

  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],

  // Optional: Auto-start servers (comment out if servers are already running)
  // webServer: [
  //   {
  //     command: "cd ../../apps/api && bun run dev",
  //     url: API_URL,
  //     reuseExistingServer: !process.env.CI,
  //     timeout: 120 * 1000,
  //   },
  //   {
  //     command: "cd ../../apps/web && bun run dev",
  //     url: WEB_URL,
  //     reuseExistingServer: !process.env.CI,
  //     timeout: 120 * 1000,
  //   },
  // ],
});
