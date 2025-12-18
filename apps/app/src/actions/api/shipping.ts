"use server";

import { apiClient } from "@/infrastructure/api";
import { actionClient } from "@/actions/safe-action";
import { z } from "zod";

const calculateShippingSchema = z.object({
  weight: z.number().positive("Weight must be a positive number"),
});

export const calculateShippingAction = actionClient
  .schema(calculateShippingSchema)
  .action(async ({ parsedInput }) => {
    const result = await apiClient.calculateShipping(parsedInput);
    return result;
  });
