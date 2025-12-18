import { defineConfig, devices } from "@playwright/test";

const API_URL = process.env.API_URL || "http://localhost:4400";
const WEB_URL = process.env.WEB_URL || "http://localhost:4401";
const APP_URL = process.env.APP_URL || "http://localhost:4402";

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
});
