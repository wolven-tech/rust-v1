"use client";

import {
  type UseInfiniteQueryOptions,
  useInfiniteQuery,
} from "@tanstack/react-query";
import type { ServerActionResponse } from "./use-server-query";

/**
 * Generic hook for infinite queries using server actions
 * This provides a consistent pattern for using React Query infinite queries with Next.js server actions
 *
 * @param queryKey - The key to use for caching
 * @param serverAction - The server action function to call with pageParam
 * @param options - Additional React Query options
 */
export function useInfiniteServerQuery<TData, TError = Error>(
  queryKey: string | number | (string | number)[],
  serverAction: (pageParam: number) => Promise<ServerActionResponse<TData> | TData>,
  options?: Omit<
    UseInfiniteQueryOptions<TData, TError, TData>,
    "queryKey" | "queryFn"
  >,
) {
  const queryKeyArray = Array.isArray(queryKey) ? queryKey : [queryKey];

  return useInfiniteQuery<TData, TError>({
    queryKey: queryKeyArray,
    queryFn: async ({ pageParam = 0 }) => {
      const result = await serverAction(pageParam as number);

      // Handle different response formats from server actions
      if (result && typeof result === "object" && "data" in result) {
        const typedResult = result as ServerActionResponse<TData>;
        if (typedResult.data) {
          return typedResult.data;
        }
        throw new Error(
          typedResult.error || "Failed to fetch data: No data returned",
        );
      }

      return result as TData;
    },
    ...options,
  });
}
