"use server";

import { apiClient } from "@/infrastructure/api";
import { actionClient } from "@/actions/safe-action";

export const getMetricsAction = actionClient.action(async () => {
  const result = await apiClient.getMetrics();
  return result;
});
