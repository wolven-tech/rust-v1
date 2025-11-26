import { logger } from "@rust-v1/logger";
import { createClient } from "@rust-v1/supabase/server";
import type { Tables } from "../types";

export async function getUser() {
  const supabase = await createClient();

  try {
    const result = await supabase.auth.getUser();

    return result;
  } catch (error) {
    logger.error(error);

    throw error;
  }
}

export async function getPosts() {
  const supabase = await createClient();

  try {
    // biome-ignore lint/suspicious/noExplicitAny: Supabase SSR type inference issue with generic Database type
    const result = await (supabase.from("posts") as any).select("*");

    return result as { data: Tables<"posts">[] | null; error: Error | null };
  } catch (error) {
    logger.error(error);
    throw error;
  }
}
