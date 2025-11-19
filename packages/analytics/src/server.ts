import { logger } from "@rust-v1/logger";
import { waitUntil } from "@vercel/functions";
import { PostHog } from "posthog-node";

type Props = {
  userId?: string;
  fullName?: string | null;
};

export const setupAnalytics = async (options?: Props) => {
  const { userId, fullName } = options ?? {};

  // Initialize PostHog client
  const posthogClient =
    process.env.NEXT_PUBLIC_POSTHOG_KEY && process.env.NEXT_PUBLIC_POSTHOG_HOST
      ? new PostHog(process.env.NEXT_PUBLIC_POSTHOG_KEY, {
          host: process.env.NEXT_PUBLIC_POSTHOG_HOST,
        })
      : null;

  if (userId && fullName && posthogClient) {
    const [firstName, lastName] = fullName.split(" ");

    // Identify user in PostHog
    posthogClient.identify({
      distinctId: userId,
      properties: {
        firstName,
        lastName,
        fullName,
      },
    });
  }

  return {
    track: (options: { event: string; [key: string]: unknown }) => {
      if (process.env.NODE_ENV !== "production") {
        logger.info("Track", options);
        return;
      }

      const { event, ...rest } = options;

      // Track with PostHog
      if (posthogClient) {
        posthogClient.capture({
          distinctId: userId || "anonymous",
          event,
          properties: rest,
        });
      }
    },
    shutdown: async () => {
      // Flush PostHog events before shutdown
      if (posthogClient) {
        await posthogClient.shutdown();
      }
    },
  };
};
