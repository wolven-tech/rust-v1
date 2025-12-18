"use server";

import { actionClient } from "@/actions/safe-action";
import { apiClient } from "@/infrastructure/api";

export const getMetricsAction = actionClient.action(async () => {
  const result = await apiClient.getMetrics();
  return result;
});
