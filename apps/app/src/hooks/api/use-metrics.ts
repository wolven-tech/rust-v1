"use client";

import { getMetricsAction } from "@/actions/api";
import type { Metrics } from "@/domain/api";
import { useQuery } from "@rust-v1/react-query";

export const METRICS_QUERY_KEY = "metrics";

export function useMetrics(options?: { refetchInterval?: number }) {
  return useQuery<Metrics>({
    queryKey: [METRICS_QUERY_KEY],
    queryFn: async () => {
      const result = await getMetricsAction();

      if (result?.data) {
        return result.data;
      }

      throw new Error(result?.serverError || "Failed to fetch metrics");
    },
    refetchInterval: options?.refetchInterval,
  });
}
