import { logger } from "@rust-v1/logger";
import { createClient } from "@rust-v1/supabase/server";
import type { TablesUpdate } from "../types";

export async function updateUser(userId: string, data: TablesUpdate<"users">) {
  const supabase = await createClient();

  try {
    // biome-ignore lint/suspicious/noExplicitAny: Supabase SSR type inference issue with generic Database type
    const result = await (supabase.from("users") as any)
      .update(data)
      .eq("id", userId);

    return result;
  } catch (error) {
    logger.error(error);

    throw error;
  }
}
