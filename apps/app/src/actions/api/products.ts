"use server";

import { apiClient } from "@/infrastructure/api";
import { actionClient } from "@/actions/safe-action";
import { z } from "zod";

const searchProductsSchema = z.object({
  query: z.string().min(1, "Search query is required"),
});

export const searchProductsAction = actionClient
  .schema(searchProductsSchema)
  .action(async ({ parsedInput }) => {
    const result = await apiClient.searchProducts(parsedInput);
    return result;
  });
