"use client";

import { type UseQueryOptions, useQuery } from "@tanstack/react-query";

/**
 * Generic response type for server actions
 */
export interface ServerActionResponse<T> {
  data?: T;
  error?: string;
}

/**
 * Generic hook to fetch data using a server action
 * This provides a consistent pattern for using React Query with Next.js server actions
 *
 * @param queryKey - The key to use for caching
 * @param serverAction - The server action function to call
 * @param options - Additional React Query options
 */
export function useServerQuery<TData, TError = Error>(
  queryKey: string | number | (string | number)[],
  serverAction: () => Promise<ServerActionResponse<TData> | TData>,
  options?: Omit<UseQueryOptions<TData, TError, TData>, "queryKey" | "queryFn">,
) {
  const queryKeyArray = Array.isArray(queryKey) ? queryKey : [queryKey];

  return useQuery<TData, TError>({
    queryKey: queryKeyArray,
    queryFn: async () => {
      // In Next.js, server actions are automatically handled correctly
      // when called from client components
      const result = await serverAction();

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
