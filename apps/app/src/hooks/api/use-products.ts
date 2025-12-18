"use client";

import { useMutation, useQueryClient } from "@rust-v1/react-query";
import { searchProductsAction } from "@/actions/api";
import type { SearchProductsResponse } from "@/domain/api";
import { METRICS_QUERY_KEY } from "./use-metrics";

interface SearchProductsInput {
  query: string;
}

export function useSearchProducts() {
  const queryClient = useQueryClient();

  return useMutation<SearchProductsResponse, Error, SearchProductsInput>({
    mutationFn: async ({ query }: SearchProductsInput) => {
      const result = await searchProductsAction({ query });

      if (result?.data) {
        return result.data;
      }

      throw new Error(
        result?.serverError ||
          result?.validationErrors?.query?._errors?.[0] ||
          "Failed to search products",
      );
    },
    onSuccess: () => {
      // Invalidate metrics to reflect new API call count
      queryClient.invalidateQueries({ queryKey: [METRICS_QUERY_KEY] });
    },
  });
}
