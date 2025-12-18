// Re-export React Query primitives for direct use
export {
  useQuery,
  useMutation,
  useQueryClient,
  type UseQueryOptions,
  type UseMutationOptions,
} from "@tanstack/react-query";

// Export hooks
export { useServerQuery } from "./hooks/use-server-query";
export { useServerMutation } from "./hooks/use-server-mutation";
export { useInfiniteServerQuery } from "./hooks/use-infinite-server-query";

// Export client
export { queryClient } from "./client";

// Export provider
export { ReactQueryProvider } from "./providers/query-provider";
