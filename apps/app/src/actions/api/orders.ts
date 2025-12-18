"use server";

import { actionClient } from "@/actions/safe-action";
import { apiClient } from "@/infrastructure/api";
import { z } from "zod";

const createOrderSchema = z.object({
  product: z.string().min(1, "Product name is required"),
  quantity: z.number().int().positive("Quantity must be a positive integer"),
});

export const createOrderAction = actionClient
  .schema(createOrderSchema)
  .action(async ({ parsedInput }) => {
    const result = await apiClient.createOrder(parsedInput);
    return result;
  });
