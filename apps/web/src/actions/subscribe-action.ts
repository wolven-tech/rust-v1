"use server";

import { treaty } from "@elysiajs/eden";
import type app from "../../../api/src/index";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4400";
const client = treaty<typeof app>(API_URL);

export async function subscribeAction(formData: FormData, userGroup: string) {
  const email = formData.get("email") as string;

  const { data, error } = await client.api.subscribe.post({
    email,
    userGroup,
  });

  if (error) {
    return {
      success: false,
      error: error.message || "Failed to subscribe",
    };
  }

  return data;
}
