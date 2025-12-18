"use client";

import { useMutation, useQueryClient } from "@rust-v1/react-query";
import { createOrderAction } from "@/actions/api";
import type { Order } from "@/domain/api";
import { METRICS_QUERY_KEY } from "./use-metrics";

interface CreateOrderInput {
  product: string;
  quantity: number;
}

export function useCreateOrder() {
  const queryClient = useQueryClient();

  return useMutation<Order, Error, CreateOrderInput>({
    mutationFn: async ({ product, quantity }: CreateOrderInput) => {
      const result = await createOrderAction({ product, quantity });

      if (result?.data) {
        return result.data;
      }

      throw new Error(
        result?.serverError ||
          result?.validationErrors?.product?._errors?.[0] ||
          result?.validationErrors?.quantity?._errors?.[0] ||
          "Failed to create order",
      );
    },
    onSuccess: () => {
      // Invalidate metrics to reflect new order count
      queryClient.invalidateQueries({ queryKey: [METRICS_QUERY_KEY] });
    },
  });
}
