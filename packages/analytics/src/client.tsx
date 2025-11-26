"use client";

import { logger } from "@rust-v1/logger";
import posthog from "posthog-js";
import { useEffect } from "react";

const isProd = process.env.NODE_ENV === "production";

const Provider = () => {
  useEffect(() => {
    if (
      isProd &&
      process.env.NEXT_PUBLIC_POSTHOG_KEY &&
      process.env.NEXT_PUBLIC_POSTHOG_HOST
    ) {
      posthog.init(process.env.NEXT_PUBLIC_POSTHOG_KEY, {
        api_host: process.env.NEXT_PUBLIC_POSTHOG_HOST,
        person_profiles: "identified_only",
        capture_pageview: true,
        capture_pageleave: true,
      });
    }
  }, []);

  return null;
};

const track = (options: { event: string; [key: string]: unknown }) => {
  if (!isProd) {
    logger.info(options, "Track");
    return;
  }

  const { event, ...rest } = options;

  // Track with PostHog
  if (typeof window !== "undefined" && posthog.__loaded) {
    posthog.capture(event, rest);
  }
};

export { Provider, track, posthog };
