"use client";

import {
  type UseMutationOptions,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import type { ServerActionResponse } from "./use-server-query";

/**
 * Generic hook to mutate data using a server action
 * This provides a consistent pattern for using React Query mutations with Next.js server actions
 *
 * @param serverAction - The server action function to call with data
 * @param options - Additional React Query options
 * @param invalidateQueries - Query keys to invalidate on success
 */
export function useServerMutation<TData, TVariables, TError = Error>(
  serverAction: (
    data: TVariables,
  ) => Promise<ServerActionResponse<TData> | TData>,
  options?: Omit<UseMutationOptions<TData, TError, TVariables>, "mutationFn">,
  invalidateQueries?: string[],
) {
  const queryClient = useQueryClient();

  return useMutation<TData, TError, TVariables>({
    mutationFn: async (variables: TVariables) => {
      // In Next.js, server actions are automatically handled correctly
      // when called from client components
      const result = await serverAction(variables);

      // Handle different response formats from server actions
      if (result && typeof result === "object" && "data" in result) {
        const typedResult = result as ServerActionResponse<TData>;
        if (typedResult.data) {
          return typedResult.data;
        }
        throw new Error(
          typedResult.error || "Failed to execute action: No data returned",
        );
      }

      return result as TData;
    },
    onSuccess: () => {
      // Invalidate queries if specified
      if (invalidateQueries) {
        for (const query of invalidateQueries) {
          queryClient.invalidateQueries({ queryKey: [query] });
        }
      }
    },
    ...options,
  });
}
