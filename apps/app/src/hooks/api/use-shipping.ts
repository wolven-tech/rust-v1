"use client";

import { useMutation, useQueryClient } from "@rust-v1/react-query";
import { calculateShippingAction } from "@/actions/api";
import type { ShippingCost } from "@/domain/api";
import { METRICS_QUERY_KEY } from "./use-metrics";

interface CalculateShippingInput {
  weight: number;
}

export function useCalculateShipping() {
  const queryClient = useQueryClient();

  return useMutation<ShippingCost, Error, CalculateShippingInput>({
    mutationFn: async ({ weight }: CalculateShippingInput) => {
      const result = await calculateShippingAction({ weight });

      if (result?.data) {
        return result.data;
      }

      throw new Error(
        result?.serverError ||
          result?.validationErrors?.weight?._errors?.[0] ||
          "Failed to calculate shipping",
      );
    },
    onSuccess: () => {
      // Invalidate metrics to reflect new API call count
      queryClient.invalidateQueries({ queryKey: [METRICS_QUERY_KEY] });
    },
  });
}
